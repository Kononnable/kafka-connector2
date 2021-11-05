use crate::clone_to_from_java::{clone_to_from_java_for_struct, CloneToFromJava};
use indexmap::IndexMap;
use jni::{
    objects::{JObject, JValue},
    sys::jstring,
    JNIEnv,
};
use kafka_connector_macros::rust_property_getter;

#[derive(Debug, Clone)]
pub struct MetricName {
    pub name: String,
    pub group: String,
    pub description: String,
    pub tags: IndexMap<String, String>,
}
impl MetricName {
    pub fn new(
        name: String,
        group: String,
        description: String,
        tags: IndexMap<String, String>,
    ) -> MetricName {
        MetricName {
            name,
            group,
            description,
            tags,
        }
    }
}

clone_to_from_java_for_struct!(MetricName, "org/apache/kafka/common/MetricName");

/*
 * Class:     org_apache_kafka_common_MetricName
 * Method:    rustConstructor
 * Signature: (Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/util/Map;)V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_MetricName_rustConstructor(
    env: JNIEnv,
    obj: JObject,
    name: jstring,
    group: jstring,
    description: jstring,
    tags: JObject,
) {
    let result = || -> jni::errors::Result<_> {
        let name: String = env.get_string(name.into())?.into();
        let group: String = env.get_string(group.into())?.into();
        let description: String = env.get_string(description.into())?.into();
        let tags = CloneToFromJava::clone_from_java(env, JValue::Object(tags))?;

        let record_header = Box::new(MetricName::new(name, group, description, tags));
        let ptr = Box::into_raw(record_header);
        env.set_field(obj, "rustPointer", "J", JValue::Long(ptr as i64))?;

        Ok(())
    }();
    match result {
        Ok(_) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!("{:?}", result),
    }
}

/*
 * Class:     org_apache_kafka_common_MetricName
 * Method:    rustDeconstructor
 * Signature: ()V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_MetricName_rustDeconstructor(
    env: JNIEnv,
    obj: JObject,
) {
    let result = || -> jni::errors::Result<_> {
        let ptr = env.get_field(obj, "rustPointer", "J")?.j()?;
        let _obj = unsafe { Box::from_raw(ptr as *mut MetricName) };

        Ok(())
    }();
    match result {
        Ok(_) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!("{:?}", result),
    }
}

rust_property_getter!(
 * Struct:    MetricName
 * Class:     org_apache_kafka_common_MetricName
 * Method:    name
 * Signature: ()Ljava/lang/String;
);

rust_property_getter!(
 * Struct:    MetricName
 * Class:     org_apache_kafka_common_MetricName
 * Method:    group
 * Signature: ()Ljava/lang/String;
);

rust_property_getter!(
 * Struct:    MetricName
 * Class:     org_apache_kafka_common_MetricName
 * Method:    tags
 * Signature: ()Ljava/util/Map;
);

rust_property_getter!(
 * Struct:    MetricName
 * Class:     org_apache_kafka_common_MetricName
 * Method:    description
 * Signature: ()Ljava/lang/String;
);
