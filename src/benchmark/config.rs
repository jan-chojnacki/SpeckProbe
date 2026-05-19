use crate::search::executor::{BackendHint, CipherFunction, CipherMode};
use crate::speck::SpeckVersion;
use crate::speck::SpeckVersion::{
    Speck32_64, Speck48_72, Speck48_96, Speck64_96, Speck64_128, Speck96_96, Speck96_144,
    Speck128_128, Speck128_192, Speck128_256,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BenchmarkConfig {
    pub samples: usize,
    pub step: usize,
    pub cipher_modes: Vec<CipherMode>,
    pub speck_versions: Vec<SpeckVersion>,
    pub cipher_functions: Vec<CipherFunction>,
    pub backend_hints: Vec<BackendHint>,
    pub suffix_bytes_values: Vec<usize>,
}

pub fn sample() -> BenchmarkConfig {
    BenchmarkConfig {
        samples: 3,
        step: 4,
        cipher_modes: vec![CipherMode::Ecb, CipherMode::Cbc],
        speck_versions: vec![
            Speck32_64,
            Speck48_72,
            Speck48_96,
            Speck64_96,
            Speck64_128,
            Speck96_96,
            Speck96_144,
            Speck128_128,
            Speck128_192,
            Speck128_256,
        ],
        cipher_functions: vec![CipherFunction::EncryptInflight],
        #[cfg(all(target_arch = "x86_64", target_feature = "avx512bw"))]
        backend_hints: vec![
            BackendHint::Avx512,
            BackendHint::Avx2,
            BackendHint::Sse2,
            BackendHint::Scalar,
        ],
        #[cfg(all(
            target_arch = "x86_64",
            target_feature = "avx2",
            not(target_feature = "avx512bw")
        ))]
        backend_hints: vec![BackendHint::Avx2, BackendHint::Sse2, BackendHint::Scalar],
        #[cfg(all(
            target_arch = "x86_64",
            not(target_feature = "avx2"),
            not(target_feature = "avx512bw")
        ))]
        backend_hints: vec![BackendHint::Sse2, BackendHint::Scalar],
        #[cfg(target_arch = "aarch64")]
        backend_hints: vec![BackendHint::Neon, BackendHint::Scalar],
        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
        backend_hints: vec![BackendHint::Scalar],
        suffix_bytes_values: vec![1, 2],
    }
}
