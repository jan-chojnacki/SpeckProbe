use clap::ValueEnum;
use serde::{Deserialize, Serialize};

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

impl From<BackendHint> for runtime::api::BackendHint {
    fn from(value: BackendHint) -> Self {
        match value {
            BackendHint::Auto => runtime::api::BackendHint::Auto,
            BackendHint::Scalar => runtime::api::BackendHint::Scalar,
            #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
            BackendHint::Sse2 => runtime::api::BackendHint::Sse2,
            #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
            BackendHint::Avx2 => runtime::api::BackendHint::Avx2,
            #[cfg(all(target_arch = "x86_64", target_feature = "avx512bw"))]
            BackendHint::Avx512 => runtime::api::BackendHint::Avx512,
            #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
            BackendHint::Neon => runtime::api::BackendHint::Neon,
        }
    }
}
