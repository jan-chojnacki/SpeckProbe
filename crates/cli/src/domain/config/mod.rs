mod data_serde;
pub mod generate_sample;
pub mod io;
mod key_serde;

use crate::enums::{BackendHint, CipherFunction, CipherMode, SpeckVersion};
use data_serde::deserialize_u64_pairs;
use data_serde::serialize_u64_pairs;
use key_serde::deserialize_from_hex;
use key_serde::serialize_as_hex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub cipher_mode: CipherMode,
    pub speck_version: SpeckVersion,
    pub cipher_function: CipherFunction,
    pub suffix_bytes_size: usize,
    pub num_threads: usize,
    pub cap: usize,
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
