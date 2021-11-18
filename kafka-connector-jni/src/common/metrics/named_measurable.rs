use crate::{
    clone_to_from_java::{clone_to_from_java_for_struct, CloneToFromJava},
    common::{metric_name::MetricName, metrics::measurable::JavaMeasurable},
};
use jni::{
    objects::{JObject, JValue},
    JNIEnv,
};
use kafka_connector_macros::rust_property_getter;

use super::measurable::Measurable;

#[derive(Clone)]
pub struct NamedMeasurable {
    pub name: MetricName,
    pub stat: Measurable,
}

clone_to_from_java_for_struct!(
    NamedMeasurable,
    "org/apache/kafka/common/metrics/NamedMeasurable"
);

/*
 * Class:     org_apache_kafka_common_metrics_NamedMeasurable
 * Method:    rustConstructor
 * Signature: (Lorg/apache/kafka/common/MetricName;Lorg/apache/kafka/common/metrics/Measurable;)V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_NamedMeasurable_rustConstructor(
    env: JNIEnv,
    obj: JObject,
    metric_name: JObject,
    measurable: JObject,
) {
    let result = || -> jni::errors::Result<_> {
        let metric_name = MetricName::clone_from_java(env, metric_name.into())?;
        let measurable = JavaMeasurable::new(env, measurable)?;
        let metrics_config = Box::new(NamedMeasurable {
            name: metric_name,
            stat: Measurable::Java(measurable),
        });
        let ptr = Box::into_raw(metrics_config);
        env.set_field(obj, "rustPointer", "J", JValue::Long(ptr as i64))?;

        Ok(())
    }();
    match result {
        Ok(_) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!("{:?}", result),
    }
}

/*
 * Class:     org_apache_kafka_common_metrics_NamedMeasurable
 * Method:    rustDestructor
 * Signature: ()V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_NamedMeasurable_rustDestructor(
    env: JNIEnv,
    obj: JObject,
) {
    let result = || -> jni::errors::Result<_> {
        let ptr = env.get_field(obj, "rustPointer", "J")?.j()?;
        let _obj = unsafe { Box::from_raw(ptr as *mut NamedMeasurable) };

        Ok(())
    }();
    match result {
        Ok(_) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!("{:?}", result),
    }
}

rust_property_getter!(
 * Struct:    NamedMeasurable
 * Class:     org_apache_kafka_common_metrics_NamedMeasurable
 * Method:    name
 * Signature: ()Lorg/apache/kafka/common/MetricName;
);

rust_property_getter!(
 * Struct:    NamedMeasurable
 * Class:     org_apache_kafka_common_metrics_NamedMeasurable
 * Method:    stat
 * Signature: ()Lorg/apache/kafka/common/metrics/Measurable;
);
