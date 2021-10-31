use jni::{objects::JObject, sys::jobject, JNIEnv};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimestampType {
    NoTimestampType,
    CreateTime,
    LogAppendTime,
}

// TODO: Make a derive macro?
impl TimestampType {
    pub fn from_jobject(env: JNIEnv, obj: JObject) -> jni::errors::Result<TimestampType> {
        let class = env.find_class("org/apache/kafka/common/record/TimestampType")?;
        assert!(env.is_instance_of(obj, class)?, "Wrong object class");
        let no_timestamp = env
            .get_static_field(
                class,
                "NO_TIMESTAMP_TYPE",
                "Lorg/apache/kafka/common/record/TimestampType;",
            )?
            .l()?;
        if env.is_same_object(obj, no_timestamp)? {
            return Ok(TimestampType::NoTimestampType);
        }
        let create_time = env
            .get_static_field(
                class,
                "CREATE_TIME",
                "Lorg/apache/kafka/common/record/TimestampType;",
            )?
            .l()?;
        if env.is_same_object(obj, create_time)? {
            return Ok(TimestampType::CreateTime);
        }
        let log_append_time = env
            .get_static_field(
                class,
                "LOG_APPEND_TIME",
                "Lorg/apache/kafka/common/record/TimestampType;",
            )?
            .l()?;
        if env.is_same_object(obj, log_append_time)? {
            return Ok(TimestampType::LogAppendTime);
        }
        panic!("Unknown enum value")
    }
    pub fn to_jobject(&self, env: JNIEnv) -> jni::errors::Result<jobject> {
        let class = env.find_class("org/apache/kafka/common/record/TimestampType")?;
        match self {
            TimestampType::NoTimestampType => Ok(env
                .get_static_field(
                    class,
                    "NO_TIMESTAMP_TYPE",
                    "Lorg/apache/kafka/common/record/TimestampType;",
                )?
                .l()?
                .into_inner()),
            TimestampType::CreateTime => Ok(env
                .get_static_field(
                    class,
                    "CREATE_TIME",
                    "Lorg/apache/kafka/common/record/TimestampType;",
                )?
                .l()?
                .into_inner()),
            TimestampType::LogAppendTime => Ok(env
                .get_static_field(
                    class,
                    "LOG_APPEND_TIME",
                    "Lorg/apache/kafka/common/record/TimestampType;",
                )?
                .l()?
                .into_inner()),
        }
    }
}
