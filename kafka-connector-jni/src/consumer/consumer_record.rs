use jni::objects::JObject;
use jni::sys::{jint, jlong, jobject, jstring};
use jni::JNIEnv;

/*
 * Class:     org_apache_kafka_clients_consumer_ConsumerRecord
 * Method:    topic
 * Signature: ()Ljava/lang/String;
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_clients_consumer_ConsumerRecord_topic(
    env: JNIEnv,
    obj: JObject,
) -> jstring {
    let res = env.get_field(obj, "topic", "Ljava/lang/String;").unwrap();

    res.l().unwrap().into_inner()
    // let res = res.l().unwrap();

    // let input: String = env
    //     .get_string(res.into())
    //     .expect("Couldn't get java string!")
    //     .into();

    // let output = env.new_string(input).expect("Couldn't create java string!");
    // // Finally, extract the raw pointer to return.
    // output.into_inner()
}

/*
 * Class:     org_apache_kafka_clients_consumer_ConsumerRecord
 * Method:    partition
 * Signature: ()I
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_clients_consumer_ConsumerRecord_partition(
    env: JNIEnv,
    obj: JObject,
) -> jint {
    let res = env.get_field(obj, "partition", "I").unwrap();
    res.i().unwrap()
}

/*
 * Class:     org_apache_kafka_clients_consumer_ConsumerRecord
 * Method:    headers
 * Signature: ()Lorg/apache/kafka/common/header/Headers;
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_clients_consumer_ConsumerRecord_headers(
    env: JNIEnv,
    obj: JObject,
) -> jobject {
    env.get_field(obj, "headers", "Lorg/apache/kafka/common/header/Headers;")
        .unwrap()
        .l()
        .unwrap()
        .into_inner()
}

/*
 * Class:     org_apache_kafka_clients_consumer_ConsumerRecord
 * Method:    key
 * Signature: ()Ljava/lang/Object;
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_clients_consumer_ConsumerRecord_key(
    env: JNIEnv,
    obj: JObject,
) -> jobject {
    env.get_field(obj, "key", "Ljava/lang/Object;")
        .unwrap()
        .l()
        .unwrap()
        .into_inner()
}

/*
 * Class:     org_apache_kafka_clients_consumer_ConsumerRecord
 * Method:    value
 * Signature: ()Ljava/lang/Object;
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_clients_consumer_ConsumerRecord_value(
    env: JNIEnv,
    obj: JObject,
) -> jobject {
    env.get_field(obj, "value", "Ljava/lang/Object;")
        .unwrap()
        .l()
        .unwrap()
        .into_inner()
}

/*
 * Class:     org_apache_kafka_clients_consumer_ConsumerRecord
 * Method:    offset
 * Signature: ()J
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_clients_consumer_ConsumerRecord_offset(
    env: JNIEnv,
    obj: JObject,
) -> jlong {
    env.get_field(obj, "offset", "J").unwrap().j().unwrap()
}

/*
 * Class:     org_apache_kafka_clients_consumer_ConsumerRecord
 * Method:    timestamp
 * Signature: ()J
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_clients_consumer_ConsumerRecord_timestamp(
    env: JNIEnv,
    obj: JObject,
) -> jlong {
    env.get_field(obj, "timestamp", "J").unwrap().j().unwrap()
}

/*
 * Class:     org_apache_kafka_clients_consumer_ConsumerRecord
 * Method:    timestampType
 * Signature: ()Lorg/apache/kafka/common/record/TimestampType;
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_clients_consumer_ConsumerRecord_timestampType(
    env: JNIEnv,
    obj: JObject,
) -> jobject {
    env.get_field(
        obj,
        "timestampType",
        "Lorg/apache/kafka/common/record/TimestampType;",
    )
    .unwrap()
    .l()
    .unwrap()
    .into_inner()
}

/*
 * Class:     org_apache_kafka_clients_consumer_ConsumerRecord
 * Method:    serializedKeySize
 * Signature: ()I
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_clients_consumer_ConsumerRecord_serializedKeySize(
    env: JNIEnv,
    obj: JObject,
) -> jint {
    env.get_field(obj, "serializedKeySize", "I")
        .unwrap()
        .i()
        .unwrap()
}

/*
 * Class:     org_apache_kafka_clients_consumer_ConsumerRecord
 * Method:    serializedValueSize
 * Signature: ()I
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_clients_consumer_ConsumerRecord_serializedValueSize(
    env: JNIEnv,
    obj: JObject,
) -> jint {
    env.get_field(obj, "serializedValueSize", "I")
        .unwrap()
        .i()
        .unwrap()
}

/*
 * Class:     org_apache_kafka_clients_consumer_ConsumerRecord
 * Method:    leaderEpoch
 * Signature: ()Ljava/util/Optional;
 */
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_org_apache_kafka_clients_consumer_ConsumerRecord_leaderEpoch(
    env: JNIEnv,
    obj: JObject,
) -> jobject {
    env.get_field(obj, "leaderEpoch", "Ljava/util/Optional;")
        .unwrap()
        .l()
        .unwrap()
        .into_inner()
}
