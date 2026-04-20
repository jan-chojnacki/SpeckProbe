use clap::ValueEnum;
use serde::{Deserialize, Serialize};

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
