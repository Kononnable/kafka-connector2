use jni::{
    objects::{JObject, JValue},
    sys::{jdouble, jlong, jobject},
    JNIEnv,
};

use crate::{
    clone_from_java::CloneFromJava, clone_to_java::CloneToJava,
    common::metrics::metric_config::MetricConfig, java_stored_object::FromJObject,
};

use super::sample::Sample;

#[derive(Debug, Clone)]
pub struct SampledStat {
    initial_value: f64,
    current: usize,
    samples: Vec<Sample>,
    stat_type: StatType,
}
#[derive(Debug, Clone, Copy)]
pub enum StatType {
    Avg,
    Min,
    Max,
    WindowedSum,
    WindowedCount,
}
impl SampledStat {
    pub fn new(stat_type: StatType) -> SampledStat {
        let initial_value = match stat_type {
            StatType::Avg => 0_f64,
            StatType::Min => f64::MAX,
            StatType::Max => f64::MIN,
            StatType::WindowedSum | StatType::WindowedCount => 0_f64,
        };
        SampledStat {
            initial_value,
            current: 0,
            samples: vec![],
            stat_type,
        }
    }
    pub fn record(&mut self, config: &MetricConfig, value: f64, time_ms: u128) {
        let stat_type = self.stat_type;
        let mut sample = self.current(time_ms);
        if sample.is_complete(time_ms, config) {
            self.current = (self.current + 1) % config.samples as usize;
            if self.current as usize >= self.samples.len() {
                let new_sample = Sample::new(self.initial_value, time_ms);
                self.samples.push(new_sample);
                sample = self.samples.last_mut().unwrap();
            } else {
                sample = self.current(time_ms);
                sample.reset(time_ms);
            };
        }
        SampledStat::update(sample, value, stat_type);
        sample.event_count += 1;
    }

    pub fn measure(&mut self, config: &MetricConfig, now: u128) -> f64 {
        self.purge_obsolete_samples(config, now);
        self.combine(&self.samples)
    }
    pub fn current(&mut self, time_ms: u128) -> &mut Sample {
        if self.samples.is_empty() {
            self.samples.push(Sample::new(self.initial_value, time_ms));
        }
        self.samples.get_mut(self.current).unwrap()
    }
    pub fn oldest(&mut self, now: u128) -> &Sample {
        if self.samples.is_empty() {
            self.samples.push(Sample::new(self.initial_value, now));
        }
        self.samples
            .iter()
            .min_by_key(|x| x.last_window_ms)
            .unwrap()
    }

    fn update(sample: &mut Sample, value: f64, stat_type: StatType) {
        match stat_type {
            StatType::Avg | StatType::WindowedSum => sample.value += value,
            StatType::Min => sample.value = f64::min(sample.value, value),
            StatType::Max => sample.value = f64::max(sample.value, value),
            StatType::WindowedCount => sample.value += 1_f64,
        }
    }
    pub fn combine(&self, samples: &[Sample]) -> f64 {
        match self.stat_type {
            StatType::Avg => {
                let (total, count) = samples
                    .iter()
                    .fold((0_f64, 0_u64), |(total, count), sample| {
                        (total + sample.value, count + sample.event_count)
                    });
                if count == 0 {
                    f64::NAN
                } else {
                    total / count as f64
                }
            }
            StatType::Min => {
                let (min, count) =
                    samples
                        .iter()
                        .fold((f64::MAX, 0_u64), |(total, count), sample| {
                            (f64::min(total, sample.value), count + sample.event_count)
                        });
                if count == 0 {
                    f64::NAN
                } else {
                    min
                }
            }
            StatType::Max => {
                let (max, count) =
                    samples
                        .iter()
                        .fold((f64::MIN, 0_u64), |(total, count), sample| {
                            (f64::max(total, sample.value), count + sample.event_count)
                        });
                if count == 0 {
                    f64::NAN
                } else {
                    max
                }
            }
            StatType::WindowedSum | StatType::WindowedCount => samples
                .iter()
                .fold(0_f64, |total, sample| total + sample.value),
        }
    }
    pub(super) fn purge_obsolete_samples(&mut self, config: &MetricConfig, now: u128) {
        let (max_window_ms, overflow) =
            u128::overflowing_sub(now, config.samples as u128 * config.time_window_ms);
        if !overflow {
            self.samples
                .iter_mut()
                .filter(|s| s.last_window_ms <= max_window_ms)
                .for_each(|s| s.reset(now));
        }
    }
}

