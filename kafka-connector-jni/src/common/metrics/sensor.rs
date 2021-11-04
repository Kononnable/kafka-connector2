use jni::{objects::JObject, sys::jobject, JNIEnv};
use kafka_connector_macros::JavaEnum;

use crate::clone_to_from_java::CloneToFromJava;

#[derive(Clone, Copy, Debug, PartialEq, Eq, JavaEnum)]
#[java_class = "org/apache/kafka/common/metrics/Sensor$RecordingLevel"]
pub enum RecordingLevel {
    #[java_variant = "INFO"]
    Info,
    #[java_variant = "DEBUG"]
    Debug,
    #[java_variant = "TRACE"]
    Trace,
}

impl RecordingLevel {
    pub fn should_record(&self, recording_level: RecordingLevel) -> bool {
        match recording_level {
            RecordingLevel::Info => [RecordingLevel::Info].contains(self),
            RecordingLevel::Debug => [RecordingLevel::Info, RecordingLevel::Debug].contains(self),
            RecordingLevel::Trace => true,
        }
    }
}

/*
 * Class:     org_apache_kafka_common_metrics_Sensor_RecordingLevel
 * Method:    shouldRecord
 * Signature: (Lorg/apache/kafka/common/metrics/Sensor/RecordingLevel;)Z
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_Sensor_00024RecordingLevel_shouldRecord(
    env: JNIEnv,
    obj: JObject,
    val: jobject,
) -> bool {
    let result = || -> jni::errors::Result<_> {
        let this = RecordingLevel::clone_from_java(env, obj.into())?;
        let val = RecordingLevel::clone_from_java(env, val.into())?;
        Ok(this.should_record(val))
    }();
    match result {
        Ok(v) => v,
        Err(jni::errors::Error::JavaException) => true,
        _ => panic!("{:?}", result),
    }
}
