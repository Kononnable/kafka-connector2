use jni::{
    objects::{GlobalRef, JObject, JValue},
    JNIEnv,
};

use crate::clone_to_from_java::CloneToFromJava;

use super::metric_config::MetricConfig;

#[derive(Clone)]
pub enum Measurable {
    Java(JavaMeasurable),
}

impl CloneToFromJava for Measurable {
    fn clone_to_java<'a>(&self, env: JNIEnv<'a>) -> jni::errors::Result<jni::objects::JValue<'a>> {
        match self {
            Measurable::Java(m) => m.clone_to_java(env),
        }
    }

    fn clone_from_java(env: JNIEnv, obj: jni::objects::JValue) -> jni::errors::Result<Self>
    where
        Self: Sized,
    {
        JavaMeasurable::clone_from_java(env, obj).map(Measurable::Java)
    }
}
#[derive(Clone)]
pub struct JavaMeasurable {
    measure_fn: GlobalRef,
}
impl JavaMeasurable {
    pub fn new(env: JNIEnv, measure_fn: JObject) -> jni::errors::Result<JavaMeasurable> {
        Ok(JavaMeasurable {
            measure_fn: env.new_global_ref(measure_fn)?,
        })
    }
    pub fn measure(&self, env: JNIEnv, config: MetricConfig, now: u64) -> jni::errors::Result<f64> {
        let config = config.clone_to_java(env)?;
        let ret = env
            .call_method(
                self.measure_fn.as_obj(),
                "measure",
                "(Lorg/apache/kafka/common/metrics/MetricConfig;J)D",
                &[config, JValue::Long(now as i64)],
            )?
            .d()?;
        Ok(ret)
    }
}
impl CloneToFromJava for JavaMeasurable {
    fn clone_to_java<'a>(&self, env: JNIEnv<'a>) -> jni::errors::Result<JValue<'a>> {
        self.measure_fn.clone_to_java(env)
    }

    fn clone_from_java(env: JNIEnv, obj: JValue) -> jni::errors::Result<Self>
    where
        Self: Sized,
    {
        Ok(JavaMeasurable {
            measure_fn: env.new_global_ref(obj.l()?)?,
        })
    }
}
