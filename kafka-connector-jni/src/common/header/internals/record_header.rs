use bytes::Bytes;
use jni::{
    objects::{JObject, JValue},
    sys::{jbyteArray, jstring},
    JNIEnv,
};

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
 * Method:    rustDeconstructor
 * Signature: ()V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_header_internals_RecordHeader_rustDeconstructor(
    env: JNIEnv,
    obj: JObject,
) {
    let result = || -> jni::errors::Result<_> {
        let ptr = env.get_field(obj, "rustPointer", "J")?.j()?;
        let _record_header = unsafe { Box::from_raw(ptr as *mut RecordHeader) };

        Ok(())
    }();
    match result {
        Ok(_) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!("{:?}", result),
    }
}

/*
 * Class:     org_apache_kafka_common_header_internals_RecordHeader
 * Method:    key
 * Signature: ()Ljava/lang/String;
 */

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_header_internals_RecordHeader_key(
    env: JNIEnv,
    obj: JObject,
) -> jstring {
    let result = || -> jni::errors::Result<_> {
        let ptr = env.get_field(obj, "rustPointer", "J")?.j()?;
        let record_header = unsafe { Box::from_raw(ptr as *mut RecordHeader) };
        let key = record_header.key.clone();
        let _ptr = Box::into_raw(record_header);
        let key = env.new_string(key)?.into_inner();
        Ok(key)
    }();
    match result {
        Ok(val) => val,
        Err(jni::errors::Error::JavaException) => JObject::null().into_inner(),
        _ => panic!("{:?}", result),
    }
}
/*
 * Class:     org_apache_kafka_common_header_internals_RecordHeader
 * Method:    value
 * Signature: ()[B
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_header_internals_RecordHeader_value(
    env: JNIEnv,
    obj: JObject,
) -> jbyteArray {
    let result = || -> jni::errors::Result<_> {
        let ptr = env.get_field(obj, "rustPointer", "J")?.j()?;
        let record_header = unsafe { Box::from_raw(ptr as *mut RecordHeader) };
        let value = record_header.value.clone();
        let _ptr = Box::into_raw(record_header);
        let values = env.byte_array_from_slice(value.as_ref())?;
        Ok(values)
    }();
    match result {
        Ok(val) => val,
        Err(jni::errors::Error::JavaException) => JObject::null().into_inner(),
        _ => panic!("{:?}", result),
    }
}
