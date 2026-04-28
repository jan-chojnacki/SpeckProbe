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

pub fn generate_benchmark_sample() -> BenchmarkConfig {
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
