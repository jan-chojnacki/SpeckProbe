use crate::runtime::api::CipherMode;
use thiserror::Error;

#[derive(Debug, Error, Eq, PartialEq)]
pub enum SPECKError {
    /// Key byte length does not match the requirement for the chosen SPECK variant.
    #[error("expected {expected} bytes, got {got}")]
    InvalidKeyLength { expected: usize, got: usize },

    /// IV byte length does not match the block size of the chosen SPECK variant.
    #[error("expected {expected} bytes, got {got}")]
    InvalidIvLength { expected: usize, got: usize },

    /// CBC mode requires an IV but none was provided.
    #[error("expected iv when using {mode}")]
    IvMissing { mode: CipherMode },

    /// Ciphertext length is not a multiple of the block size and cannot be decrypted.
    #[error("expected data length to be a multiple of {expected_multiple}, got {got}")]
    InvalidDataLength {
        expected_multiple: usize,
        got: usize,
    },
}
