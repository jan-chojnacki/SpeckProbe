use super::{BackendHint, CipherFunction, CipherMode, SpeckVersion};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BenchmarkConfig {
    pub bits: usize,
    pub cipher_modes: Vec<CipherMode>,
    pub speck_versions: Vec<SpeckVersion>,
    pub cipher_functions: Vec<CipherFunction>,
    pub backend_hints: Vec<BackendHint>,
    pub suffix_bytes_values: Vec<usize>,
}
