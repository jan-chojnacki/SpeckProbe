use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct RawRecord {
    pub(crate) group: String,
    pub(crate) function: String,
    pub(crate) value: String,
    pub(crate) throughput_num: u64,
    pub(crate) throughput_type: String,
    pub(crate) sample_measured_value: f64,
    pub(crate) unit: String,
    pub(crate) iteration_count: u64,
}
