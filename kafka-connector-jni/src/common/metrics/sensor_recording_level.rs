use jni::{objects::JObject, sys::jobject, JNIEnv};
use kafka_connector_macros::JavaEnum;

use crate::clone_to_from_java::CloneToFromJava;

#[derive(Clone, Copy, Debug, PartialEq, Eq, JavaEnum)]
#[java_class = "org/apache/kafka/common/metrics/SensorRecordingLevel"]
pub enum SensorRecordingLevel {
    #[java_variant = "INFO"]
    Info,
    #[java_variant = "DEBUG"]
    Debug,
    #[java_variant = "TRACE"]
    Trace,
}

impl SensorRecordingLevel {
    pub fn should_record(&self, recording_level: SensorRecordingLevel) -> bool {
        match recording_level {
            SensorRecordingLevel::Info => [SensorRecordingLevel::Info].contains(self),
            SensorRecordingLevel::Debug => {
                [SensorRecordingLevel::Info, SensorRecordingLevel::Debug].contains(self)
            }
            SensorRecordingLevel::Trace => true,
        }
    }
}

/*
 * Class:     org_apache_kafka_common_metrics_SensorRecordingLevel
 * Method:    shouldRecord
 * Signature: (Lorg/apache/kafka/common/metrics/SensorRecordingLevel;)Z
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_SensorRecordingLevel_shouldRecord(
    env: JNIEnv,
    obj: JObject,
    val: jobject,
) -> bool {
    let result = || -> jni::errors::Result<_> {
        let this = SensorRecordingLevel::clone_from_java(env, obj.into())?;
        let val = SensorRecordingLevel::clone_from_java(env, val.into())?;
        Ok(this.should_record(val))
    }();
    match result {
        Ok(v) => v,
        Err(jni::errors::Error::JavaException) => true,
        _ => panic!("{:?}", result),
    }
}
