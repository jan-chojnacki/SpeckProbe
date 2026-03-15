use crate::speck_version::SpeckVersion;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BlockError {
    #[error("expected {expected} bytes, got {got}")]
    InvalidLength { expected: usize, got: usize },
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Block {
    bytes: Vec<u8>,
}

impl Block {
    pub fn new(bytes: &[u8], version: &SpeckVersion) -> Result<Self, BlockError> {
        let expected = version.block_size_bytes();

        if bytes.len() != expected {
            return Err(BlockError::InvalidLength {
                expected,
                got: bytes.len(),
            });
        }

        Ok(Self {
            bytes: Vec::from(bytes),
        })
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn as_u16x2_le(&self) -> Result<[u16; 2], BlockError> {
        const EXPECTED: usize = 4;

        if self.bytes.len() != EXPECTED {
            return Err(BlockError::InvalidLength {
                expected: EXPECTED,
                got: self.bytes.len(),
            });
        }

        Ok([
            u16::from_le_bytes([self.bytes[0], self.bytes[1]]),
            u16::from_le_bytes([self.bytes[2], self.bytes[3]]),
        ])
    }

    pub fn as_u24x2_le(&self) -> Result<[u32; 2], BlockError> {
        const EXPECTED: usize = 6;

        if self.bytes.len() != EXPECTED {
            return Err(BlockError::InvalidLength {
                expected: EXPECTED,
                got: self.bytes.len(),
            });
        }

        Ok([
            u32::from_le_bytes([self.bytes[0], self.bytes[1], self.bytes[2], 0]),
            u32::from_le_bytes([self.bytes[3], self.bytes[4], self.bytes[5], 0]),
        ])
    }

    pub fn as_u32x2_le(&self) -> Result<[u32; 2], BlockError> {
        const EXPECTED: usize = 8;

        if self.bytes.len() != EXPECTED {
            return Err(BlockError::InvalidLength {
                expected: EXPECTED,
                got: self.bytes.len(),
            });
        }

        Ok([
            u32::from_le_bytes([self.bytes[0], self.bytes[1], self.bytes[2], self.bytes[3]]),
            u32::from_le_bytes([self.bytes[4], self.bytes[5], self.bytes[6], self.bytes[7]]),
        ])
    }

    pub fn as_u48x2_le(&self) -> Result<[u64; 2], BlockError> {
        const EXPECTED: usize = 12;

        if self.bytes.len() != EXPECTED {
            return Err(BlockError::InvalidLength {
                expected: EXPECTED,
                got: self.bytes.len(),
            });
        }

        Ok([
            u64::from_le_bytes([
                self.bytes[0],
                self.bytes[1],
                self.bytes[2],
                self.bytes[3],
                self.bytes[4],
                self.bytes[5],
                0,
                0,
            ]),
            u64::from_le_bytes([
                self.bytes[6],
                self.bytes[7],
                self.bytes[8],
                self.bytes[9],
                self.bytes[10],
                self.bytes[11],
                0,
                0,
            ]),
        ])
    }

    pub fn as_u64x2_le(&self) -> Result<[u64; 2], BlockError> {
        const EXPECTED: usize = 16;

        if self.bytes.len() != EXPECTED {
            return Err(BlockError::InvalidLength {
                expected: EXPECTED,
                got: self.bytes.len(),
            });
        }

        Ok([
            u64::from_le_bytes([
                self.bytes[0],
                self.bytes[1],
                self.bytes[2],
                self.bytes[3],
                self.bytes[4],
                self.bytes[5],
                self.bytes[6],
                self.bytes[7],
            ]),
            u64::from_le_bytes([
                self.bytes[8],
                self.bytes[9],
                self.bytes[10],
                self.bytes[11],
                self.bytes[12],
                self.bytes[13],
                self.bytes[14],
                self.bytes[15],
            ]),
        ])
    }
}
