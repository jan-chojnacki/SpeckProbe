use crate::search::executor::{BackendHint, CipherFunction};
use crate::speck::SpeckVersion;
use serde::Serialize;

/// A single timing measurement produced by one benchmark pass.
#[derive(Debug, Serialize)]
pub struct BenchmarkRecord {
    pub bits_measured: usize,
    pub benchmark: &'static str,
    pub backend: BackendHint,
    pub architecture: &'static str,
    pub function: CipherFunction,
    pub version: SpeckVersion,
    pub suffix: usize,
    pub throughput_num: u64,
    pub unit: &'static str,
    pub duration_ns: u128,
}
