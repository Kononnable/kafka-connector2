use jni::{
    objects::{JObject, JValue},
    sys::{jdouble, jobject},
    JNIEnv,
};

use crate::{
    clone_from_java::CloneFromJava, clone_to_java::CloneToJava, common::metric_name::MetricName,
    java_stored_object::FromJObject, java_struct_standard_impl,
};

#[derive(Debug, Clone)]
pub struct Frequency {
    metric_name: MetricName,
    center_value: f64,
}

java_struct_standard_impl!(Frequency, "org/apache/kafka/common/metrics/stats/Frequency");

/*
 * Class:     org_apache_kafka_common_metrics_stats_Frequency
 * Method:    rustConstructor
 * Signature: (Lorg/apache/kafka/common/MetricName;D)V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_Frequency_rustConstructor(
    env: JNIEnv,
    obj: JObject,
    metric_name: jobject,
    center_value: jdouble,
) {
    let result = || -> jni::errors::Result<_> {
        let metric_name = MetricName::clone_from_java(env, metric_name.into())?;
        let histogram = Box::new(Frequency {
            metric_name,
            center_value,
        });
        let ptr = Box::into_raw(histogram);
        env.set_field(obj, "rustPointer", "J", JValue::Long(ptr as i64))?;

        Ok(())
    }();
    match result {
        Ok(_) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!("{:?}", result),
    }
}

/*
 * Class:     org_apache_kafka_common_metrics_stats_Frequency
 * Method:    rustDestructor
 * Signature: ()V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_Frequency_rustDestructor(
    env: JNIEnv,
    obj: JObject,
) {
    let result = || -> jni::errors::Result<_> {
        let ptr = env.get_field(obj, "rustPointer", "J")?.j()?;
        let _obj = unsafe { Box::from_raw(ptr as *mut Frequency) };

        Ok(())
    }();
    match result {
        Ok(_) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!("{:?}", result),
    }
}

/*
 * Class:     org_apache_kafka_common_metrics_stats_Frequency
 * Method:    name
 * Signature: ()Lorg/apache/kafka/common/MetricName;
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_Frequency_name(
    env: JNIEnv,
    obj: JObject,
) -> jobject {
    let result = || -> jni::errors::Result<_> {
        let frequency = Frequency::from_jobject(env, obj)?;
        MetricName::clone_to_java(&frequency.metric_name, env)?.l()
    }();
    match result {
        Ok(val) => val.into_inner(),
        Err(jni::errors::Error::JavaException) => JObject::null().into_inner(),
        _ => panic!("{:?}", result),
    }
}

/*
 * Class:     org_apache_kafka_common_metrics_stats_Frequency
 * Method:    centerValue
 * Signature: ()D
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_Frequency_centerValue(
    env: JNIEnv,
    obj: JObject,
) -> jdouble {
    let result = || -> jni::errors::Result<_> {
        let frequency = Frequency::from_jobject(env, obj)?;
        Ok(frequency.center_value)
    }();
    match result {
        Ok(val) => val,
        Err(jni::errors::Error::JavaException) => 0.0,
        _ => panic!("{:?}", result),
    }
}
