use jni::{
    objects::{JObject, JValue},
    sys::{jdouble, jlong},
    JNIEnv,
};

use crate::{
    clone_from_java::CloneFromJava, clone_to_from_java_for_struct,
    common::metrics::metric_config::MetricConfig, java_stored_object::FromJObject,
};

#[derive(Debug, Clone)]
pub struct CumulativeStat {
    total: f64,
    stat_type: StatType,
}
#[derive(Debug, Clone, Copy)]
pub enum StatType {
    CumulativeSum,
    CumulativeCount,
}
impl CumulativeStat {
    pub fn new(value: f64, stat_type: StatType) -> CumulativeStat {
        CumulativeStat {
            total: value,
            stat_type,
        }
    }
    pub fn record(&mut self, _config: &MetricConfig, value: f64, _time_ms: u128) {
        match self.stat_type {
            StatType::CumulativeSum => self.total += value,
            StatType::CumulativeCount => self.total += 1_f64,
        }
    }

    pub fn measure(&mut self, _config: &MetricConfig, _now: u128) -> f64 {
        self.total
    }
}

clone_to_from_java_for_struct!(
    CumulativeStat,
    "org/apache/kafka/common/metrics/stats/CumulativeSum"
);

fn java_cumulative_stat_record(
    env: JNIEnv,
    obj: JObject,
    config: JObject,
    value: jdouble,
    time_ms: jlong,
) {
    let result = || -> jni::errors::Result<_> {
        let config = MetricConfig::clone_from_java(env, config.into()).unwrap_or_default();
        let mut stat = CumulativeStat::from_jobject(env, obj)?;
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
 * Class:     org_apache_kafka_common_metrics_stats_CumulativeSum
 * Method:    record
 * Signature: (Lorg/apache/kafka/common/metrics/MetricConfig;DJ)V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_CumulativeSum_record(
    env: JNIEnv,
    obj: JObject,
    config: JObject,
    value: jdouble,
    time_ms: jlong,
) {
    java_cumulative_stat_record(env, obj, config, value, time_ms);
}
/*
 * Class:     org_apache_kafka_common_metrics_stats_CumulativeCount
 * Method:    record
 * Signature: (Lorg/apache/kafka/common/metrics/MetricConfig;DJ)V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_CumulativeCount_record(
    env: JNIEnv,
    obj: JObject,
    config: JObject,
    value: jdouble,
    time_ms: jlong,
) {
    java_cumulative_stat_record(env, obj, config, value, time_ms);
}

/*
 * Class:     org_apache_kafka_common_metrics_stats_CumulativeSum
 * Method:    measure
 * Signature: (Lorg/apache/kafka/common/metrics/MetricConfig;J)D
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_CumulativeSum_measure(
    env: JNIEnv,
    obj: JObject,
    config: JObject,
    now: jlong,
) -> f64 {
    let result = || -> jni::errors::Result<_> {
        let config = MetricConfig::clone_from_java(env, config.into())?;
        let mut stat = CumulativeStat::from_jobject(env, obj)?;
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
 * Class:     org_apache_kafka_common_metrics_stats_CumulativeSum
 * Method:    rustConstructor
 * Signature: (D)V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_CumulativeSum_rustConstructor(
    env: JNIEnv,
    obj: JObject,
    value: jdouble,
) {
    let result = || -> jni::errors::Result<_> {
        let cumulative_stat = Box::new(CumulativeStat::new(value, StatType::CumulativeSum));
        let ptr = Box::into_raw(cumulative_stat);
        env.set_field(obj, "rustPointer", "J", JValue::Long(ptr as i64))?;

        Ok(())
    }();
    match result {
        Ok(_) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!("{:?}", result),
    }
}

/*
 * Class:     org_apache_kafka_common_metrics_stats_CumulativeCount
 * Method:    rustConstructor
 * Signature: (D)V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_CumulativeCount_rustConstructor(
    env: JNIEnv,
    obj: JObject,
    value: jdouble,
) {
    let result = || -> jni::errors::Result<_> {
        let cumulative_stat = Box::new(CumulativeStat::new(value, StatType::CumulativeCount));
        let ptr = Box::into_raw(cumulative_stat);
        env.set_field(obj, "rustPointer", "J", JValue::Long(ptr as i64))?;

        Ok(())
    }();
    match result {
        Ok(_) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!("{:?}", result),
    }
}

/*
 * Class:     org_apache_kafka_common_metrics_stats_CumulativeSum
 * Method:    rustDestructor
 * Signature: ()V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_CumulativeSum_rustDestructor(
    env: JNIEnv,
    obj: JObject,
) {
    let result = || -> jni::errors::Result<_> {
        let ptr = env.get_field(obj, "rustPointer", "J")?.j()?;
        let _obj = unsafe { Box::from_raw(ptr as *mut CumulativeStat) };

        Ok(())
    }();
    match result {
        Ok(_) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!("{:?}", result),
    }
}
