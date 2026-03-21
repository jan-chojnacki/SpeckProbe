use crate::api::version::SpeckVersion;
use crate::domain::block::Block;

#[derive(Debug, Copy, Clone, Eq, PartialEq, strum::Display)]
pub enum Operation {
    Encrypt,
    Decrypt,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SearchRangeRequest {
    pub speck_version: SpeckVersion,
    pub start_key: u64,
    pub key_count: u64,
    pub prefix: Vec<u8>,
    pub data_bytes: Block,
    pub expected_bytes: Block,
    pub operation: Operation,
}