impl CloneToJava for SampledStat {
    fn clone_to_java<'a>(&self, env: JNIEnv<'a>) -> jni::errors::Result<JValue<'a>> {
        let class_name = match self.stat_type {
            StatType::Avg => "Avg",
            StatType::Min => "Min",
            StatType::Max => "Max",
            StatType::WindowedSum => "WindowedSum",
            StatType::WindowedCount => "WindowedCount",
        };
        let class = env.find_class(&format!(
            "org/apache/kafka/common/metrics/stats/{}",
            class_name
        ))?;
        let obj = env.alloc_object(class)?;

        let copy = Box::new(self.clone());
        let ptr = Box::into_raw(copy);
        env.set_field(
            obj,
            "rustPointer",
            "J",
            jni::objects::JValue::Long(ptr as i64),
        )?;
        Ok(obj.into())
    }
}
clone_from_java!(
    SampledStat,
    "org/apache/kafka/common/metrics/stats/SampledStat"
);
from_jobject!(
    SampledStat,
    "org/apache/kafka/common/metrics/stats/SampledStat"
);

/*
 * Class:     org_apache_kafka_common_metrics_stats_SampledStat
 * Method:    record
 * Signature: (Lorg/apache/kafka/common/metrics/MetricConfig;DJ)V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_SampledStat_record(
    env: JNIEnv,
    obj: JObject,
    config: JObject,
    value: jdouble,
    time_ms: jlong,
) {
    let result = || -> jni::errors::Result<_> {
        let config = MetricConfig::clone_from_java(env, config.into())?;
        let mut stat = SampledStat::from_jobject(env, obj)?;
        stat.modify(|stat| {
            stat.record(&config, value, time_ms as u128);
        });
        Ok(())
    }();
    match result {
        Ok(_) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!("{:?}", result),
    }
}

/*
 * Class:     org_apache_kafka_common_metrics_stats_SampledStat
 * Method:    measure
 * Signature: (Lorg/apache/kafka/common/metrics/MetricConfig;J)D
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_SampledStat_measure(
    env: JNIEnv,
    obj: JObject,
    config: JObject,
    now: jlong,
) -> f64 {
    let result = || -> jni::errors::Result<_> {
        let config = MetricConfig::clone_from_java(env, config.into())?;
        let mut stat = SampledStat::from_jobject(env, obj)?;
        let ret = stat.modify(|stat| stat.measure(&config, now as u128));
        Ok(ret)
    }();
    match result {
        Ok(v) => v,
        Err(jni::errors::Error::JavaException) => Default::default(),
        _ => panic!("{:?}", result),
    }
}

/*
 * Class:     org_apache_kafka_common_metrics_stats_SampledStat
 * Method:    current
 * Signature: (J)Lorg/apache/kafka/common/metrics/stats/SampledStat/Sample;
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_SampledStat_current(
    env: JNIEnv,
    obj: JObject,
    time_ms: jlong,
) -> jobject {
    let result = || -> jni::errors::Result<_> {
        let mut stat = SampledStat::from_jobject(env, obj)?;
        let ret =
            stat.modify(|stat| CloneToJava::clone_to_java(stat.current(time_ms as u128), env));
        ret?.l()
    }();
    match result {
        Ok(v) => v.into_inner(),
        Err(jni::errors::Error::JavaException) => JObject::null().into_inner(),
        _ => panic!("{:?}", result),
    }
}

/*
 * Class:     org_apache_kafka_common_metrics_stats_SampledStat
 * Method:    oldest
 * Signature: (J)Lorg/apache/kafka/common/metrics/stats/SampledStat/Sample;
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_SampledStat_oldest(
    env: JNIEnv,
    obj: JObject,
    time_ms: jlong,
) -> jobject {
    let result = || -> jni::errors::Result<_> {
        let mut stat = SampledStat::from_jobject(env, obj)?;
        let ret = stat.modify(|stat| CloneToJava::clone_to_java(stat.oldest(time_ms as u128), env));
        ret?.l()
    }();
    match result {
        Ok(v) => v.into_inner(),
        Err(jni::errors::Error::JavaException) => JObject::null().into_inner(),
        _ => panic!("{:?}", result),
    }
}

/*
 * Class:     org_apache_kafka_common_metrics_stats_SampledStat
 * Method:    purgeObsoleteSamples
 * Signature: (Lorg/apache/kafka/common/metrics/MetricConfig;J)V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_SampledStat_purgeObsoleteSamples(
    env: JNIEnv,
    obj: JObject,
    config: JObject,
    now: jlong,
) {
    let result = || -> jni::errors::Result<_> {
        let config = MetricConfig::clone_from_java(env, config.into())?;
        let mut stat = SampledStat::from_jobject(env, obj)?;
        stat.modify(|stat| stat.purge_obsolete_samples(&config, now as u128));
        Ok(())
    }();
    match result {
        Ok(_) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!("{:?}", result),
    }
}

fn java_sampled_stat_constructor(env: JNIEnv, obj: JObject, type_: StatType) {
    let result = || -> jni::errors::Result<_> {
        let sampled_stat = Box::new(SampledStat::new(type_));
        let ptr = Box::into_raw(sampled_stat);
        env.set_field(obj, "rustPointer", "J", JValue::Long(ptr as i64))?;

        Ok(())
    }();
    match result {
        Ok(_) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!("{:?}", result),
    }
}
fn java_sampled_stat_destructor(env: JNIEnv, obj: JObject) {
    let result = || -> jni::errors::Result<_> {
        let ptr = env.get_field(obj, "rustPointer", "J")?.j()?;
        let _obj = unsafe { Box::from_raw(ptr as *mut SampledStat) };

        Ok(())
    }();
    match result {
        Ok(_) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!("{:?}", result),
    }
}

fn java_sampled_stat_combine(env: JNIEnv, obj: JObject, samples: JObject) -> jdouble {
    let result = || -> jni::errors::Result<_> {
        let stat = SampledStat::from_jobject(env, obj)?;
        let samples: Vec<Sample> = CloneFromJava::clone_from_java(env, samples.into())?;
        let ret = stat.combine(&samples);
        Ok(ret)
    }();
    match result {
        Ok(v) => v,
        Err(jni::errors::Error::JavaException) => 0_f64,
        _ => panic!("{:?}", result),
    }
}
/*
 * Class:     org_apache_kafka_common_metrics_stats_Avg
 * Method:    rustConstructor
 * Signature: ()V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_Avg_rustConstructor(
    env: JNIEnv,
    obj: JObject,
) {
    java_sampled_stat_constructor(env, obj, StatType::Avg)
}

/*
 * Class:     org_apache_kafka_common_metrics_stats_Avg
 * Method:    rustDestructor
 * Signature: ()V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_Avg_rustDestructor(
    env: JNIEnv,
    obj: JObject,
) {
    java_sampled_stat_destructor(env, obj)
}

/*
 * Class:     org_apache_kafka_common_metrics_stats_Avg
 * Method:    combine
 * Signature: (Ljava/util/List;Lorg/apache/kafka/common/metrics/MetricConfig;J)D
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_Avg_combine(
    env: JNIEnv,
    obj: JObject,
    samples: JObject,
    _config: JObject,
    _now: jlong,
) -> jdouble {
    java_sampled_stat_combine(env, obj, samples)
}

/*
 * Class:     org_apache_kafka_common_metrics_stats_Max
 * Method:    rustConstructor
 * Signature: ()V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_Max_rustConstructor(
    env: JNIEnv,
    obj: JObject,
) {
    java_sampled_stat_constructor(env, obj, StatType::Max)
}

/*
* Class:     org_apache_kafka_common_metrics_stats_Max
* Method:    rustDestructor
* Signature: ()V
*/
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_Max_rustDestructor(
    env: JNIEnv,
    obj: JObject,
) {
    java_sampled_stat_destructor(env, obj)
}

