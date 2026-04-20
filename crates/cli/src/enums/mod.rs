use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, ValueEnum, Serialize, Deserialize)]
pub enum SpeckVersion {
    #[value(name = "32/64")]
    Speck32_64,
    #[value(name = "48/72")]
    Speck48_72,
    #[value(name = "48/96")]
    Speck48_96,
    #[value(name = "64/96")]
    Speck64_96,
    #[value(name = "64/128")]
    Speck64_128,
    #[value(name = "96/96")]
    Speck96_96,
    #[value(name = "96/144")]
    Speck96_144,
    #[value(name = "128/128")]
    Speck128_128,
    #[value(name = "128/192")]
    Speck128_192,
    #[value(name = "128/256")]
    Speck128_256,
}

impl From<SpeckVersion> for speck::SpeckVersion {
    fn from(value: SpeckVersion) -> Self {
        match value {
            SpeckVersion::Speck32_64 => speck::SpeckVersion::Speck32_64,
            SpeckVersion::Speck48_72 => speck::SpeckVersion::Speck48_72,
            SpeckVersion::Speck48_96 => speck::SpeckVersion::Speck48_96,
            SpeckVersion::Speck64_96 => speck::SpeckVersion::Speck64_96,
            SpeckVersion::Speck64_128 => speck::SpeckVersion::Speck64_128,
            SpeckVersion::Speck96_96 => speck::SpeckVersion::Speck96_96,
            SpeckVersion::Speck96_144 => speck::SpeckVersion::Speck96_144,
            SpeckVersion::Speck128_128 => speck::SpeckVersion::Speck128_128,
            SpeckVersion::Speck128_192 => speck::SpeckVersion::Speck128_192,
            SpeckVersion::Speck128_256 => speck::SpeckVersion::Speck128_256,
        }
    }
}

#[derive(Debug, Clone, ValueEnum, Serialize, Deserialize)]
pub enum CipherFunction {
    Encrypt,
    Decrypt,
    EncryptInflight,
}

impl From<CipherFunction> for runtime::api::CipherFunction {
    fn from(value: CipherFunction) -> Self {
        match value {
            CipherFunction::Encrypt => runtime::api::CipherFunction::Encrypt,
            CipherFunction::Decrypt => runtime::api::CipherFunction::Decrypt,
            CipherFunction::EncryptInflight => runtime::api::CipherFunction::EncryptInflight,
        }
    }
}

#[derive(Debug, Clone, ValueEnum, Serialize, Deserialize)]
pub enum CipherMode {
    Ecb,
    Cbc,
}

impl From<CipherMode> for runtime::api::CipherMode {
    fn from(value: CipherMode) -> Self {
        match value {
            CipherMode::Ecb => runtime::api::CipherMode::Ecb,
            CipherMode::Cbc => runtime::api::CipherMode::Cbc,
        }
    }
}

#[derive(Debug, Clone, ValueEnum, Serialize, Deserialize)]
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
