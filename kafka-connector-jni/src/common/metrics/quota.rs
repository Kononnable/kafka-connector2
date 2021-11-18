use crate::clone_to_from_java::clone_to_from_java_for_struct;
use jni::{
    objects::{JObject, JValue},
    sys::{jboolean, jdouble},
    JNIEnv,
};
use kafka_connector_macros::rust_property_getter;

#[derive(Debug, Clone)]
pub struct Quota {
    pub is_upper_bound: bool,
    pub bound: f64,
}
impl Quota {
    pub fn new(is_upper_bound: bool, bound: f64) -> Quota {
        Quota {
            is_upper_bound,
            bound,
        }
    }
    pub fn is_acceptable(&self, value: f64) -> bool {
        match self.is_upper_bound {
            true => value <= self.bound,
            false => value >= self.bound,
        }
    }
}

clone_to_from_java_for_struct!(Quota, "org/apache/kafka/common/metrics/Quota");

/*
 * Class:     org_apache_kafka_common_metrics_Quota
 * Method:    rustConstructor
 * Signature: (DZ)V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_Quota_rustConstructor(
    env: JNIEnv,
    obj: JObject,
    bound: jdouble,
    is_upper: jboolean,
) {
    let result = || -> jni::errors::Result<_> {
        let quota = Box::new(Quota::new(is_upper != 0, bound));
        let ptr = Box::into_raw(quota);
        env.set_field(obj, "rustPointer", "J", JValue::Long(ptr as i64))?;

        Ok(())
    }();
    match result {
        Ok(_) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!("{:?}", result),
    }
}

/*
 * Class:     org_apache_kafka_common_metrics_Quota
 * Method:    rustDestructor
 * Signature: ()V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_Quota_rustDestructor(
    env: JNIEnv,
    obj: JObject,
) {
    let result = || -> jni::errors::Result<_> {
        let ptr = env.get_field(obj, "rustPointer", "J")?.j()?;
        let _obj = unsafe { Box::from_raw(ptr as *mut Quota) };

        Ok(())
    }();
    match result {
        Ok(_) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!("{:?}", result),
    }
}

rust_property_getter!(
 * Struct:    Quota
 * Class:     org_apache_kafka_common_metrics_Quota
 * Method:    isUpperBound
 * Signature: ()Z
);

rust_property_getter!(
 * Struct:    Quota
 * Class:     org_apache_kafka_common_metrics_Quota
 * Method:    bound
 * Signature: ()D
);

/*
 * Class:     org_apache_kafka_common_metrics_Quota
 * Method:    acceptable
 * Signature: (D)Z
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_Quota_acceptable(
    env: jni::JNIEnv,
    obj: jni::objects::JObject,
    value: jdouble,
) -> jni::sys::jboolean {
    let result = || -> jni::errors::Result<_> {
        let ptr = env.get_field(obj, "rustPointer", "J")?.j()?;
        let rust_struct = unsafe { Box::from_raw(ptr as *mut Quota) };
        let ret = rust_struct.is_acceptable(value);
        let _ptr = Box::into_raw(rust_struct);

        Ok(ret as u8)
    };
    match result() {
        Ok(v) => v,
        Err(jni::errors::Error::JavaException) => Default::default(),
        _ => panic!(),
    }
}