/*
* Class:     org_apache_kafka_common_metrics_stats_Max
* Method:    combine
* Signature: (Ljava/util/List;Lorg/apache/kafka/common/metrics/MetricConfig;J)D
*/
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_Max_combine(
    env: JNIEnv,
    obj: JObject,
    samples: JObject,
    _config: JObject,
    _now: jlong,
) -> jdouble {
    java_sampled_stat_combine(env, obj, samples)
}

/*
 * Class:     org_apache_kafka_common_metrics_stats_Min
 * Method:    rustConstructor
 * Signature: ()V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_Min_rustConstructor(
    env: JNIEnv,
    obj: JObject,
) {
    java_sampled_stat_constructor(env, obj, StatType::Min)
}

/*
 * Class:     org_apache_kafka_common_metrics_stats_Min
 * Method:    rustDestructor
 * Signature: ()V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_Min_rustDestructor(
    env: JNIEnv,
    obj: JObject,
) {
    java_sampled_stat_destructor(env, obj)
}

/*
 * Class:     org_apache_kafka_common_metrics_stats_Min
 * Method:    combine
 * Signature: (Ljava/util/List;Lorg/apache/kafka/common/metrics/MetricConfig;J)D
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_Min_combine(
    env: JNIEnv,
    obj: JObject,
    samples: JObject,
    _config: JObject,
    _now: jlong,
) -> jdouble {
    java_sampled_stat_combine(env, obj, samples)
}

/*
 * Class:     org_apache_kafka_common_metrics_stats_WindowedCount
 * Method:    rustConstructor
 * Signature: ()V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_WindowedCount_rustConstructor(
    env: JNIEnv,
    obj: JObject,
) {
    java_sampled_stat_constructor(env, obj, StatType::WindowedCount)
}

/*
* Class:     org_apache_kafka_common_metrics_stats_WindowedCount
* Method:    rustDestructor
* Signature: ()V
*/
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_WindowedCount_rustDestructor(
    env: JNIEnv,
    obj: JObject,
) {
    java_sampled_stat_destructor(env, obj)
}

/*
 * Class:     org_apache_kafka_common_metrics_stats_WindowedSum
 * Method:    rustConstructor
 * Signature: ()V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_WindowedSum_rustConstructor(
    env: JNIEnv,
    obj: JObject,
) {
    java_sampled_stat_constructor(env, obj, StatType::WindowedSum)
}

/*
 * Class:     org_apache_kafka_common_metrics_stats_WindowedSum
 * Method:    rustDestructor
 * Signature: ()V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_WindowedSum_rustDestructor(
    env: JNIEnv,
    obj: JObject,
) {
    java_sampled_stat_destructor(env, obj)
}

/*
 * Class:     org_apache_kafka_common_metrics_stats_WindowedSum
 * Method:    combine
 * Signature: (Ljava/util/List;Lorg/apache/kafka/common/metrics/MetricConfig;J)D
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_WindowedSum_combine(
    env: JNIEnv,
    obj: JObject,
    samples: JObject,
    _config: JObject,
    _now: jlong,
) -> jdouble {
    java_sampled_stat_combine(env, obj, samples)
}
