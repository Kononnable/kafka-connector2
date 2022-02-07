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
pub struct Percentile {
    metric_name: MetricName,
    percentile: f64,
}

java_struct_standard_impl!(
    Percentile,
    "org/apache/kafka/common/metrics/stats/Percentile"
);

/*
 * Class:     org_apache_kafka_common_metrics_stats_Percentile
 * Method:    rustConstructor
 * Signature: (Lorg/apache/kafka/common/MetricName;D)V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_Percentile_rustConstructor(
    env: JNIEnv,
    obj: JObject,
    metric_name: jobject,
    percentile: jdouble,
) {
    let result = || -> jni::errors::Result<_> {
        let metric_name = MetricName::clone_from_java(env, metric_name.into())?;
        let percentile = Box::new(Percentile {
            metric_name,
            percentile,
        });
        let ptr = Box::into_raw(percentile);
        env.set_field(obj, "rustPointer", "J", JValue::Long(ptr as i64))?;

        Ok(())
    }();
    match result {
        Ok(_) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!("{:?}", result),
    }
}

/*
 * Class:     org_apache_kafka_common_metrics_stats_Percentile
 * Method:    rustDestructor
 * Signature: ()V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_Percentile_rustDestructor(
    env: JNIEnv,
    obj: JObject,
) {
    let result = || -> jni::errors::Result<_> {
        let ptr = env.get_field(obj, "rustPointer", "J")?.j()?;
        let _obj = unsafe { Box::from_raw(ptr as *mut Percentile) };

        Ok(())
    }();
    match result {
        Ok(_) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!("{:?}", result),
    }
}

/*
 * Class:     org_apache_kafka_common_metrics_stats_Percentile
 * Method:    name
 * Signature: ()Lorg/apache/kafka/common/MetricName;
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_Percentile_name(
    env: JNIEnv,
    obj: JObject,
) -> jobject {
    let result = || -> jni::errors::Result<_> {
        let Percentile = Percentile::from_jobject(env, obj)?;
        MetricName::clone_to_java(&Percentile.metric_name, env)?.l()
    }();
    match result {
        Ok(val) => val.into_inner(),
        Err(jni::errors::Error::JavaException) => JObject::null().into_inner(),
        _ => panic!("{:?}", result),
    }
}

/*
 * Class:     org_apache_kafka_common_metrics_stats_Percentile
 * Method:    centerValue
 * Signature: ()D
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_stats_Percentile_percentile(
    env: JNIEnv,
    obj: JObject,
) -> jdouble {
    let result = || -> jni::errors::Result<_> {
        let Percentile = Percentile::from_jobject(env, obj)?;
        Ok(Percentile.percentile)
    }();
    match result {
        Ok(val) => val,
        Err(jni::errors::Error::JavaException) => 0.0,
        _ => panic!("{:?}", result),
    }
}
