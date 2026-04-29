use crate::probe::config::enums::{CipherMode, SpeckVersion};
use crate::probe::config::serde::key::{
    deserialize_from_hex, deserialize_from_hex_opt, serialize_as_hex, serialize_as_hex_opt,
};
use serde::{Deserialize, Serialize};

/// Configuration for a single encrypt/decrypt operation.
#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptConfig {
    pub speck_version: SpeckVersion,
    pub cipher_mode: CipherMode,
    /// Cipher key, serialized as space-separated hex bytes.
    #[serde(
        serialize_with = "serialize_as_hex",
        deserialize_with = "deserialize_from_hex"
    )]
    pub key: Vec<u8>,
    /// Initialization vector for CBC mode; omitted for ECB.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_as_hex_opt",
        deserialize_with = "deserialize_from_hex_opt"
    )]
    pub iv: Option<Vec<u8>>,
    pub data: String,
}

pub fn sample() -> EncryptConfig {
    EncryptConfig {
        speck_version: SpeckVersion::Speck32_64,
        cipher_mode: CipherMode::Ecb,
        key: vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07],
        iv: None,
        data: "Hello world!".to_string(),
    }
}
