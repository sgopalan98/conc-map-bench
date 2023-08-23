use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    pub name: String,
    pub total_ops: u64,
    pub ops_per_req: usize,
    pub client_threads: usize,
    pub server_threads: usize,
    pub spent: f64,
    pub throughput: f64,
    #[serde(with = "timestamp")]
    pub latency: Duration,
}

mod timestamp {
    use super::*;

    use serde::{de::Deserializer, ser::Serializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        u64::deserialize(deserializer).map(Duration::from_nanos)
    }

    pub fn serialize<S>(value: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (value.as_nanos() as u64).serialize(serializer)
    }
}
