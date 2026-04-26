use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, ValueEnum, Serialize, Deserialize)]
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
