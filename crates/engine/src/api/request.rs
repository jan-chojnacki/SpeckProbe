use crate::api::version::SpeckVersion;
use crate::domain::block::Block;

pub enum Operation {
    Encrypt,
    Decrypt,
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