use crate::types::CipherOperationMode;
use thiserror::Error;

#[derive(Debug, Error, Eq, PartialEq)]
pub enum SPECKError {
    #[error("expected {expected} bytes, got {got}")]
    InvalidKeyLength { expected: usize, got: usize },
    #[error("expected {expected} bytes, got {got}")]
    InvalidIvLength { expected: usize, got: usize },
    #[error("expected iv when using {cipher_operation_mode}")]
    IvMissing {
        cipher_operation_mode: CipherOperationMode,
    },
    #[error("expected data length to be a multiple of {expected_multiple}, got {got}")]
    InvalidDataLength {
        expected_multiple: usize,
        got: usize,
    },
}
