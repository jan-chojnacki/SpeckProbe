use crate::search::executor::{BackendHint, CipherFunction, CipherMode};
use crate::speck::SpeckVersion;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct BenchmarkRecord {
    pub bits_measured: usize,
    pub benchmark: &'static str,
    pub backend: BackendHint,
    pub architecture: &'static str,
    pub cipher_mode: CipherMode,
    pub function: CipherFunction,
    pub version: SpeckVersion,
    pub suffix: usize,
    pub throughput_num: u64,
    pub unit: &'static str,
    pub duration_ns: u128,
}
