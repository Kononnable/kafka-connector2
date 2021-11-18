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
    ($struct_name:ty, $class_name:literal) => {
        impl crate::clone_to_from_java::CloneToFromJava for $struct_name {
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
impl CloneToFromJava for u32 {
    fn clone_to_java<'a>(&self, _env: JNIEnv<'a>) -> jni::errors::Result<JValue<'a>> {
        Ok(match *self {
            u32::MAX => JValue::Int(i32::MAX),
            u32::MIN => JValue::Int(i32::MIN),
            v => JValue::Int(v as i32),
        })
    }

    fn clone_from_java(_env: JNIEnv, obj: JValue) -> jni::errors::Result<Self>
    where
        Self: Sized,
    {
        Ok(obj.i()? as u32)
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
impl CloneToFromJava for u64 {
    fn clone_to_java<'a>(&self, _env: JNIEnv<'a>) -> jni::errors::Result<JValue<'a>> {
        Ok(match *self {
            u64::MAX => JValue::Long(i64::MAX),
            u64::MIN => JValue::Long(i64::MIN),
            v => JValue::Long(v as i64),
        })
    }

    fn clone_from_java(_env: JNIEnv, obj: JValue) -> jni::errors::Result<Self>
    where
        Self: Sized,
    {
        Ok(obj.j()? as u64)
    }
}
impl CloneToFromJava for u128 {
    fn clone_to_java<'a>(&self, _env: JNIEnv<'a>) -> jni::errors::Result<JValue<'a>> {
        Ok(match *self {
            u128::MAX => JValue::Long(i64::MAX),
            u128::MIN => JValue::Long(i64::MIN),
            v => JValue::Long(v as i64),
        })
    }

    fn clone_from_java(_env: JNIEnv, obj: JValue) -> jni::errors::Result<Self>
    where
        Self: Sized,
    {
        Ok(obj.j()? as u128)
    }
}

impl CloneToFromJava for bool {
    fn clone_to_java<'a>(&self, _env: JNIEnv<'a>) -> jni::errors::Result<JValue<'a>> {
        Ok(JValue::Bool((*self) as u8))
    }

    fn clone_from_java(_env: JNIEnv, obj: JValue) -> jni::errors::Result<Self>
    where
        Self: Sized,
    {
        obj.z()
    }
}

impl CloneToFromJava for f64 {
    fn clone_to_java<'a>(&self, _env: JNIEnv<'a>) -> jni::errors::Result<JValue<'a>> {
        Ok(JValue::Double(*self))
    }

    fn clone_from_java(_env: JNIEnv, obj: JValue) -> jni::errors::Result<Self>
    where
        Self: Sized,
    {
        obj.d()
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

impl<T> CloneToFromJava for Vec<T>
where
    T: CloneToFromJava,
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
