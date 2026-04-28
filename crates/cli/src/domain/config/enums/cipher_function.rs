use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Copy, Clone, ValueEnum, Serialize, Deserialize)]
pub enum CipherFunction {
    Encrypt,
    Decrypt,
    EncryptInflight,
}

impl fmt::Display for CipherFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            CipherFunction::Encrypt => "Encrypt",
            CipherFunction::Decrypt => "Decrypt",
            CipherFunction::EncryptInflight => "EncryptInflight",
        };
        write!(f, "{}", s)
    }
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
