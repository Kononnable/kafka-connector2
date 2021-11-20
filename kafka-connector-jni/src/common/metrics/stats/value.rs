use jni::{
    objects::{JObject, JValue},
    sys::{jdouble, jlong},
    JNIEnv,
};

use crate::{
    clone_from_java::CloneFromJava, common::metrics::metric_config::MetricConfig,
    java_stored_object::FromJObject, java_struct_standard_impl,
};

#[derive(Debug, Clone, Default)]
pub struct Value {
    value: f64,
}

impl Value {
    pub fn record(&mut self, _config: &MetricConfig, value: f64, _time_ms: u128) {
        self.value = value;
    }

    pub fn measure(&mut self, _config: &MetricConfig, _now: u128) -> f64 {
        self.value
    }
}
java_struct_standard_impl!(Value, "org/apache/kafka/common/metrics/stats/Value");

/*
 * Class:     org_apache_kafka_common_metrics_stats_Value
 * Method:    record
 * Signature: (Lorg/apache/kafka/common/metrics/MetricConfig;DJ)V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_Value_record(
    env: JNIEnv,
    obj: JObject,
    config: JObject,
    value: jdouble,
    time_ms: jlong,
) {
    let result = || -> jni::errors::Result<_> {
        let config = MetricConfig::clone_from_java(env, config.into()).unwrap_or_default();
        let mut stat = Value::from_jobject(env, obj)?;
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
 * Class:     org_apache_kafka_common_metrics_stats_Value
 * Method:    measure
 * Signature: (Lorg/apache/kafka/common/metrics/MetricConfig;J)D
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_Value_measure(
    env: JNIEnv,
    obj: JObject,
    config: JObject,
    now: jlong,
) -> f64 {
    let result = || -> jni::errors::Result<_> {
        let config = MetricConfig::clone_from_java(env, config.into())?;
        let mut stat = Value::from_jobject(env, obj)?;
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
 * Class:     org_apache_kafka_common_metrics_stats_Value
 * Method:    rustConstructor
 * Signature: ()V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_Value_rustConstructor(
    env: JNIEnv,
    obj: JObject,
) {
    let result = || -> jni::errors::Result<_> {
        let value = Box::new(Value::default());
        let ptr = Box::into_raw(value);
        env.set_field(obj, "rustPointer", "J", JValue::Long(ptr as i64))?;

        Ok(())
    }();
    match result {
        Ok(_) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!("{:?}", result),
    }
}

/*
 * Class:     org_apache_kafka_common_metrics_stats_Value
 * Method:    rustDestructor
 * Signature: ()V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_Value_rustDestructor(
    env: JNIEnv,
    obj: JObject,
) {
    let result = || -> jni::errors::Result<_> {
        let ptr = env.get_field(obj, "rustPointer", "J")?.j()?;
        let _obj = unsafe { Box::from_raw(ptr as *mut Value) };

        Ok(())
    }();
    match result {
        Ok(_) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!("{:?}", result),
    }
}
