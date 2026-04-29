use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Block cipher mode of operation.
#[derive(Debug, Copy, Clone, ValueEnum, Serialize, Deserialize)]
pub enum CipherMode {
    /// Electronic Code Book — each block encrypted independently.
    Ecb,
    /// Cipher Block Chaining — each block XORed with the previous ciphertext block.
    Cbc,
}

impl fmt::Display for CipherMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CipherMode::Ecb => write!(f, "ECB"),
            CipherMode::Cbc => write!(f, "CBC"),
        }
    }
}

impl From<CipherMode> for crate::runtime::api::CipherMode {
    fn from(value: CipherMode) -> Self {
        match value {
            CipherMode::Ecb => crate::runtime::api::CipherMode::Ecb,
            CipherMode::Cbc => crate::runtime::api::CipherMode::Cbc,
        }
    }
}

impl From<CipherMode> for crate::cipher::CipherMode {
    fn from(value: CipherMode) -> Self {
        match value {
            CipherMode::Ecb => crate::cipher::CipherMode::ECB,
            CipherMode::Cbc => crate::cipher::CipherMode::CBC,
        }
    }
}
