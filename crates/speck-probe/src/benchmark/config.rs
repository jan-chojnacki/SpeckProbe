use crate::search::executor::{BackendHint, CipherFunction, CipherMode};
use crate::speck::SpeckVersion;
use serde::{Deserialize, Serialize};

/// Configuration for a full benchmark run covering all listed parameter combinations.
#[derive(Debug, Serialize, Deserialize)]
pub struct BenchmarkConfig {
    /// Key-space width in bits measured per pass.
    pub bits: usize,
    pub cipher_modes: Vec<CipherMode>,
    pub speck_versions: Vec<SpeckVersion>,
    pub cipher_functions: Vec<CipherFunction>,
    pub backend_hints: Vec<BackendHint>,
    /// Suffix byte counts that are swept per pass to vary the work per iteration.
    pub suffix_bytes_values: Vec<usize>,
}

pub fn sample() -> BenchmarkConfig {
    BenchmarkConfig {
        bits: 24,
        cipher_modes: vec![CipherMode::Ecb],
        speck_versions: vec![SpeckVersion::Speck32_64, SpeckVersion::Speck64_128],
        cipher_functions: vec![
            CipherFunction::Encrypt,
            CipherFunction::EncryptInflight,
            CipherFunction::Decrypt,
        ],
        backend_hints: vec![BackendHint::Auto, BackendHint::Scalar],
        suffix_bytes_values: vec![1, 2],
    }
}
