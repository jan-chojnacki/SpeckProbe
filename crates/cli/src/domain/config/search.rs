use crate::domain::config::codec::data::{deserialize_u64_pairs, serialize_u64_pairs};
use crate::domain::config::codec::key::{deserialize_from_hex, serialize_as_hex};
use crate::domain::config::{BackendHint, CipherFunction, CipherMode, SpeckVersion};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchConfig {
    pub cipher_mode: CipherMode,
    pub speck_version: SpeckVersion,
    pub cipher_function: CipherFunction,
    pub suffix_bytes_size: usize,
    pub num_threads: usize,
    pub backend_hint: BackendHint,
    #[serde(
        serialize_with = "serialize_as_hex",
        deserialize_with = "deserialize_from_hex"
    )]
    pub start: Vec<u8>,
    #[serde(
        serialize_with = "serialize_as_hex",
        deserialize_with = "deserialize_from_hex"
    )]
    pub end: Vec<u8>,
    #[serde(
        serialize_with = "serialize_u64_pairs",
        deserialize_with = "deserialize_u64_pairs"
    )]
    pub data: Vec<[u64; 2]>,
    #[serde(
        serialize_with = "serialize_u64_pairs",
        deserialize_with = "deserialize_u64_pairs"
    )]
    pub expected: Vec<[u64; 2]>,
}

pub fn generate_sample() -> SearchConfig {
    SearchConfig {
        cipher_mode: CipherMode::Ecb,
        speck_version: SpeckVersion::Speck32_64,
        cipher_function: CipherFunction::EncryptInflight,
        suffix_bytes_size: 3,
        num_threads: num_cpus::get(),
        backend_hint: BackendHint::Auto,
        start: vec![0; 5],
        end: vec![255, 15, 0, 0, 0],
        data: vec![[0, 0], [1, 1]],
        expected: vec![[0, 0], [1, 1]],
    }
}
