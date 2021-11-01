use bytes::Bytes;
use jni::{
    objects::{JObject, JValue},
    sys::{jbyteArray, jobject, jstring},
    JNIEnv,
};

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

const CLASS_NAME: &str = "org/apache/kafka/common/header/internals/RecordHeader";
// May be slower, but have to clone if we want to avoid Arc<> on each structure in rust
pub fn clone_from_java(env: JNIEnv, obj: JObject) -> jni::errors::Result<RecordHeader> {
    let class = env.find_class(CLASS_NAME)?;
    if !env.is_instance_of(obj, class)? {
        env.throw_new("java/lang/Exception", "Wrong object class")?;
        // dbg!("Wrong object class");
        return Err(jni::errors::Error::JavaException);
    }
    let ptr = env.get_field(obj, "rustPointer", "J")?.j()?;
    let record_header = unsafe { Box::from_raw(ptr as *mut RecordHeader) };
    let clone = record_header.as_ref().clone();
    let _ptr = Box::into_raw(record_header);
    Ok(clone)
}
// Clones object to java - making a clone readonly in most cases(java changes won't affect real state)
// May produce errors in java (testing) logic, however needed if we want to avoid Arc<> on rust structures
pub fn clone_to_java(env: JNIEnv, header: &RecordHeader) -> jni::errors::Result<jobject> {
    let class = env.find_class(CLASS_NAME)?;
    let obj = env.alloc_object(class)?;
    let copy = Box::new(header.clone());
    let ptr = Box::into_raw(copy);
    env.set_field(obj, "rustPointer", "J", JValue::Long(ptr as i64))?;
    Ok(obj.into_inner())
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
