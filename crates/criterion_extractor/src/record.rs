use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize)]
pub(crate) struct Record {
    row_index: usize,
    benchmark: String,
    backend: String,
    function: String,
    version: String,
    suffix: String,
    throughput_num: u64,
    throughput_type: String,
    sample_measured_value: f64,
    unit: String,
    iteration_count: u64,
}

fn split_once_or_default(value: &str) -> (String, String) {
    match value.split_once('/') {
        Some((left, right)) => (left.to_string(), right.to_string()),
        None => (value.to_string(), String::new()),
    }
}

impl From<(usize, RawRecord)> for Record {
    fn from((row_index, value): (usize, RawRecord)) -> Self {
        let (benchmark, backend) = split_once_or_default(&value.group);
        let (version, suffix) = split_once_or_default(&value.value);

        Self {
            row_index,
            benchmark,
            backend,
            function: value.function,
            version,
            suffix,
            throughput_num: value.throughput_num,
            throughput_type: value.throughput_type,
            sample_measured_value: value.sample_measured_value,
            unit: value.unit,
            iteration_count: value.iteration_count,
        }
    }
}
