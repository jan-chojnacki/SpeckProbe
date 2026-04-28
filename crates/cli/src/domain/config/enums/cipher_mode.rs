use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Copy, Clone, ValueEnum, Serialize, Deserialize)]
pub enum CipherMode {
    Ecb,
    Cbc,
}

impl fmt::Display for CipherMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            CipherMode::Ecb => "ECB",
            CipherMode::Cbc => "CBC",
        };
        write!(f, "{}", s)
    }
}

impl From<CipherMode> for runtime::api::CipherMode {
    fn from(value: CipherMode) -> Self {
        match value {
            CipherMode::Ecb => runtime::api::CipherMode::Ecb,
            CipherMode::Cbc => runtime::api::CipherMode::Cbc,
        }
    }
}

impl From<CipherMode> for utils::CipherOperationMode {
    fn from(value: CipherMode) -> Self {
        match value {
            CipherMode::Ecb => utils::CipherOperationMode::ECB,
            CipherMode::Cbc => utils::CipherOperationMode::CBC,
        }
    }
}
