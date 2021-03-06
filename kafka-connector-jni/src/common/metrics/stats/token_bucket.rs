use crate::{
    clone_from_java::CloneFromJava,
    common::metrics::{
        internals::metric_utils::TimeUnit, metric_config::MetricConfig, quota::Quota,
    },
    java_stored_object::FromJObject,
    java_struct_standard_impl,
};
use jni::{
    objects::{JObject, JValue},
    sys::{jdouble, jlong},
    JNIEnv,
};
use kafka_connector_macros::rust_property_getter;

#[derive(Clone)]
pub struct TokenBucket {
    pub unit: TimeUnit,
    pub tokens: f64,
    pub last_update_ms: u128,
}

impl TokenBucket {
    pub fn measure(&mut self, config: &MetricConfig, time_ms: u128) -> f64 {
        match config.quota {
            Some(Quota { bound, .. }) => {
                let burst = self.burst(config);
                self.refill(bound, burst, time_ms);
                self.tokens
            }
            None => f64::MAX,
        }
    }
    pub fn record(&mut self, config: &MetricConfig, value: f64, time_ms: u128) {
        if let Some(Quota { bound, .. }) = config.quota {
            let burst = self.burst(config);
            self.refill(bound, burst, time_ms);
            self.tokens = f64::min(burst, self.tokens - value)
        }
    }

    fn refill(&mut self, quota: f64, burst: f64, time_ms: u128) {
        self.tokens = f64::min(
            burst,
            self.tokens + quota * self.unit.convert(time_ms - self.last_update_ms),
        );
        self.last_update_ms = time_ms;
    }
    fn burst(&self, config: &MetricConfig) -> f64 {
        (config.samples as f64)
            * self.unit.convert(config.time_window_ms)
            * config.quota.as_ref().unwrap().bound
    }
}

java_struct_standard_impl!(
    TokenBucket,
    "org/apache/kafka/common/metrics/stats/TokenBucket"
);

/*
 * Class:     org_apache_kafka_common_metrics_stats_TokenBucket
 * Method:    rustConstructor
 * Signature: (Ljava/util/concurrent/TimeUnit;)V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_TokenBucket_rustConstructor(
    env: JNIEnv,
    obj: JObject,
    time_unit: JObject,
) {
    let result = || -> jni::errors::Result<_> {
        let time_unit = TimeUnit::clone_from_java(env, time_unit.into())?;
        let token_bucket = Box::new(TokenBucket {
            unit: time_unit,
            tokens: 0f64,
            last_update_ms: 0,
        });
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
 * Class:     org_apache_kafka_common_metrics_stats_TokenBucket
 * Method:    rustDestructor
 * Signature: ()V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_TokenBucket_rustDestructor(
    env: JNIEnv,
    obj: JObject,
) {
    let result = || -> jni::errors::Result<_> {
        let ptr = env.get_field(obj, "rustPointer", "J")?.j()?;
        let _obj = unsafe { Box::from_raw(ptr as *mut TokenBucket) };

        Ok(())
    }();
    match result {
        Ok(_) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!("{:?}", result),
    }
}

rust_property_getter!(
 * Struct:    TokenBucket
 * Class:     org_apache_kafka_common_metrics_stats_TokenBucket
 * Method:    unit
 * Signature: ()Ljava/util/concurrent/TimeUnit;
);

rust_property_getter!(
 * Struct:    TokenBucket
 * Class:     org_apache_kafka_common_metrics_stats_TokenBucket
 * Method:    tokens
 * Signature: ()D
);

rust_property_getter!(
 * Struct:    TokenBucket
 * Class:     org_apache_kafka_common_metrics_stats_TokenBucket
 * Method:    lastUpdateMs
 * Signature: ()J
);

/*
 * Class:     org_apache_kafka_common_metrics_stats_TokenBucket
 * Method:    measure
 * Signature: (Lorg/apache/kafka/common/metrics/MetricConfig;J)D
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_TokenBucket_measure(
    env: JNIEnv,
    obj: JObject,
    config: JObject,
    time_ms: jlong,
) -> jdouble {
    let result = || -> jni::errors::Result<_> {
        let config = MetricConfig::clone_from_java(env, config.into())?;
        let mut token_bucket = TokenBucket::from_jobject(env, obj)?;
        Ok(token_bucket.modify(|bucket| bucket.measure(&config, time_ms as u128)))
    }();
    match result {
        Ok(v) => v,
        Err(jni::errors::Error::JavaException) => 0f64,
        _ => panic!("{:?}", result),
    }
}
/*
 * Class:     org_apache_kafka_common_metrics_stats_TokenBucket
 * Method:    record
 * Signature: (Lorg/apache/kafka/common/metrics/MetricConfig;DJ)V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_TokenBucket_record(
    env: JNIEnv,
    obj: JObject,
    config: JObject,
    value: jdouble,
    time_ms: jlong,
) {
    let result = || -> jni::errors::Result<_> {
        let config = MetricConfig::clone_from_java(env, config.into())?;
        let mut token_bucket = TokenBucket::from_jobject(env, obj)?;
        token_bucket.modify(|bucket| bucket.record(&config, value, time_ms as u128));
        Ok(())
    }();
    match result {
        Ok(_) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!("{:?}", result),
    }
}
