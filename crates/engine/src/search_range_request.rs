use crate::block::{Block, BlockError};
use crate::speck_version::SpeckVersion;
use thiserror::Error;

pub enum Operation {
    Encrypt,
    Decrypt,
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum RequestError {
    #[error(transparent)]
    BlockError(#[from] BlockError),
}

pub struct SearchRangeRequest {
    pub speck_version: SpeckVersion,
    pub start_key: u64,
    pub key_count: u64,
    pub prefix: Vec<u8>,
    pub data_bytes: Block,
    pub expected_bytes: Block,
    pub operation: Operation,
}
