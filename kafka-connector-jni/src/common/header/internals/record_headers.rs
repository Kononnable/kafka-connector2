use crate::{clone_to_from_java_obj, CloneToFromJava};
use std::{
    ops::{Deref, DerefMut},
    panic,
};

use jni::{
    objects::{JObject, JValue},
    sys::{jbyteArray, jobject, jstring},
    JNIEnv,
};

use super::record_header::RecordHeader;

#[derive(Default, Clone)]
pub struct RecordHeaders(Vec<RecordHeader>);
impl Deref for RecordHeaders {
    type Target = Vec<RecordHeader>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RecordHeaders {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

clone_to_from_java_obj!(
    RecordHeaders,
    "org/apache/kafka/common/header/internals/RecordHeaders"
);

/*
 * Class:     org_apache_kafka_common_header_internals_RecordHeaders
 * Method:    rustConstructor
 * Signature: ()V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_header_internals_RecordHeaders_rustConstructor(
    env: JNIEnv,
    obj: JObject,
) {
    let result = || -> jni::errors::Result<_> {
        let record_headers = Box::new(RecordHeaders::default());
        let ptr = Box::into_raw(record_headers);
        env.set_field(obj, "rustPointer", "J", JValue::Long(ptr as i64))?;

        Ok(())
    }();
    match result {
        Ok(_) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!("{:?}", result),
    }
}
/*
 * Class:     org_apache_kafka_common_header_internals_RecordHeaders
 * Method:    rustDeconstructor
 * Signature: ()V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_header_internals_RecordHeaders_rustDeconstructor(
    env: JNIEnv,
    obj: JObject,
) {
    let result = || -> jni::errors::Result<_> {
        let ptr = env.get_field(obj, "rustPointer", "J")?.j()?;
        let _record_headers = unsafe { Box::from_raw(ptr as *mut RecordHeaders) };

        Ok(())
    }();
    match result {
        Ok(_) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!("{:?}", result),
    }
}

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

        let header = RecordHeader::clone_from_java(env, header.into())?;

        let ptr = env.get_field(obj, "rustPointer", "J")?.j()?;
        let mut record_headers = unsafe { Box::from_raw(ptr as *mut RecordHeaders) };
        record_headers.push(header);
        let _ptr = Box::into_raw(record_headers);
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
        let key: String = env.get_string(key.into())?.into();
        let value = if value.is_null() {
            vec![]
        } else {
            env.convert_byte_array(value).unwrap()
        };
        let header = RecordHeader::new(key, value);

        let ptr = env.get_field(obj, "rustPointer", "J")?.j()?;
        let mut record_headers = unsafe { Box::from_raw(ptr as *mut RecordHeaders) };
        record_headers.push(header);
        let _ptr = Box::into_raw(record_headers);

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
        env.call_method(
            obj,
            "checkKey",
            "(Ljava/lang/String;)V",
            &[JValue::Object(key.into())],
        )?;
        let key: String = env.get_string(key.into())?.into();

        let ptr = env.get_field(obj, "rustPointer", "J")?.j()?;
        let record_headers = unsafe { Box::from_raw(ptr as *mut RecordHeaders) };
        let filtered = record_headers
            .0
            .into_iter()
            .filter(|header| header.key != key)
            .collect::<Vec<_>>();
        let filtered = RecordHeaders(filtered);
        let ptr = Box::into_raw(Box::new(filtered));
        env.set_field(obj, "rustPointer", "J", JValue::Long(ptr as i64))?;

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

        let key: String = env.get_string(key.into())?.into();
        let ptr = env.get_field(obj, "rustPointer", "J")?.j()?;
        let record_headers = unsafe { Box::from_raw(ptr as *mut RecordHeaders) };
        let result = record_headers
            .iter()
            .rev()
            .find(|x| x.key == key)
            .map(|header| header.clone_to_java(env))
            .transpose()?
            .map(JValue::l)
            .transpose()?;
        let _ptr = Box::into_raw(record_headers);

        Ok(result.unwrap_or_else(JObject::null))
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

        let key: String = env.get_string(key.into())?.into();
        let ptr = env.get_field(obj, "rustPointer", "J")?.j()?;
        let record_headers = unsafe { Box::from_raw(ptr as *mut RecordHeaders) };
        let result = record_headers
            .iter()
            .filter(|x| x.key == key)
            .map(|header| header.clone_to_java(env))
            .collect::<jni::errors::Result<Vec<_>>>()?
            .into_iter()
            .map(JValue::l)
            .collect::<jni::errors::Result<Vec<_>>>()?;
        let result = result.into_iter().map(Into::into).collect::<Vec<JObject>>();
        let _ptr = Box::into_raw(record_headers);

        let array_list_class = env.find_class("java/util/ArrayList")?;
        let array = env.new_object(array_list_class, "(I)V", &[(result.len() as i32).into()])?;
        for object in result {
            env.call_method(array, "add", "(Ljava/lang/Object;)Z", &[object.into()])?;
        }

        Ok(array)
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
        let ptr = env.get_field(obj, "rustPointer", "J")?.j()?;
        let record_headers = unsafe { Box::from_raw(ptr as *mut RecordHeaders) };
        let result = record_headers
            .iter()
            .map(|header| header.clone_to_java(env))
            .collect::<jni::errors::Result<Vec<_>>>()?
            .into_iter()
            .map(JValue::l)
            .collect::<jni::errors::Result<Vec<_>>>()?;
        let result = result.into_iter().map(Into::into).collect::<Vec<JObject>>();
        let _ptr = Box::into_raw(record_headers);

        let array_list_class = env.find_class("java/util/ArrayList")?;
        let array = env.new_object(array_list_class, "(I)V", &[(result.len() as i32).into()])?;
        for object in result {
            env.call_method(array, "add", "(Ljava/lang/Object;)Z", &[object.into()])?;
        }
        let iterator = env
            .call_method(array, "iterator", "()Ljava/util/Iterator;", &[])?
            .l()?;

        Ok(iterator)
    }();
    match result {
        Ok(val) => val.into_inner(),
        Err(jni::errors::Error::JavaException) => JObject::null().into_inner(),
        _ => panic!("{:?}", result),
    }
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
        let ptr = env.get_field(obj, "rustPointer", "J")?.j()?;
        let record_headers = unsafe { Box::from_raw(ptr as *mut RecordHeaders) };
        let result = record_headers
            .iter()
            .map(|header| header.clone_to_java(env))
            .collect::<jni::errors::Result<Vec<_>>>()?
            .into_iter()
            .map(JValue::l)
            .collect::<jni::errors::Result<Vec<_>>>()?;
        let result = result.into_iter().map(Into::into).collect::<Vec<JObject>>();
        let _ptr = Box::into_raw(record_headers);

        let element_class = env.find_class("org/apache/kafka/common/header/Header")?;
        let array = env.new_object_array(result.len() as i32, element_class, JObject::null())?;
        for object in result.into_iter().enumerate() {
            env.set_object_array_element(array, object.0 as i32, object.1)?;
        }

        Ok(array)
    }();
    match result {
        Ok(val) => val,
        Err(jni::errors::Error::JavaException) => JObject::null().into_inner(),
        _ => panic!("{:?}", result),
    }
}
