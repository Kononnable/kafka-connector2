use std::panic;

use jni::{
    objects::{JObject, JValue},
    sys::{jbyteArray, jobject, jstring},
    JNIEnv,
};

/*
 * Class:     org_apache_kafka_common_header_internals_RecordHeaders
 * Method:    add
 * Signature: (Lorg/apache/kafka/common/header/Header;)Lorg/apache/kafka/common/header/Headers;
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_header_internals_RecordHeaders_add__Lorg_apache_kafka_common_header_Header_2(
    env: JNIEnv,
    obj: JObject,
    header: JObject,
) -> jobject {
    let result = || -> jni::errors::Result<()> {
        let class = env
            .find_class("java/util/Objects")
            .expect("Failed to load the target class");

        let error_msg = env.new_string("Header cannot be null.").unwrap();

        env.call_static_method(
            class,
            "requireNonNull",
            "(Ljava/lang/Object;Ljava/lang/String;)Ljava/lang/Object;",
            &[JValue::Object(header), JValue::Object(error_msg.into())],
        )?;

        env.call_method(obj, "canWrite", "()V", &[])?;

        let headers = env.get_field(obj, "headers", "Ljava/util/List;")?.l()?;
        env.call_method(
            headers,
            "add",
            "(Ljava/lang/Object;)Z",
            &[JValue::Object(header)],
        )?;
        Ok(())
    }();

    match result {
        Ok(()) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!(),
    }

    obj.into_inner()
}

/*
 * Class:     org_apache_kafka_common_header_internals_RecordHeaders
 * Method:    add
 * Signature: (Ljava/lang/String;[B)Lorg/apache/kafka/common/header/Headers;
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_header_internals_RecordHeaders_add__Ljava_lang_String_2_3B(
    env: JNIEnv,
    obj: JObject,
    key: jstring,
    value: jbyteArray,
) -> jobject {
    let result = || -> jni::errors::Result<()> {
        let class = env
            .find_class("org/apache/kafka/common/header/internals/RecordHeader")
            .expect("Failed to load the target class");

        let header = env.new_object(
            class,
            "(Ljava/lang/String;[B)V",
            &[JValue::Object(key.into()), JValue::Object(value.into())],
        )?;

        env.call_method(
            obj,
            "add",
            "(Lorg/apache/kafka/common/header/Header;)Lorg/apache/kafka/common/header/Headers;",
            &[JValue::Object(header)],
        )?;
        Ok(())
    }();

    match result {
        Ok(()) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!(),
    }

    obj.into_inner()
}

/*
 * Class:     org_apache_kafka_common_header_internals_RecordHeaders
 * Method:    remove
 * Signature: (Ljava/lang/String;)Lorg/apache/kafka/common/header/Headers;
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_header_internals_RecordHeaders_remove(
    env: JNIEnv,
    obj: JObject,
    key: jstring,
) -> jobject {
    let result = || -> jni::errors::Result<()> {
        env.call_method(obj, "canWrite", "()V", &[])?;
        env.call_method(
            obj,
            "checkKey",
            "(Ljava/lang/String;)V",
            &[JValue::Object(key.into())],
        )?;
        let iterator = env
            .call_method(obj, "iterator", "()Ljava/util/Iterator;", &[])?
            .l()?;

        while env.call_method(iterator, "hasNext", "()Z", &[])?.z()? {
            let current_value = env
                .call_method(iterator, "next", "()Ljava/lang/Object;", &[])?
                .l()?;
            let current_key = env
                .call_method(current_value, "key", "()Ljava/lang/String;", &[])?
                .l()?;
            let is_equal = env
                .call_method(
                    current_key,
                    "equals",
                    "(Ljava/lang/Object;)Z",
                    &[JValue::Object(key.into())],
                )?
                .z()?;
            if is_equal {
                env.call_method(iterator, "remove", "()V", &[])?;
            }
        }
        Ok(())
    }();
    match result {
        Ok(()) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!(),
    }

    obj.into_inner()
}

/*
 * Class:     org_apache_kafka_common_header_internals_RecordHeaders
 * Method:    lastHeader
 * Signature: (Ljava/lang/String;)Lorg/apache/kafka/common/header/Header;
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_header_internals_RecordHeaders_lastHeader(
    env: JNIEnv,
    obj: JObject,
    key: jstring,
) -> jobject {
    let result = || -> jni::errors::Result<_> {
        env.call_method(
            obj,
            "checkKey",
            "(Ljava/lang/String;)V",
            &[JValue::Object(key.into())],
        )?;
        let headers = env.get_field(obj, "headers", "Ljava/util/List;")?.l()?;
        let size = env.call_method(headers, "size", "()I", &[])?.i()?;
        for i in (0..size).rev() {
            let header = env
                .call_method(headers, "get", "(I)Ljava/lang/Object;", &[JValue::Int(i)])?
                .l()?;
            let current_key = env
                .call_method(header, "key", "()Ljava/lang/String;", &[])?
                .l()?;
            let is_equal = env
                .call_method(
                    current_key,
                    "equals",
                    "(Ljava/lang/Object;)Z",
                    &[JValue::Object(key.into())],
                )?
                .z()?;
            if is_equal {
                return Ok(header);
            }
        }
        Ok(JObject::null())
    }();
    match result {
        Ok(val) => val.into_inner(),
        Err(jni::errors::Error::JavaException) => JObject::null().into_inner(),
        _ => panic!("{:?}", result),
    }
}

/*
 * Class:     org_apache_kafka_common_header_internals_RecordHeaders
 * Method:    headers
 * Signature: (Ljava/lang/String;)Ljava/lang/Iterable;
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_header_internals_RecordHeaders_headers(
    env: JNIEnv,
    obj: JObject,
    key: jstring,
) -> jobject {
    let result = || -> jni::errors::Result<_> {
        env.call_method(
            obj,
            "checkKey",
            "(Ljava/lang/String;)V",
            &[JValue::Object(key.into())],
        )?;

        let iterator = env
            .call_method(obj, "iterator", "()Ljava/util/Iterator;", &[])?
            .l()?;

        let iterator_class = env
            .find_class(
                "org/apache/kafka/common/header/internals/RecordHeaders$FilterByKeyIterator",
            )
            .expect("Failed to load the target class");

        let filter_iterator = env.new_object(
            iterator_class,
            "(Ljava/util/Iterator;Ljava/lang/String;)V",
            &[iterator.into(), key.into()],
        )?;

        let rust_lib_class = env
            .find_class("org/apache/kafka/RustLib")
            .expect("Failed to load the target class");
        let iterable = env
            .call_static_method(
                rust_lib_class,
                "iteratorToIterable",
                "(Ljava/util/Iterator;)Ljava/lang/Iterable;",
                &[filter_iterator.into()],
            )?
            .l()?;

        Ok(iterable)
    }();
    match result {
        Ok(val) => val.into_inner(),
        Err(jni::errors::Error::JavaException) => JObject::null().into_inner(),
        _ => panic!("{:?}", result),
    }
}

/*
 * Class:     org_apache_kafka_common_header_internals_RecordHeaders
 * Method:    iterator
 * Signature: ()Ljava/util/Iterator;
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_header_internals_RecordHeaders_iterator(
    env: JNIEnv,
    obj: JObject,
) -> jobject {
    let result = || -> jni::errors::Result<_> {
        let headers = env.get_field(obj, "headers", "Ljava/util/List;")?.l()?;
        let iterator = env
            .call_method(headers, "iterator", "()Ljava/util/Iterator;", &[])?
            .l()?;
        let close_aware = env
            .call_method(
                obj,
                "closeAware",
                "(Ljava/util/Iterator;)Ljava/util/Iterator;",
                &[iterator.into()],
            )?
            .l()?;
        Ok(close_aware)
    }();
    match result {
        Ok(val) => val.into_inner(),
        Err(jni::errors::Error::JavaException) => JObject::null().into_inner(),
        _ => panic!("{:?}", result),
    }
}

/*
 * Class:     org_apache_kafka_common_header_internals_RecordHeaders
 * Method:    setReadOnly
 * Signature: ()V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_header_internals_RecordHeaders_setReadOnly(
    env: JNIEnv,
    obj: JObject,
) {
    env.set_field(obj, "isReadOnly", "Z", true.into()).unwrap();
}

/*
 * Class:     org_apache_kafka_common_header_internals_RecordHeaders
 * Method:    toArray
 * Signature: ()[Lorg/apache/kafka/common/header/Header;
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_header_internals_RecordHeaders_toArray(
    env: JNIEnv,
    obj: JObject,
) -> jobject {
    let result = || -> jni::errors::Result<_> {
        let headers = env.get_field(obj, "headers", "Ljava/util/List;")?.l()?;
        let isEmpty = env.call_method(headers, "isEmpty", "()Z", &[])?.z()?;
        if isEmpty {
            let record = env.find_class("org/apache/kafka/common/record/Record")?;
            let ret_val = env
                .get_static_field(
                    record,
                    "EMPTY_HEADERS",
                    "[Lorg/apache/kafka/common/header/Header;",
                )?
                .l()?;
            Ok(ret_val)
        } else {
            let ret_val = env
                .call_method(headers, "toArray", "()[Ljava/lang/Object;", &[])?
                .l()?;
            Ok(ret_val)
        }
    }();
    match result {
        Ok(val) => val.into_inner(),
        Err(jni::errors::Error::JavaException) => JObject::null().into_inner(),
        _ => panic!("{:?}", result),
    }
}
/*
 * Class:     org_apache_kafka_common_header_internals_RecordHeaders
 * Method:    checkKey
 * Signature: (Ljava/lang/String;)V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_header_internals_RecordHeaders_checkKey(
    env: JNIEnv,
    _obj: JObject,
    key: jstring,
) {
    let result = || -> jni::errors::Result<_> {
        if key.is_null() {
            env.throw_new("java/lang/IllegalArgumentException", "key cannot be null.")?;
        };
        Ok(())
    }();
    match result {
        Ok(_) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!("{:?}", result),
    }
}

/*
 * Class:     org_apache_kafka_common_header_internals_RecordHeaders
 * Method:    canWrite
 * Signature: ()V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_header_internals_RecordHeaders_canWrite(
    env: JNIEnv,
    obj: JObject,
) {
    let result = || -> jni::errors::Result<_> {
        let is_readonly = env.get_field(obj, "isReadOnly", "Z")?.z()?;

        if is_readonly {
            env.throw_new(
                "java/lang/IllegalStateException",
                "RecordHeaders has been closed.",
            )?;
        };
        Ok(())
    }();
    match result {
        Ok(_) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!("{:?}", result),
    }
}
