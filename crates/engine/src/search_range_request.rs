use crate::block::{Block, BlockError};
use crate::speck_version::SpeckVersion;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RequestError {
    #[error("invalid data length {source}")]
    InvalidDataLength {
        #[source]
        source: BlockError,
    },

    #[error("invalid expected length {source}")]
    InvalidExpectedLength {
        #[source]
        source: BlockError,
    },
}

pub struct SearchRangeRequest {
    pub speck_version: SpeckVersion,
    pub start_key: u64,
    pub key_count: u64,
    pub prefix: Vec<u8>,
    pub data_bytes: Block,
    pub expected_bytes: Block,
}
