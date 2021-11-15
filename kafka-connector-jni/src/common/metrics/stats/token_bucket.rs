use crate::{
    clone_to_from_java::{clone_to_from_java_for_struct, CloneToFromJava},
    common::metrics::{
        internals::metric_utils::TimeUnit, metric_config::MetricConfig, quota::Quota,
    },
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

clone_to_from_java_for_struct!(
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
 * Method:    rustDeconstructor
 * Signature: ()V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_TokenBucket_rustDeconstructor(
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
        let mut token_bucket = TokenBucket::clone_from_java(env, obj.into())?;
        let ret = token_bucket.measure(&config, time_ms as u128);
        token_bucket.replace_java_obj(env, obj)?;
        Ok(ret)
        // modify_java_obj(
        //     env,
        //     obj,
        //     "org/apache/kafka/common/metrics/stats/TokenBucket",
        //     |x: &mut TokenBucket| x.measure(&config, time_ms as u128),
        // )
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
        let mut token_bucket = TokenBucket::clone_from_java(env, obj.into())?;
        token_bucket.record(&config, value, time_ms as u128);
        token_bucket.replace_java_obj(env, obj)?;
        Ok(())
        // modify_java_obj(
        //     env,
        //     obj,
        //     "org/apache/kafka/common/metrics/stats/TokenBucket",
        //     |x: &mut TokenBucket| x.record(&config, value, time_ms as u128),
        // )
    }();
    match result {
        Ok(_) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!("{:?}", result),
    }
}

// fn modify_java_obj<T, F, R>(
//     env: JNIEnv,
//     obj: JObject,
//     class_name: &str,
//     mut func: F,
// ) -> jni::errors::Result<R>
// where
//     F: FnMut(&mut T) -> R,
// {
//     let class = env.find_class(class_name)?;
//     if !env.is_instance_of(obj, class)? {
//         env.throw_new("java/lang/Exception", "Wrong object class")?;
//         return Err(jni::errors::Error::JavaException);
//     }
//     let ptr = env.get_field(obj, "rustPointer", "J")?.j()?;
//     let mut this = unsafe { Box::from_raw(ptr as *mut T) };
//     let ret = func(&mut this);

//     let _ptr = Box::into_raw(this);
//     Ok(ret)
// }
