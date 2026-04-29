use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Which cipher operation to benchmark or use as the key-test function.
#[derive(Debug, Copy, Clone, ValueEnum, Serialize, Deserialize)]
pub enum CipherFunction {
    Encrypt,
    Decrypt,
    /// Encrypt where plaintext and ciphertext share the same buffer (in-place).
    EncryptInflight,
}

impl fmt::Display for CipherFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CipherFunction::Encrypt => write!(f, "Encrypt"),
            CipherFunction::Decrypt => write!(f, "Decrypt"),
            CipherFunction::EncryptInflight => write!(f, "EncryptInflight"),
        }
    }
}

impl From<CipherFunction> for crate::runtime::api::CipherFunction {
    fn from(value: CipherFunction) -> Self {
        match value {
            CipherFunction::Encrypt => crate::runtime::api::CipherFunction::Encrypt,
            CipherFunction::Decrypt => crate::runtime::api::CipherFunction::Decrypt,
            CipherFunction::EncryptInflight => crate::runtime::api::CipherFunction::EncryptInflight,
        }
    }
}
