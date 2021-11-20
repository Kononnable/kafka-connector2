use jni::{
    objects::{JObject, JValue},
    sys::{jdouble, jlong},
    JNIEnv,
};
use kafka_connector_macros::rust_property_getter;

use crate::{
    clone_from_java::CloneFromJava,
    common::metrics::{internals::metric_utils::TimeUnit, metric_config::MetricConfig},
    java_stored_object::FromJObject,
    java_struct_standard_impl,
};

use super::sampled_stat::SampledStat;

#[derive(Debug, Clone)]
pub struct Rate {
    time_unit: TimeUnit,
    stat: SampledStat,
    rate_type: RateType,
}
#[derive(Debug, Clone, Copy)]
pub enum RateType {
    Standard,
    Simple,
}
impl Rate {
    pub fn new(time_unit: TimeUnit, stat: SampledStat, rate_type: RateType) -> Rate {
        Rate {
            rate_type,
            stat,
            time_unit,
        }
    }
    pub fn record(&mut self, config: &MetricConfig, value: f64, time_ms: u128) {
        self.stat.record(config, value, time_ms)
    }

    pub fn measure(&mut self, config: &MetricConfig, now: u128) -> f64 {
        let value = self.stat.measure(config, now);
        let window_size = self.window_size(config, now);
        value / self.time_unit.convert(window_size)
    }
    pub fn window_size(&mut self, config: &MetricConfig, now: u128) -> u128 {
        self.stat.purge_obsolete_samples(config, now);
        let mut elapsed = u128::saturating_sub(now, self.stat.oldest(now).last_window_ms);
        match self.rate_type {
            RateType::Standard => {
                let num_full_windows = (elapsed / config.time_window_ms) as u32;
                let min_full_windows = config.samples - 1;
                if num_full_windows < min_full_windows {
                    elapsed +=
                        ((min_full_windows - num_full_windows) as u128) * config.time_window_ms;
                }
                elapsed
            }
            RateType::Simple => u128::max(elapsed, config.time_window_ms),
        }
    }
}

java_struct_standard_impl!(Rate, "org/apache/kafka/common/metrics/stats/Rate");

/*
 * Class:     org_apache_kafka_common_metrics_stats_SimpleRate
 * Method:    rustConstructor
 * Signature: (Ljava/util/concurrent/TimeUnit;Lorg/apache/kafka/common/metrics/stats/SampledStat;)V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_SimpleRate_rustConstructor(
    env: JNIEnv,
    obj: JObject,
    time_unit: JObject,
    stat: JObject,
) {
    let result = || -> jni::errors::Result<_> {
        let time_unit = TimeUnit::clone_from_java(env, time_unit.into())?;
        let stat = SampledStat::clone_from_java(env, stat.into())?;
        let rate = Box::new(Rate::new(time_unit, stat, RateType::Simple));
        let ptr = Box::into_raw(rate);
        env.set_field(obj, "rustPointer", "J", JValue::Long(ptr as i64))?;

        Ok(())
    }();
    match result {
        Ok(_) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!("{:?}", result),
    }
}
/*
 * Class:     org_apache_kafka_common_metrics_stats_Rate
 * Method:    rustConstructor
 * Signature: (Ljava/util/concurrent/TimeUnit;Lorg/apache/kafka/common/metrics/stats/SampledStat;)V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_Rate_rustConstructor(
    env: JNIEnv,
    obj: JObject,
    time_unit: JObject,
    stat: JObject,
) {
    let result = || -> jni::errors::Result<_> {
        let time_unit = TimeUnit::clone_from_java(env, time_unit.into())?;
        let stat = SampledStat::clone_from_java(env, stat.into())?;
        let rate = Box::new(Rate::new(time_unit, stat, RateType::Standard));
        let ptr = Box::into_raw(rate);
        env.set_field(obj, "rustPointer", "J", JValue::Long(ptr as i64))?;

        Ok(())
    }();
    match result {
        Ok(_) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!("{:?}", result),
    }
}
/*
 * Class:     org_apache_kafka_common_metrics_stats_Rate
 * Method:    rustDestructor
 * Signature: ()V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_Rate_rustDestructor(
    env: JNIEnv,
    obj: JObject,
) {
    let result = || -> jni::errors::Result<_> {
        let ptr = env.get_field(obj, "rustPointer", "J")?.j()?;
        let _obj = unsafe { Box::from_raw(ptr as *mut Rate) };

        Ok(())
    }();
    match result {
        Ok(_) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!("{:?}", result),
    }
}

rust_property_getter!(
 * Struct:    Rate
 * Class:     org_apache_kafka_common_metrics_stats_Rate
 * Method:    stat
 * Signature: ()Lorg/apache/kafka/common/metrics/stats/SampledStat;
);

/*
 * Class:     org_apache_kafka_common_metrics_stats_Rate
 * Method:    record
 * Signature: (Lorg/apache/kafka/common/metrics/MetricConfig;DJ)V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_Rate_record(
    env: JNIEnv,
    obj: JObject,
    config: JObject,
    value: jdouble,
    time_ms: jlong,
) {
    let result = || -> jni::errors::Result<_> {
        let config = MetricConfig::clone_from_java(env, config.into()).unwrap_or_default();
        let mut rate = Rate::from_jobject(env, obj)?;
        rate.modify(|rate| {
            rate.record(&config, value, time_ms as u128);
        });
        Ok(())
    }();
    match result {
        Ok(_) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!("{:?}", result),
    }
}

/*
 * Class:     org_apache_kafka_common_metrics_stats_Rate
 * Method:    measure
 * Signature: (Lorg/apache/kafka/common/metrics/MetricConfig;J)D
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_Rate_measure(
    env: JNIEnv,
    obj: JObject,
    config: JObject,
    now: jlong,
) -> f64 {
    let result = || -> jni::errors::Result<_> {
        let config = MetricConfig::clone_from_java(env, config.into())?;
        let mut rate = Rate::from_jobject(env, obj)?;
        let ret = rate.modify(|rate| rate.measure(&config, now as u128));
        Ok(ret)
    }();
    match result {
        Ok(v) => v,
        Err(jni::errors::Error::JavaException) => Default::default(),
        _ => panic!("{:?}", result),
    }
}

/*
 * Class:     org_apache_kafka_common_metrics_stats_Rate
 * Method:    windowSize
 * Signature: (Lorg/apache/kafka/common/metrics/MetricConfig;J)J
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_Rate_windowSize(
    env: JNIEnv,
    obj: JObject,
    config: JObject,
    now: jlong,
) -> i64 {
    let result = || -> jni::errors::Result<_> {
        let config = MetricConfig::clone_from_java(env, config.into())?;
        let mut rate = Rate::from_jobject(env, obj)?;
        let ret = rate.modify(|rate| rate.window_size(&config, now as u128)) as i64;
        Ok(ret)
    }();
    match result {
        Ok(v) => v,
        Err(jni::errors::Error::JavaException) => Default::default(),
        _ => panic!("{:?}", result),
    }
}

/*
 * Class:     org_apache_kafka_common_metrics_stats_SimpleRate
 * Method:    windowSize
 * Signature: (Lorg/apache/kafka/common/metrics/MetricConfig;J)J
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_SimpleRate_windowSize(
    env: JNIEnv,
    obj: JObject,
    config: JObject,
    now: jlong,
) -> i64 {
    Java_org_apache_kafka_common_metrics_stats_Rate_windowSize(env, obj, config, now)
}
