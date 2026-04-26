use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, ValueEnum, Serialize, Deserialize)]
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
