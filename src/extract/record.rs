use super::RawRecord;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CriterionRecord {
    pub row_index: usize,
    pub benchmark: String,
    pub backend: String,
    pub architecture: String,
    pub function: String,
    pub version: String,
    pub suffix: String,
    pub throughput_num: u64,
    pub throughput_type: String,
    pub sample_measured_value: f64,
    pub unit: String,
    pub iteration_count: u64,
}

fn split_once_or_default(value: &str) -> (String, String) {
    match value.split_once('/') {
        Some((left, right)) => (left.to_string(), right.to_string()),
        None => (value.to_string(), String::new()),
    }
}

impl CriterionRecord {
    pub(crate) fn from_raw(row_index: usize, raw: RawRecord, architecture: &str) -> Self {
        let (benchmark, backend) = split_once_or_default(&raw.group);
        let (version, suffix) = split_once_or_default(&raw.value);
        Self {
            row_index,
            benchmark,
            backend,
            architecture: architecture.to_string(),
            function: raw.function,
            version,
            suffix,
            throughput_num: raw.throughput_num,
            throughput_type: raw.throughput_type,
            sample_measured_value: raw.sample_measured_value,
            unit: raw.unit,
            iteration_count: raw.iteration_count,
        }
    }
}
