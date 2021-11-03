use jni::{
    objects::{GlobalRef, JObject, JValue},
    JNIEnv,
};

pub mod clients;
pub mod common;

impl CloneToFromJava for String {
    fn clone_to_java<'a>(&self, env: JNIEnv<'a>) -> jni::errors::Result<JValue<'a>> {
        Ok(JValue::Object(env.new_string(self)?.into()))
    }

    fn clone_from_java(env: JNIEnv, obj: JValue) -> jni::errors::Result<Self>
    where
        Self: Sized,
    {
        env.get_string(obj.l()?.into()).map(Into::into)
    }
}

impl CloneToFromJava for i32 {
    fn clone_to_java<'a>(&self, env: JNIEnv<'a>) -> jni::errors::Result<JValue<'a>> {
        Ok(JValue::Int(*self))
    }

    fn clone_from_java(env: JNIEnv, obj: JValue) -> jni::errors::Result<Self>
    where
        Self: Sized,
    {
        obj.i()
    }
}

impl CloneToFromJava for i64 {
    fn clone_to_java<'a>(&self, env: JNIEnv<'a>) -> jni::errors::Result<JValue<'a>> {
        Ok(JValue::Long(*self))
    }

    fn clone_from_java(env: JNIEnv, obj: JValue) -> jni::errors::Result<Self>
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

// TODO: Make a type alias/struct, JavaGenericParameter?
// TODO: Refactor
impl CloneToFromJava for GlobalRef {
    fn clone_to_java<'a>(&self, env: JNIEnv<'a>) -> jni::errors::Result<JValue<'a>> {
        let x = self.clone();
        let y = x.as_obj();
        let z = y.into_inner();
        let w: JObject = z.into();
        Ok(JValue::Object(w))
    }

    fn clone_from_java(env: JNIEnv, obj: JValue) -> jni::errors::Result<Self>
    where
        Self: Sized,
    {
        let obj = obj.l()?;
        let ref_ = env.new_global_ref(obj)?;
        Ok(ref_)
    }
}

pub trait CloneToFromJava {
    // Clones object to java - making a clone readonly in most cases(java changes won't affect real state)
    // May produce errors in java (testing) logic, however needed if we want to avoid Arc<> on rust structures
    fn clone_to_java<'a>(&self, env: JNIEnv<'a>) -> jni::errors::Result<JValue<'a>>;

    // May be slower, but have to clone if we want to avoid Arc<> on each structure in rust
    fn clone_from_java(env: JNIEnv, obj: JValue) -> jni::errors::Result<Self>
    where
        Self: Sized;
}

macro_rules! clone_to_from_java_obj {
    ($struct_name:ident, $class_name:literal) => {
        impl crate::CloneToFromJava for $struct_name {
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
            fn clone_to_java<'a>(&self, env: JNIEnv<'a>) -> jni::errors::Result<JValue<'a>> {
                let class = env.find_class($class_name)?;
                let obj = env.alloc_object(class)?;
                let copy = Box::new(self.clone());
                let ptr = Box::into_raw(copy);
                env.set_field(obj, "rustPointer", "J", JValue::Long(ptr as i64))?;
                Ok(obj.into())
            }
        }
    };
}
pub(crate) use clone_to_from_java_obj;
