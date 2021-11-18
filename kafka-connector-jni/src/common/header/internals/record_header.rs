use crate::clone_to_from_java::clone_to_from_java_for_struct;
use bytes::Bytes;
use jni::{
    objects::{JObject, JValue},
    sys::{jbyteArray, jstring},
    JNIEnv,
};
use kafka_connector_macros::rust_property_getter;

#[derive(Debug, Clone)]
pub struct RecordHeader {
    pub key: String,
    pub value: Bytes,
}
impl RecordHeader {
    pub fn new(key: String, value: impl Into<Bytes>) -> RecordHeader {
        let value = value.into();
        RecordHeader { key, value }
    }
}

clone_to_from_java_for_struct!(
    RecordHeader,
    "org/apache/kafka/common/header/internals/RecordHeader"
);

/*
 * Class:     org_apache_kafka_common_header_internals_RecordHeader
 * Method:    rustConstructor
 * Signature: (Ljava/lang/String;[B)V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_header_internals_RecordHeader_rustConstructor(
    env: JNIEnv,
    obj: JObject,
    key: jstring,
    value: jbyteArray,
) {
    let result = || -> jni::errors::Result<_> {
        let key: String = env.get_string(key.into())?.into();
        let value = if value.is_null() {
            vec![]
        } else {
            env.convert_byte_array(value).unwrap()
        };
        let record_header = Box::new(RecordHeader::new(key, value));
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
 * Class:     org_apache_kafka_common_header_internals_RecordHeader
 * Method:    rustDestructor
 * Signature: ()V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_header_internals_RecordHeader_rustDestructor(
    env: JNIEnv,
    obj: JObject,
) {
    let result = || -> jni::errors::Result<_> {
        let ptr = env.get_field(obj, "rustPointer", "J")?.j()?;
        let _obj = unsafe { Box::from_raw(ptr as *mut RecordHeader) };

        Ok(())
    }();
    match result {
        Ok(_) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!("{:?}", result),
    }
}

rust_property_getter!(
 * Struct:    RecordHeader
 * Class:     org_apache_kafka_common_header_internals_RecordHeader
 * Method:    key
 * Signature: ()Ljava/lang/String;
);

rust_property_getter!(
 * Struct:    RecordHeader
 * Class:     org_apache_kafka_common_header_internals_RecordHeader
 * Method:    value
 * Signature: ()[]B
);
