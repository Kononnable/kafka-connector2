use kafka_connector_macros::JavaEnum;

#[derive(Clone, Copy, Debug, PartialEq, Eq, JavaEnum)]
#[java_class = "java/util/concurrent/TimeUnit"]
pub enum TimeUnit {
    #[java_variant = "NANOSECONDS"]
    Nanoseconds,
    #[java_variant = "MICROSECONDS"]
    Microseconds,
    #[java_variant = "MILLISECONDS"]
    Milliseconds,
    #[java_variant = "SECONDS"]
    Seconds,
    #[java_variant = "MINUTES"]
    Minutes,
    #[java_variant = "HOURS"]
    Hours,
    #[java_variant = "DAYS"]
    Days,
}
impl TimeUnit {
    pub fn convert(&self, time_ms: u128) -> f64 {
        let time_ms = time_ms as f64;
        match self {
            TimeUnit::Nanoseconds => time_ms * 1_000_000.0,
            TimeUnit::Microseconds => time_ms * 1_000.0,
            TimeUnit::Milliseconds => time_ms,
            TimeUnit::Seconds => time_ms / 1_000.0,
            TimeUnit::Minutes => time_ms / (60.0 * 1_000.0),
            TimeUnit::Hours => time_ms / (60.0 * 60.0 * 1_000.0),
            TimeUnit::Days => time_ms / (24.0 * 60.0 * 60.0 * 1_000.0),
        }
    }
}
