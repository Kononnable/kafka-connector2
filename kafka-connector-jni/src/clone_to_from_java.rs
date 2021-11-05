use std::{collections::HashMap, hash::Hash};

use bytes::Bytes;
use indexmap::{IndexMap, IndexSet};
use jni::{
    objects::{GlobalRef, JValue},
    JNIEnv,
};

pub trait CloneToFromJava {
    /// Clones object to java - making a clone readonly in most cases(java changes won't affect a real state)
    /// May produce errors in java (testing) logic, however needed if we want to avoid Arc<T> on rust structures
    fn clone_to_java<'a>(&self, env: JNIEnv<'a>) -> jni::errors::Result<JValue<'a>>;

    /// May be slower, but have to clone if we want to avoid Arc<T> on each structure in rust
    fn clone_from_java(env: JNIEnv, obj: JValue) -> jni::errors::Result<Self>
    where
        Self: Sized;
}

macro_rules! clone_to_from_java_for_struct {
    ($struct_name:ident, $class_name:literal) => {
        impl crate::clone_to_from_java::CloneToFromJava for $struct_name {
            fn clone_to_java<'a>(&self, env: JNIEnv<'a>) -> jni::errors::Result<JValue<'a>> {
                let class = env.find_class($class_name)?;
                let obj = env.alloc_object(class)?;
                let copy = Box::new(self.clone());
                let ptr = Box::into_raw(copy);
                env.set_field(obj, "rustPointer", "J", JValue::Long(ptr as i64))?;
                Ok(obj.into())
            }
            fn clone_from_java(env: JNIEnv, obj: JValue) -> jni::errors::Result<Self> {
                let obj = obj.l()?;
                let class = env.find_class($class_name)?;
                if !env.is_instance_of(obj, class)? {
                    env.throw_new("java/lang/Exception", "Wrong object class")?;
                    return Err(jni::errors::Error::JavaException);
                }
                let ptr = env.get_field(obj, "rustPointer", "J")?.j()?;
                let record_header = unsafe { Box::from_raw(ptr as *mut Self) };
                let clone = record_header.as_ref().clone();
                let _ptr = Box::into_raw(record_header);
                Ok(clone)
            }
        }
    };
}
pub(crate) use clone_to_from_java_for_struct;

impl CloneToFromJava for String {
    fn clone_to_java<'a>(&self, env: JNIEnv<'a>) -> jni::errors::Result<JValue<'a>> {
        env.new_string(self).map(Into::into).map(JValue::Object)
    }

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

impl CloneToFromJava for i32 {
    fn clone_to_java<'a>(&self, _env: JNIEnv<'a>) -> jni::errors::Result<JValue<'a>> {
        Ok(JValue::Int(*self))
    }

    fn clone_from_java(_env: JNIEnv, obj: JValue) -> jni::errors::Result<Self>
    where
        Self: Sized,
    {
        obj.i()
    }
}

impl CloneToFromJava for i64 {
    fn clone_to_java<'a>(&self, _env: JNIEnv<'a>) -> jni::errors::Result<JValue<'a>> {
        Ok(JValue::Long(*self))
    }

    fn clone_from_java(_env: JNIEnv, obj: JValue) -> jni::errors::Result<Self>
    where
        Self: Sized,
    {
        obj.j()
    }
}

impl CloneToFromJava for Option<i32> {
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

impl CloneToFromJava for GlobalRef {
    fn clone_to_java<'a>(&self, _env: JNIEnv<'a>) -> jni::errors::Result<JValue<'a>> {
        let o = self.clone().as_obj().into_inner();
        Ok(JValue::Object(o.into()))
    }

    fn clone_from_java(env: JNIEnv, obj: JValue) -> jni::errors::Result<Self>
    where
        Self: Sized,
    {
        let obj = obj.l()?;
        env.new_global_ref(obj)
    }
}

impl CloneToFromJava for Bytes {
    fn clone_to_java<'a>(&self, env: JNIEnv<'a>) -> jni::errors::Result<JValue<'a>> {
        let o = env.byte_array_from_slice(self)?;
        Ok(JValue::Object(o.into()))
    }

    fn clone_from_java(env: JNIEnv, obj: JValue) -> jni::errors::Result<Self>
    where
        Self: Sized,
    {
        let o = obj.l()?.into_inner();
        env.convert_byte_array(o).map(Bytes::from)
    }
}

impl<K, V> CloneToFromJava for HashMap<K, V>
where
    K: CloneToFromJava + Eq + Hash,
    V: CloneToFromJava,
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

impl<K, V> CloneToFromJava for IndexMap<K, V>
where
    K: CloneToFromJava + Eq + Hash,
    V: CloneToFromJava,
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

impl<K> CloneToFromJava for IndexSet<K>
where
    K: CloneToFromJava + Eq + Hash,
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
