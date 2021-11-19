use std::time::Duration;

use crate::clone_to_from_java_for_struct;
use indexmap::IndexMap;
use jni::{
    objects::{JObject, JValue},
    JNIEnv,
};
use kafka_connector_macros::{rust_property_chain_setter, rust_property_getter};

use super::{quota::Quota, sensor_recording_level::SensorRecordingLevel};

#[derive(Debug, Clone)]
pub struct MetricConfig {
    pub quota: Option<Quota>,
    pub samples: u32,
    pub event_window: u64,
    pub time_window_ms: u128,
    pub tags: IndexMap<String, String>,
    pub record_level: SensorRecordingLevel,
}
impl Default for MetricConfig {
    fn default() -> Self {
        Self {
            quota: Default::default(),
            samples: 2,
            event_window: u64::MAX,
            time_window_ms: Duration::from_secs(30).as_millis(),
            tags: Default::default(),
            record_level: SensorRecordingLevel::Info,
        }
    }
}
clone_to_from_java_for_struct!(MetricConfig, "org/apache/kafka/common/metrics/MetricConfig");

/*
 * Class:     org_apache_kafka_common_metrics_MetricConfig
 * Method:    rustConstructor
 * Signature: ()V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_MetricConfig_rustConstructor(
    env: JNIEnv,
    obj: JObject,
) {
    let result = || -> jni::errors::Result<_> {
        let metrics_config = Box::new(MetricConfig::default());
        let ptr = Box::into_raw(metrics_config);
        env.set_field(obj, "rustPointer", "J", JValue::Long(ptr as i64))?;

        Ok(())
    }();
    match result {
        Ok(_) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!("{:?}", result),
    }
}

/*
 * Class:     org_apache_kafka_common_metrics_MetricConfig
 * Method:    rustDestructor
 * Signature: ()V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_common_metrics_MetricConfig_rustDestructor(
    env: JNIEnv,
    obj: JObject,
) {
    let result = || -> jni::errors::Result<_> {
        let ptr = env.get_field(obj, "rustPointer", "J")?.j()?;
        let _obj = unsafe { Box::from_raw(ptr as *mut MetricConfig) };

        Ok(())
    }();
    match result {
        Ok(_) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!("{:?}", result),
    }
}

rust_property_getter!(
 * Function:  Java_org_apache_kafka_common_metrics_MetricConfig_quota__
 * Nullable:  True
 * Struct:    MetricConfig
 * Class:     org_apache_kafka_common_metrics_MetricConfig
 * Method:    quota
 * Signature: ()Lorg/apache/kafka/common/metrics/Quota;
);

rust_property_chain_setter!(
 * Function:  Java_org_apache_kafka_common_metrics_MetricConfig_quota__Lorg_apache_kafka_common_metrics_Quota_2
 * Nullable:  True
 * Struct:    MetricConfig
 * Class:     org_apache_kafka_common_metrics_MetricConfig
 * Method:    quota
 * Signature: (Lorg/apache/kafka/common/metrics/Quota;)Lorg/apache/kafka/common/metrics/MetricConfig;
);

rust_property_getter!(
 * Function:  Java_org_apache_kafka_common_metrics_MetricConfig_eventWindow__
 * Struct:    MetricConfig
 * Class:     org_apache_kafka_common_metrics_MetricConfig
 * Method:    eventWindow
 * Signature: ()J
);

rust_property_chain_setter!(
 * Function:  Java_org_apache_kafka_common_metrics_MetricConfig_eventWindow__J
 * Struct:    MetricConfig
 * Class:     org_apache_kafka_common_metrics_MetricConfig
 * Method:    eventWindow
 * Signature: (J)Lorg/apache/kafka/common/metrics/MetricConfig;
);

rust_property_getter!(
 * Function:  Java_org_apache_kafka_common_metrics_MetricConfig_timeWindowMs__
 * Struct:    MetricConfig
 * Class:     org_apache_kafka_common_metrics_MetricConfig
 * Method:    timeWindowMs
 * Signature: ()J
);

rust_property_chain_setter!(
 * Function:  Java_org_apache_kafka_common_metrics_MetricConfig_timeWindowMs__J
 * Struct:    MetricConfig
 * Class:     org_apache_kafka_common_metrics_MetricConfig
 * Method:    timeWindowMs
 * Signature: (J)Lorg/apache/kafka/common/metrics/MetricConfig;
);

rust_property_getter!(
 * Function:  Java_org_apache_kafka_common_metrics_MetricConfig_tags__
 * Struct:    MetricConfig
 * Class:     org_apache_kafka_common_metrics_MetricConfig
 * Method:    tags
 * Signature: ()Ljava/util/Map;
);

rust_property_chain_setter!(
 * Function:  Java_org_apache_kafka_common_metrics_MetricConfig_tags__Ljava_util_Map_2
 * Struct:    MetricConfig
 * Class:     org_apache_kafka_common_metrics_MetricConfig
 * Method:    tags
 * Signature: (Ljava/util/Map;)Lorg/apache/kafka/common/metrics/MetricConfig;
);

rust_property_getter!(
 * Function:  Java_org_apache_kafka_common_metrics_MetricConfig_samples__
 * Struct:    MetricConfig
 * Class:     org_apache_kafka_common_metrics_MetricConfig
 * Method:    samples
 * Signature: ()I
);

rust_property_chain_setter!(
 * Function:  Java_org_apache_kafka_common_metrics_MetricConfig_samples__I
 * Struct:    MetricConfig
 * Class:     org_apache_kafka_common_metrics_MetricConfig
 * Method:    samples
 * Signature: (I)Lorg/apache/kafka/common/metrics/MetricConfig;
);

rust_property_getter!(
 * Function:  Java_org_apache_kafka_common_metrics_MetricConfig_recordLevel__
 * Struct:    MetricConfig
 * Class:     org_apache_kafka_common_metrics_MetricConfig
 * Method:    recordLevel
 * Signature: ()Lorg/apache/kafka/common/metrics/SensorRecordingLevel;
);

rust_property_chain_setter!(
 * Function:  Java_org_apache_kafka_common_metrics_MetricConfig_recordLevel__Lorg_apache_kafka_common_metrics_SensorRecordingLevel_2
 * Struct:    MetricConfig
 * Class:     org_apache_kafka_common_metrics_MetricConfig
 * Method:    recordLevel
 * Signature: (Lorg/apache/kafka/common/metrics/SensorRecordingLevel;)Lorg/apache/kafka/common/metrics/MetricConfig;
);
