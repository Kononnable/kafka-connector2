use crate::{clone_from_java::CloneFromJava, java_struct_standard_impl};
use indexmap::IndexSet;
use jni::{
    objects::{JObject, JValue},
    sys::jstring,
    JNIEnv,
};
use kafka_connector_macros::rust_property_getter;

#[derive(Debug, Clone)]
pub struct MetricNameTemplate {
    pub name: String,
    pub group: String,
    pub description: String,
    pub tags: IndexSet<String>,
}
impl MetricNameTemplate {
    pub fn new(
        name: String,
        group: String,
        description: String,
        tags: IndexSet<String>,
    ) -> MetricNameTemplate {
        MetricNameTemplate {
            name,
            group,
            description,
            tags,
        }
    }
}

java_struct_standard_impl!(
    MetricNameTemplate,
    "org/apache/kafka/common/MetricNameTemplate"
);

/*
 * Class:     org_apache_kafka_common_MetricNameTemplate
 * Method:    rustConstructor
 * Signature: (Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/util/Set;)V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_MetricNameTemplate_rustConstructor(
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
        let tags = CloneFromJava::clone_from_java(env, JValue::Object(tags))?;

        let record_header = Box::new(MetricNameTemplate::new(name, group, description, tags));
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
 * Class:     org_apache_kafka_common_MetricNameTemplate
 * Method:    rustDestructor
 * Signature: ()V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_MetricNameTemplate_rustDestructor(
    env: JNIEnv,
    obj: JObject,
) {
    let result = || -> jni::errors::Result<_> {
        let ptr = env.get_field(obj, "rustPointer", "J")?.j()?;
        let _obj = unsafe { Box::from_raw(ptr as *mut MetricNameTemplate) };

        Ok(())
    }();
    match result {
        Ok(_) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!("{:?}", result),
    }
}

rust_property_getter!(
 * Struct:    MetricNameTemplate
 * Class:     org_apache_kafka_common_MetricNameTemplate
 * Method:    name
 * Signature: ()Ljava/lang/String;
);

rust_property_getter!(
 * Struct:    MetricNameTemplate
 * Class:     org_apache_kafka_common_MetricNameTemplate
 * Method:    group
 * Signature: ()Ljava/lang/String;
);

rust_property_getter!(
 * Struct:    MetricNameTemplate
 * Class:     org_apache_kafka_common_MetricNameTemplate
 * Method:    tags
 * Signature: ()Ljava/util/Map;
);

rust_property_getter!(
 * Struct:    MetricNameTemplate
 * Class:     org_apache_kafka_common_MetricNameTemplate
 * Method:    description
 * Signature: ()Ljava/lang/String;
);
