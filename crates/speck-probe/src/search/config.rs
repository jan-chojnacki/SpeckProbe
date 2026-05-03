use super::serde::deserialize_from_hex;
use super::serde::deserialize_u64_pairs;
use super::serde::serialize_as_hex;
use super::serde::serialize_u64_pairs;
use crate::speck::SpeckVersion;
use crate::search::executor::{BackendHint, CipherFunction, CipherMode};
use serde::{Deserialize, Serialize};

/// Configuration for a key-search operation.
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchConfig {
    pub cipher_mode: CipherMode,
    pub speck_version: SpeckVersion,
    pub cipher_function: CipherFunction,
    /// Number of low-order suffix bytes appended to every candidate key before testing.
    pub suffix_bytes_size: usize,
    pub num_threads: usize,
    pub backend_hint: BackendHint,
    /// Inclusive lower bound of the key search range, serialized as space-separated hex.
    #[serde(
        serialize_with = "serialize_as_hex",
        deserialize_with = "deserialize_from_hex"
    )]
    pub start: Vec<u8>,
    /// Inclusive upper bound of the key search range, serialized as space-separated hex.
    #[serde(
        serialize_with = "serialize_as_hex",
        deserialize_with = "deserialize_from_hex"
    )]
    pub end: Vec<u8>,
    /// Known plaintext/ciphertext pairs used to verify candidate keys.
    #[serde(
        serialize_with = "serialize_u64_pairs",
        deserialize_with = "deserialize_u64_pairs"
    )]
    pub data: Vec<[u64; 2]>,
    /// Expected output pairs that a correct key must produce.
    #[serde(
        serialize_with = "serialize_u64_pairs",
        deserialize_with = "deserialize_u64_pairs"
    )]
    pub expected: Vec<[u64; 2]>,
}

pub fn sample() -> SearchConfig {
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
