use super::benchmark::BenchmarkConfig;
use super::{BackendHint, CipherFunction, CipherMode, SpeckVersion};

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
