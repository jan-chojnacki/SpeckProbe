use crate::domain::block::Block;
use speck::SpeckVersion;

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

impl SearchRangeRequest {
    pub fn new(
        speck_version: SpeckVersion,
        start_key: u64,
        key_count: u64,
        prefix: Vec<u8>,
        data_bytes: Block,
        expected_bytes: Block,
        operation: Operation,
    ) -> Self {
        Self {
            speck_version,
            start_key,
            key_count,
            prefix,
            data_bytes,
            expected_bytes,
            operation,
        }
    }
}
