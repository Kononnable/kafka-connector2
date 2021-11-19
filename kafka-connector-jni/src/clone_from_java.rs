use std::{collections::HashMap, hash::Hash};

use bytes::Bytes;
use indexmap::{IndexMap, IndexSet};
use jni::{
    objects::{GlobalRef, JValue},
    JNIEnv,
};

pub trait CloneFromJava {
    /// May be slower, but have to clone if we want to avoid Arc<T> on each structure in rust
    fn clone_from_java(env: JNIEnv, obj: JValue) -> jni::errors::Result<Self>
    where
        Self: Sized;
}

macro_rules! clone_from_java {
    ($struct_name:ty, $class_name:literal) => {
        impl crate::clone_from_java::CloneFromJava for $struct_name {
            fn clone_from_java(
                env: jni::JNIEnv,
                obj: jni::objects::JValue,
            ) -> jni::errors::Result<Self> {
                let obj = obj.l()?;
                let class = env.find_class($class_name)?;
                if !env.is_instance_of(obj, class)? {
                    env.throw_new("java/lang/Exception", "Wrong object class")?;
                    return Err(jni::errors::Error::JavaException);
                }
                let ptr = env.get_field(obj, "rustPointer", "J")?.j()?;
                let this = unsafe { Box::from_raw(ptr as *mut Self) };
                let clone = this.as_ref().clone();
                let _ptr = Box::into_raw(this);
                Ok(clone)
            }
        }
    };
}

impl CloneFromJava for String {
    fn clone_from_java(env: JNIEnv, obj: JValue) -> jni::errors::Result<Self>
    where
        Self: Sized,
    {
        let obj = obj.l()?;
        if obj.is_null() {
            return Ok("".to_owned());
        }
        env.get_string(obj.into()).map(Into::into)
    }
}

impl CloneFromJava for i32 {
    fn clone_from_java(_env: JNIEnv, obj: JValue) -> jni::errors::Result<Self>
    where
        Self: Sized,
    {
        obj.i()
    }
}
impl CloneFromJava for u32 {
    fn clone_from_java(_env: JNIEnv, obj: JValue) -> jni::errors::Result<Self>
    where
        Self: Sized,
    {
        Ok(obj.i()? as u32)
    }
}

impl CloneFromJava for i64 {
    fn clone_from_java(_env: JNIEnv, obj: JValue) -> jni::errors::Result<Self>
    where
        Self: Sized,
    {
        obj.j()
    }
}
impl CloneFromJava for u64 {
    fn clone_from_java(_env: JNIEnv, obj: JValue) -> jni::errors::Result<Self>
    where
        Self: Sized,
    {
        Ok(obj.j()? as u64)
    }
}
impl CloneFromJava for u128 {
    fn clone_from_java(_env: JNIEnv, obj: JValue) -> jni::errors::Result<Self>
    where
        Self: Sized,
    {
        Ok(obj.j()? as u128)
    }
}

impl CloneFromJava for bool {
    fn clone_from_java(_env: JNIEnv, obj: JValue) -> jni::errors::Result<Self>
    where
        Self: Sized,
    {
        obj.z()
    }
}

impl CloneFromJava for f64 {
    fn clone_from_java(_env: JNIEnv, obj: JValue) -> jni::errors::Result<Self>
    where
        Self: Sized,
    {
        obj.d()
    }
}

impl CloneFromJava for Option<i32> {
    fn clone_from_java(env: JNIEnv, obj: JValue) -> jni::errors::Result<Self>
    where
        Self: Sized,
    {
        let obj = obj.l()?;
        let is_present = env.call_method(obj, "isPresent", "()Z", &[])?.z()?;
        match is_present {
            true => {
                let integer_obj = env
                    .call_method(obj, "get", "()Ljava/lang/Object;", &[])?
                    .l()?;
                let value = env.call_method(integer_obj, "intValue", "()I", &[])?.i()?;
                Ok(Some(value))
            }
            false => Ok(None),
        }
    }
}

