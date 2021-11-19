use jni::{
    objects::{JObject, JValue},
    sys::{jdouble, jlong},
    JNIEnv,
};
use kafka_connector_macros::{rust_property_chain_setter, rust_property_getter};

use crate::{
    clone_from_java::CloneFromJava, clone_to_from_java_for_struct,
    common::metrics::metric_config::MetricConfig, java_stored_object::FromJObject,
};

#[derive(Debug, Clone)]
pub struct Sample {
    pub initial_value: f64,
    pub event_count: u64,
    pub last_window_ms: u128,
    pub value: f64,
}
clone_to_from_java_for_struct!(Sample, "org/apache/kafka/common/metrics/stats/Sample");

impl Sample {
    pub fn new(initial_value: f64, now: u128) -> Sample {
        Sample {
            event_count: 0,
            initial_value,
            last_window_ms: now,
            value: initial_value,
        }
    }
    pub fn reset(&mut self, now: u128) {
        self.event_count = 0;
        self.last_window_ms = now;
        self.value = self.initial_value;
    }
    pub fn is_complete(&self, time_ms: u128, config: &MetricConfig) -> bool {
        u128::saturating_sub(time_ms, self.last_window_ms) >= config.time_window_ms
            || self.event_count >= config.event_window
    }
}

/*
 * Class:     org_apache_kafka_common_metrics_stats_Sample
 * Method:    rustConstructor
 * Signature: (DJ)V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_Sample_rustConstructor(
    env: JNIEnv,
    obj: JObject,
    initial_value: jdouble,
    now: jlong,
) {
    let result = || -> jni::errors::Result<_> {
        let token_bucket = Box::new(Sample::new(initial_value, now as u128));
        let ptr = Box::into_raw(token_bucket);
        env.set_field(obj, "rustPointer", "J", JValue::Long(ptr as i64))?;

        Ok(())
    }();
    match result {
        Ok(_) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!("{:?}", result),
    }
}

/*
 * Class:     org_apache_kafka_common_metrics_stats_Sample
 * Method:    rustDestructor
 * Signature: ()V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_Sample_rustDestructor(
    env: JNIEnv,
    obj: JObject,
) {
    let result = || -> jni::errors::Result<_> {
        let ptr = env.get_field(obj, "rustPointer", "J")?.j()?;
        let _obj = unsafe { Box::from_raw(ptr as *mut Sample) };

        Ok(())
    }();
    match result {
        Ok(_) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!("{:?}", result),
    }
}

rust_property_getter!(
 * Function:  Java_org_apache_kafka_common_metrics_stats_Sample_eventCount__
 * Struct:    Sample
 * Class:     org_apache_kafka_common_metrics_stats_Sample
 * Method:    eventCount
 * Signature: ()J
);

rust_property_chain_setter!(
 * Function:  Java_org_apache_kafka_common_metrics_stats_Sample_eventCount__J
 * Struct:    Sample
 * Class:     org_apache_kafka_common_metrics_stats_Sample
 * Method:    eventCount
 * Signature: (J)Lorg/apache/kafka/common/metrics/stats/Sample;
);

rust_property_getter!(
 * Struct:    Sample
 * Class:     org_apache_kafka_common_metrics_stats_Sample
 * Method:    lastWindowMs
 * Signature: ()J
);

rust_property_getter!(
 * Struct:    Sample
 * Class:     org_apache_kafka_common_metrics_stats_Sample
 * Method:    value
 * Signature: ()D
);

/*
 * Class:     org_apache_kafka_common_metrics_stats_Sample
 * Method:    reset
 * Signature: (J)V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_Sample_reset(
    env: JNIEnv,
    obj: JObject,
    now: jlong,
) {
    let result = || -> jni::errors::Result<_> {
        let mut sample = Sample::from_jobject(env, obj)?;
        sample.modify(|stat| stat.reset(now as u128));
        Ok(())
    }();
    match result {
        Ok(_) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!("{:?}", result),
    }
}

/*
 * Class:     org_apache_kafka_common_metrics_stats_Sample
 * Method:    isComplete
 * Signature: (JLorg/apache/kafka/common/metrics/MetricConfig;)Z
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_Sample_isComplete(
    env: JNIEnv,
    obj: JObject,
    time_ms: jlong,
    config: JObject,
) -> bool {
    let result = || -> jni::errors::Result<_> {
        let config = MetricConfig::clone_from_java(env, config.into())?;
        let sample = Sample::from_jobject(env, obj)?;
        let ret = sample.is_complete(time_ms as u128, &config);
        Ok(ret)
    }();
    match result {
        Ok(v) => v,
        Err(jni::errors::Error::JavaException) => false,
        _ => panic!("{:?}", result),
    }
}
