use std::ops::Deref;

use jni::{objects::JObject, JNIEnv};

macro_rules! from_jobject {
    ($struct_name:ty, $class_name:literal) => {
        impl crate::java_stored_object::FromJObject for $struct_name {
            fn from_jobject(
                env: jni::JNIEnv,
                obj: jni::objects::JObject,
            ) -> jni::errors::Result<crate::java_stored_object::JavaStoredObject<Self>> {
                crate::java_stored_object::JavaStoredObject::new(env, obj, $class_name)
            }
        }
    };
}

pub trait FromJObject {
    fn from_jobject(env: JNIEnv, obj: JObject) -> jni::errors::Result<JavaStoredObject<Self>>
    where
        Self: Sized;
}

pub struct JavaStoredObject<T> {
    pub obj: Option<Box<T>>,
}

impl<T> JavaStoredObject<T> {
    pub fn new(
        env: JNIEnv,
        obj: JObject,
        class_name: &str,
    ) -> jni::errors::Result<JavaStoredObject<T>> {
        if !class_name.is_empty() {
            let class = env.find_class(class_name)?;
            if !env.is_instance_of(obj, class)? {
                env.throw_new("java/lang/Exception", "Wrong object class")?;
                return Err(jni::errors::Error::JavaException);
            }
        }
        let ptr = env.get_field(obj, "rustPointer", "J")?.j()?;
        let this = unsafe { Box::from_raw(ptr as *mut T) };
        Ok(JavaStoredObject { obj: Some(this) })
    }
    pub fn modify<F, R>(&mut self, mut func: F) -> R
    where
        F: FnMut(&mut T) -> R,
    {
        func(&mut self.obj.as_mut().unwrap())
    }
}
impl<T> Drop for JavaStoredObject<T> {
    fn drop(&mut self) {
        let _ptr = Box::into_raw(self.obj.take().unwrap());
    }
}

impl<T> Deref for JavaStoredObject<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.obj.as_ref().unwrap()
    }
}