impl CloneFromJava for GlobalRef {
    fn clone_from_java(env: JNIEnv, obj: JValue) -> jni::errors::Result<Self>
    where
        Self: Sized,
    {
        let obj = obj.l()?;
        env.new_global_ref(obj)
    }
}

impl CloneFromJava for Bytes {
    fn clone_from_java(env: JNIEnv, obj: JValue) -> jni::errors::Result<Self>
    where
        Self: Sized,
    {
        let o = obj.l()?.into_inner();
        env.convert_byte_array(o).map(Bytes::from)
    }
}

impl<K, V> CloneFromJava for HashMap<K, V>
where
    K: CloneFromJava + Eq + Hash,
    V: CloneFromJava,
{
    fn clone_from_java(env: JNIEnv, obj: JValue) -> jni::errors::Result<Self>
    where
        Self: Sized,
    {
        let mut hash_map = HashMap::new();
        let entry_set = env
            .call_method(obj.l()?, "entrySet", "()Ljava/util/Set;", &[])?
            .l()?;
        let array = env
            .call_method(entry_set, "toArray", "()[Ljava/lang/Object;", &[])?
            .l()?
            .into_inner();
        let length = env.get_array_length(array)?;
        for i in 0..length {
            let entry = env.get_object_array_element(array, i)?;
            let key = env.call_method(entry, "getKey", "()Ljava/lang/Object;", &[])?;
            let key = K::clone_from_java(env, key)?;
            let value = env.call_method(entry, "getValue", "()Ljava/lang/Object;", &[])?;
            let value = V::clone_from_java(env, value)?;
            hash_map.insert(key, value);
        }

        Ok(hash_map)
    }
}

impl<K, V> CloneFromJava for IndexMap<K, V>
where
    K: CloneFromJava + Eq + Hash,
    V: CloneFromJava,
{
    fn clone_from_java(env: JNIEnv, obj: JValue) -> jni::errors::Result<Self>
    where
        Self: Sized,
    {
        let mut hash_map = IndexMap::new();
        let entry_set = env
            .call_method(obj.l()?, "entrySet", "()Ljava/util/Set;", &[])?
            .l()?;
        let array = env
            .call_method(entry_set, "toArray", "()[Ljava/lang/Object;", &[])?
            .l()?
            .into_inner();
        let length = env.get_array_length(array)?;
        for i in 0..length {
            let entry = env.get_object_array_element(array, i)?;
            let key = env.call_method(entry, "getKey", "()Ljava/lang/Object;", &[])?;
            let key = K::clone_from_java(env, key)?;
            let value = env.call_method(entry, "getValue", "()Ljava/lang/Object;", &[])?;
            let value = V::clone_from_java(env, value)?;
            hash_map.insert(key, value);
        }

        Ok(hash_map)
    }
}

impl<K> CloneFromJava for IndexSet<K>
where
    K: CloneFromJava + Eq + Hash,
{
    fn clone_from_java(env: JNIEnv, obj: JValue) -> jni::errors::Result<Self>
    where
        Self: Sized,
    {
        let mut hash_set = IndexSet::new();

        let array = env
            .call_method(obj.l()?, "toArray", "()[Ljava/lang/Object;", &[])?
            .l()?
            .into_inner();
        let length = env.get_array_length(array)?;
        for i in 0..length {
            let key = env.get_object_array_element(array, i)?;
            let key = K::clone_from_java(env, JValue::Object(key))?;
            hash_set.insert(key);
        }

        Ok(hash_set)
    }
}

impl<T> CloneFromJava for Vec<T>
where
    T: CloneFromJava,
{
    fn clone_from_java(env: JNIEnv, obj: JValue) -> jni::errors::Result<Self>
    where
        Self: Sized,
    {
        let mut vec = Vec::new();

        let array = env
            .call_method(obj.l()?, "toArray", "()[Ljava/lang/Object;", &[])?
            .l()?
            .into_inner();
        let length = env.get_array_length(array)?;
        for i in 0..length {
            let key = env.get_object_array_element(array, i)?;
            let item = T::clone_from_java(env, JValue::Object(key))?;
            vec.push(item);
        }

        Ok(vec)
    }
}
