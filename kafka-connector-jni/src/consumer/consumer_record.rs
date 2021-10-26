use kafka_connector_macros::java_property_getter;

java_property_getter!(
 * Class:     org_apache_kafka_clients_consumer_ConsumerRecord
 * Method:    topic
 * Signature: ()Ljava/lang/String;
);

java_property_getter!(
 * Class:     org_apache_kafka_clients_consumer_ConsumerRecord
 * Method:    partition
 * Signature: ()I
);

java_property_getter!(
 * Class:     org_apache_kafka_clients_consumer_ConsumerRecord
 * Method:    headers
 * Signature: ()Lorg/apache/kafka/common/header/Headers;
);

java_property_getter!(
 * Class:     org_apache_kafka_clients_consumer_ConsumerRecord
 * Method:    key
 * Signature: ()Ljava/lang/Object;
);

java_property_getter!(
 * Class:     org_apache_kafka_clients_consumer_ConsumerRecord
 * Method:    value
 * Signature: ()Ljava/lang/Object;
);

java_property_getter!(
 * Class:     org_apache_kafka_clients_consumer_ConsumerRecord
 * Method:    offset
 * Signature: ()J
);

java_property_getter!(
 * Class:     org_apache_kafka_clients_consumer_ConsumerRecord
 * Method:    timestamp
 * Signature: ()J
);

java_property_getter!(
 * Class:     org_apache_kafka_clients_consumer_ConsumerRecord
 * Method:    timestampType
 * Signature: ()Lorg/apache/kafka/common/record/TimestampType;
);

java_property_getter!(
 * Class:     org_apache_kafka_clients_consumer_ConsumerRecord
 * Method:    serializedKeySize
 * Signature: ()I
);

java_property_getter!(
 * Class:     org_apache_kafka_clients_consumer_ConsumerRecord
 * Method:    serializedValueSize
 * Signature: ()I
);

java_property_getter!(
 * Class:     org_apache_kafka_clients_consumer_ConsumerRecord
 * Method:    leaderEpoch
 * Signature: ()Ljava/util/Optional;
);
