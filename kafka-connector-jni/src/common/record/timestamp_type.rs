use kafka_connector_macros::JavaEnum;

#[derive(Clone, Copy, Debug, PartialEq, Eq, JavaEnum)]
#[java_class = "org/apache/kafka/common/record/TimestampType"]
pub enum TimestampType {
    #[java_variant = "NO_TIMESTAMP_TYPE"]
    NoTimestampType,
    #[java_variant = "CREATE_TIME"]
    CreateTime,
    #[java_variant = "LOG_APPEND_TIME"]
    LogAppendTime,
}
