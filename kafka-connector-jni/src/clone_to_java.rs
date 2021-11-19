use std::{collections::HashMap, hash::Hash};

use bytes::Bytes;
use indexmap::{IndexMap, IndexSet};
use jni::{
    objects::{GlobalRef, JValue},
    JNIEnv,
};
pub trait CloneToJava {
    /// Clones object to java - making a clone readonly in most cases(java changes won't affect a real state)
    /// May produce errors in java (testing) logic, however needed if we want to avoid Arc<T> on rust structures
    fn clone_to_java<'a>(&self, env: JNIEnv<'a>) -> jni::errors::Result<JValue<'a>>;
}

macro_rules! clone_to_java {
    ($struct_name:ty, $class_name:literal) => {
        impl crate::clone_to_java::CloneToJava for $struct_name {
            fn clone_to_java<'a>(
                &self,
                env: jni::JNIEnv<'a>,
            ) -> jni::errors::Result<jni::objects::JValue<'a>> {
                let class = env.find_class($class_name)?;
                let obj = env.alloc_object(class)?;
                let copy = Box::new(self.clone());
                let ptr = Box::into_raw(copy);
                env.set_field(
                    obj,
                    "rustPointer",
                    "J",
                    jni::objects::JValue::Long(ptr as i64),
                )?;
                Ok(obj.into())
            }
        }
    };
}

impl CloneToJava for String {
    fn clone_to_java<'a>(&self, env: JNIEnv<'a>) -> jni::errors::Result<JValue<'a>> {
        env.new_string(self).map(Into::into).map(JValue::Object)
    }
}
impl CloneToJava for i32 {
    fn clone_to_java<'a>(&self, _env: JNIEnv<'a>) -> jni::errors::Result<JValue<'a>> {
        Ok(JValue::Int(*self))
    }
}
impl CloneToJava for u32 {
    fn clone_to_java<'a>(&self, _env: JNIEnv<'a>) -> jni::errors::Result<JValue<'a>> {
        Ok(match *self {
            u32::MAX => JValue::Int(i32::MAX),
            u32::MIN => JValue::Int(i32::MIN),
            v => JValue::Int(v as i32),
        })
    }
}
impl CloneToJava for i64 {
    fn clone_to_java<'a>(&self, _env: JNIEnv<'a>) -> jni::errors::Result<JValue<'a>> {
        Ok(JValue::Long(*self))
    }
}
impl CloneToJava for u64 {
    fn clone_to_java<'a>(&self, _env: JNIEnv<'a>) -> jni::errors::Result<JValue<'a>> {
        Ok(match *self {
            u64::MAX => JValue::Long(i64::MAX),
            u64::MIN => JValue::Long(i64::MIN),
            v => JValue::Long(v as i64),
        })
    }
}
impl CloneToJava for u128 {
    fn clone_to_java<'a>(&self, _env: JNIEnv<'a>) -> jni::errors::Result<JValue<'a>> {
        Ok(match *self {
            u128::MAX => JValue::Long(i64::MAX),
            u128::MIN => JValue::Long(i64::MIN),
            v => JValue::Long(v as i64),
        })
    }
}

impl CloneToJava for bool {
    fn clone_to_java<'a>(&self, _env: JNIEnv<'a>) -> jni::errors::Result<JValue<'a>> {
        Ok(JValue::Bool((*self) as u8))
    }
}
impl CloneToJava for f64 {
    fn clone_to_java<'a>(&self, _env: JNIEnv<'a>) -> jni::errors::Result<JValue<'a>> {
        Ok(JValue::Double(*self))
    }
}

impl CloneToJava for Option<i32> {
    fn clone_to_java<'a>(&self, env: JNIEnv<'a>) -> jni::errors::Result<JValue<'a>> {
        let optional_class = env
            .find_class("java/util/Optional")
            .expect("Failed to load the target class");
        let value = match self {
            Some(value) => {
                let integer_class = env
                    .find_class("java/lang/Integer")
                    .expect("Failed to load the target class");

                let obj = env.new_object(integer_class, "(I)V", &[JValue::Int(*value)])?;
                env.call_static_method(
                    optional_class,
                    "of",
                    "(Ljava/lang/Object;)Ljava/util/Optional;",
                    &[obj.into()],
                )?
            }
            None => {
                env.call_static_method(optional_class, "empty", "()Ljava/util/Optional;", &[])?
            }
        };
        Ok(value)
    }
}

impl CloneToJava for GlobalRef {
    fn clone_to_java<'a>(&self, _env: JNIEnv<'a>) -> jni::errors::Result<JValue<'a>> {
        let o = self.clone().as_obj().into_inner();
        Ok(JValue::Object(o.into()))
    }
}
impl CloneToJava for Bytes {
    fn clone_to_java<'a>(&self, env: JNIEnv<'a>) -> jni::errors::Result<JValue<'a>> {
        let o = env.byte_array_from_slice(self)?;
        Ok(JValue::Object(o.into()))
    }
}

impl<K, V> CloneToJava for HashMap<K, V>
where
    K: CloneToJava + Eq + Hash,
    V: CloneToJava,
{
    fn clone_to_java<'a>(&self, env: JNIEnv<'a>) -> jni::errors::Result<JValue<'a>> {
        let class = env.find_class("java/util/HashMap")?;
        let hash_map = env.new_object(class, "()V", &[])?;
        for entry in self {
            let key = K::clone_to_java(entry.0, env)?;
            let value = V::clone_to_java(entry.1, env)?;
            env.call_method(
                hash_map,
                "put",
                "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
                &[key, value],
            )?;
        }

        Ok(JValue::Object(hash_map))
    }
}

impl<K, V> CloneToJava for IndexMap<K, V>
where
    K: CloneToJava + Eq + Hash,
    V: CloneToJava,
{
    fn clone_to_java<'a>(&self, env: JNIEnv<'a>) -> jni::errors::Result<JValue<'a>> {
        let class = env.find_class("java/util/LinkedHashMap")?;
        let hash_map = env.new_object(class, "()V", &[])?;
        for entry in self {
            let key = K::clone_to_java(entry.0, env)?;
            let value = V::clone_to_java(entry.1, env)?;
            env.call_method(
                hash_map,
                "put",
                "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
                &[key, value],
            )?;
        }

        Ok(JValue::Object(hash_map))
    }
}

impl<K> CloneToJava for IndexSet<K>
where
    K: CloneToJava + Eq + Hash,
{
    fn clone_to_java<'a>(&self, env: JNIEnv<'a>) -> jni::errors::Result<JValue<'a>> {
        let class = env.find_class("java/util/LinkedHashSet")?;
        let hash_set = env.new_object(class, "()V", &[])?;
        for key in self {
            let key = K::clone_to_java(key, env)?;
            env.call_method(hash_set, "add", "(Ljava/lang/Object;)Z", &[key])?;
        }

        Ok(JValue::Object(hash_set))
    }
}

impl<T> CloneToJava for Vec<T>
where
    T: CloneToJava,
{
    fn clone_to_java<'a>(&self, env: JNIEnv<'a>) -> jni::errors::Result<JValue<'a>> {
        let class = env.find_class("java/util/List")?;
        let list = env.new_object(class, "()V", &[])?;
        for item in self {
            let key = T::clone_to_java(item, env)?;
            env.call_method(list, "add", "(Ljava/lang/Object;)Z", &[key])?;
        }

        Ok(JValue::Object(list))
    }
}
