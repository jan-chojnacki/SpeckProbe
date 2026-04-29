use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Hint for selecting a SIMD backend; `Auto` picks the best available at runtime.
#[derive(Debug, Copy, Clone, ValueEnum, Serialize, Deserialize)]
pub enum BackendHint {
    Auto,
    Scalar,
    #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
    Sse2,
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    Avx2,
    #[cfg(all(target_arch = "x86_64", target_feature = "avx512bw"))]
    Avx512,
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    Neon,
}

impl fmt::Display for BackendHint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            BackendHint::Auto => "Auto",
            BackendHint::Scalar => "Scalar",
            #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
            BackendHint::Sse2 => "SSE2",
            #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
            BackendHint::Avx2 => "AVX2",
            #[cfg(all(target_arch = "x86_64", target_feature = "avx512bw"))]
            BackendHint::Avx512 => "AVX512",
            #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
            BackendHint::Neon => "NEON",
        };
        write!(f, "{}", s)
    }
}

impl From<BackendHint> for crate::runtime::api::BackendHint {
    fn from(value: BackendHint) -> Self {
        match value {
            BackendHint::Auto => crate::runtime::api::BackendHint::Auto,
            BackendHint::Scalar => crate::runtime::api::BackendHint::Scalar,
            #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
            BackendHint::Sse2 => crate::runtime::api::BackendHint::Sse2,
            #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
            BackendHint::Avx2 => crate::runtime::api::BackendHint::Avx2,
            #[cfg(all(target_arch = "x86_64", target_feature = "avx512bw"))]
            BackendHint::Avx512 => crate::runtime::api::BackendHint::Avx512,
            #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
            BackendHint::Neon => runtime::api::BackendHint::Neon,
        }
    }
}
