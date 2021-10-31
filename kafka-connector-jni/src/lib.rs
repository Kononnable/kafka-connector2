pub mod clients;
pub mod common;

pub trait FromJObject {
    type EnumType;
    fn from_jobject(
        env: jni::JNIEnv,
        obj: jni::objects::JObject,
    ) -> jni::errors::Result<Self::EnumType>;
}
pub trait ToJObject {
    fn to_jobject(&self, env: jni::JNIEnv) -> jni::errors::Result<jni::sys::jobject>;
}
