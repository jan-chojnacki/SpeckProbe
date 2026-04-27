use crate::domain::config::{BackendHint, CipherFunction, SpeckVersion};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct BenchmarkRecord {
    pub(crate) bits_measured: usize,
    pub(crate) benchmark: &'static str,
    pub(crate) backend: BackendHint,
    pub(crate) architecture: String,
    pub(crate) function: CipherFunction,
    pub(crate) version: SpeckVersion,
    pub(crate) suffix: usize,
    pub(crate) throughput_num: u64,
    pub(crate) unit: &'static str,
    pub(crate) duration_ns: u128,
}
