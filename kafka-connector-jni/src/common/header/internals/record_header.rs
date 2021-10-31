use jni::{
    objects::JObject,
    sys::{jbyteArray, jstring},
    JNIEnv,
};

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
        let mut key = env.get_field(obj, "key", "Ljava/lang/String;")?.l()?;
        if key.is_null() {
            let key_buffer = env
                .get_field(obj, "keyBuffer", "Ljava/nio/ByteBuffer;")?
                .l()?;
            let remaining = env.call_method(key_buffer, "remaining", "()I", &[])?;
            let utils = env.find_class("org/apache/kafka/common/utils/Utils")?;
            let value = env.call_static_method(
                utils,
                "utf8",
                "(Ljava/nio/ByteBuffer;I)Ljava/lang/String;",
                &[key_buffer.into(), remaining],
            )?;
            env.set_field(obj, "key", "Ljava/lang/String;", value)?;
            env.set_field(
                obj,
                "keyBuffer",
                "Ljava/nio/ByteBuffer;",
                JObject::null().into(),
            )?;
            key = value.l()?;
        }

        Ok(key)
    }();
    match result {
        Ok(val) => val.into_inner(),
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
        let mut value = env.get_field(obj, "value", "[B")?.l()?;
        let value_buffer = env
            .get_field(obj, "valueBuffer", "Ljava/nio/ByteBuffer;")?
            .l()?;
        if value.is_null() && !value_buffer.is_null() {
            let utils = env.find_class("org/apache/kafka/common/utils/Utils")?;
            let new_value = env.call_static_method(
                utils,
                "toArray",
                "(Ljava/nio/ByteBuffer;)[B",
                &[value_buffer.into()],
            )?;
            env.set_field(obj, "value", "[B", new_value)?;
            value = new_value.l()?;
        }

        Ok(value)
    }();
    match result {
        Ok(val) => val.into_inner(),
        Err(jni::errors::Error::JavaException) => JObject::null().into_inner(),
        _ => panic!("{:?}", result),
    }
}
