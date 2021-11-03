use jni::{
    objects::{GlobalRef, JObject, JValue},
    sys::{jint, jlong, jobject, jstring},
    JNIEnv,
};
use kafka_connector_macros::rust_property_getter;

use crate::{
    common::{
        header::internals::record_headers::RecordHeaders, record::timestamp_type::TimestampType,
    },
    CloneToFromJava,
};

pub struct ConsumerRecord<K, V> {
    pub topic: String,
    pub partition: i32,
    pub offset: i64,
    pub timestamp: i64,
    pub timestamp_type: TimestampType,
    pub serialized_key_size: i32,
    pub serialized_value_size: i32,
    pub headers: RecordHeaders,
    pub key: K,
    pub value: V,
    pub leader_epoch: Option<i32>,
}

/*
 * Class:     org_apache_kafka_clients_consumer_ConsumerRecord
 * Method:    rustConstructor
 * Signature: (Ljava/lang/String;IJJLorg/apache/kafka/common/record/TimestampType;IILjava/lang/Object;Ljava/lang/Object;Lorg/apache/kafka/common/header/Headers;Ljava/util/Optional;)V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_clients_consumer_ConsumerRecord_rustConstructor(
    env: JNIEnv,
    obj: JObject,
    topic: jstring,
    partition: jint,
    offset: jlong,
    timestamp: jlong,
    timestamp_type: jobject,
    serialized_key_size: jint,
    serialized_value_size: jint,
    key: jobject,
    value: jobject,
    headers: jobject,
    leader_epoch: jobject,
) {
    let result = || -> jni::errors::Result<_> {
        let topic = CloneToFromJava::clone_from_java(env, topic.into())?;
        let timestamp_type = CloneToFromJava::clone_from_java(env, timestamp_type.into())?;
        let headers = CloneToFromJava::clone_from_java(env, headers.into())?;
        let key: GlobalRef = CloneToFromJava::clone_from_java(env, key.into())?;
        let value: GlobalRef = CloneToFromJava::clone_from_java(env, value.into())?;
        let leader_epoch = CloneToFromJava::clone_from_java(env, leader_epoch.into())?;

        let record_headers = Box::new(ConsumerRecord {
            topic,
            partition,
            offset,
            timestamp,
            timestamp_type,
            serialized_key_size,
            serialized_value_size,
            headers,
            key,
            value,
            leader_epoch,
        });
        let ptr = Box::into_raw(record_headers);
        env.set_field(obj, "rustPointer", "J", JValue::Long(ptr as i64))?;

        Ok(())
    }();

    match result {
        Ok(_) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!("{:?}", result),
    }
}
/*
 * Class:     org_apache_kafka_clients_consumer_ConsumerRecord
 * Method:    rustDeconstructor
 * Signature: ()V
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_clients_consumer_ConsumerRecord_rustDeconstructor(
    env: JNIEnv,
    obj: JObject,
) {
    let result = || -> jni::errors::Result<_> {
        let ptr = env.get_field(obj, "rustPointer", "J")?.j()?;
        let _record_headers =
            unsafe { Box::from_raw(ptr as *mut ConsumerRecord<GlobalRef, GlobalRef>) };

        Ok(())
    }();
    match result {
        Ok(_) | Err(jni::errors::Error::JavaException) => (),
        _ => panic!("{:?}", result),
    };
}

rust_property_getter!(
 * Struct:    ConsumerRecord<GlobalRef,GlobalRef>
 * Class:     org_apache_kafka_clients_consumer_ConsumerRecord
 * Method:    topic
 * Signature: ()Ljava/lang/String;
);

rust_property_getter!(
 * Struct:    ConsumerRecord<GlobalRef,GlobalRef>
 * Class:     org_apache_kafka_clients_consumer_ConsumerRecord
 * Method:    partition
 * Signature: ()I
);

rust_property_getter!(
 * Struct:    ConsumerRecord<GlobalRef,GlobalRef>
 * Class:     org_apache_kafka_clients_consumer_ConsumerRecord
 * Method:    headers
 * Signature: ()Lorg/apache/kafka/common/header/Headers;
);

rust_property_getter!(
 * Struct:    ConsumerRecord<GlobalRef,GlobalRef>
 * Class:     org_apache_kafka_clients_consumer_ConsumerRecord
 * Method:    key
 * Signature: ()Ljava/lang/Object;
);

rust_property_getter!(
 * Struct:    ConsumerRecord<GlobalRef,GlobalRef>
 * Class:     org_apache_kafka_clients_consumer_ConsumerRecord
 * Method:    value
 * Signature: ()Ljava/lang/Object;
);

rust_property_getter!(
 * Struct:    ConsumerRecord<GlobalRef,GlobalRef>
 * Class:     org_apache_kafka_clients_consumer_ConsumerRecord
 * Method:    offset
 * Signature: ()J
);

rust_property_getter!(
 * Struct:    ConsumerRecord<GlobalRef,GlobalRef>
 * Class:     org_apache_kafka_clients_consumer_ConsumerRecord
 * Method:    timestamp
 * Signature: ()J
);

rust_property_getter!(
 * Struct:    ConsumerRecord<GlobalRef,GlobalRef>
 * Class:     org_apache_kafka_clients_consumer_ConsumerRecord
 * Method:    timestampType
 * Signature: ()Lorg/apache/kafka/common/record/TimestampType;
);

rust_property_getter!(
 * Struct:    ConsumerRecord<GlobalRef,GlobalRef>
 * Class:     org_apache_kafka_clients_consumer_ConsumerRecord
 * Method:    serializedKeySize
 * Signature: ()I
);

rust_property_getter!(
 * Struct:    ConsumerRecord<GlobalRef,GlobalRef>
 * Class:     org_apache_kafka_clients_consumer_ConsumerRecord
 * Method:    serializedValueSize
 * Signature: ()I
);

rust_property_getter!(
 * Struct:    ConsumerRecord<GlobalRef,GlobalRef>
 * Class:     org_apache_kafka_clients_consumer_ConsumerRecord
 * Method:    leaderEpoch
 * Signature: ()Ljava/util/Optional;
);
