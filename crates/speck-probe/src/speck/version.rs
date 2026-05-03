use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Eq, PartialEq, strum::Display, ValueEnum, Serialize, Deserialize)]
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

impl SpeckVersion {
    pub fn word_size_bytes(&self) -> usize {
        self.block_size_bytes() / 2
    }

    pub fn block_size_bytes(&self) -> usize {
        match self {
            SpeckVersion::Speck32_64 => 4,
            SpeckVersion::Speck48_72 => 6,
            SpeckVersion::Speck48_96 => 6,
            SpeckVersion::Speck64_96 => 8,
            SpeckVersion::Speck64_128 => 8,
            SpeckVersion::Speck96_96 => 12,
            SpeckVersion::Speck96_144 => 12,
            SpeckVersion::Speck128_128 => 16,
            SpeckVersion::Speck128_192 => 16,
            SpeckVersion::Speck128_256 => 16,
        }
    }

    pub fn prefix_size_bytes(&self) -> usize {
        match self {
            SpeckVersion::Speck32_64 => 0,
            SpeckVersion::Speck48_72 => 1,
            SpeckVersion::Speck48_96 => 4,
            SpeckVersion::Speck64_96 => 4,
            SpeckVersion::Speck64_128 => 8,
            SpeckVersion::Speck96_96 => 4,
            SpeckVersion::Speck96_144 => 10,
            SpeckVersion::Speck128_128 => 8,
            SpeckVersion::Speck128_192 => 16,
            SpeckVersion::Speck128_256 => 24,
        }
    }

    pub fn key_size_bytes(&self) -> usize {
        match self {
            SpeckVersion::Speck32_64 => 8,
            SpeckVersion::Speck48_72 => 9,
            SpeckVersion::Speck48_96 => 12,
            SpeckVersion::Speck64_96 => 12,
            SpeckVersion::Speck64_128 => 16,
            SpeckVersion::Speck96_96 => 12,
            SpeckVersion::Speck96_144 => 18,
            SpeckVersion::Speck128_128 => 16,
            SpeckVersion::Speck128_192 => 24,
            SpeckVersion::Speck128_256 => 32,
        }
    }
}
