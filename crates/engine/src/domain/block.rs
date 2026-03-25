use speck::SpeckVersion;
use thiserror::Error;

#[derive(Debug, Clone, Error, Eq, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn block(bytes: &[u8]) -> Block {
        Block {
            bytes: bytes.to_vec(),
        }
    }

    #[rstest]
    #[case(SpeckVersion::Speck32_64, vec![0xAA, 0xBB, 0xCC, 0xDD])]
    #[case(SpeckVersion::Speck48_72, vec![0xAA; 6])]
    #[case(SpeckVersion::Speck64_96, vec![0xAA; 8])]
    #[case(SpeckVersion::Speck96_96, vec![0xAA; 12])]
    #[case(SpeckVersion::Speck128_128, vec![0xAA; 16])]
    fn new_accepts_correct_length_for_version(
        #[case] version: SpeckVersion,
        #[case] bytes: Vec<u8>,
    ) {
        let block = Block::new(&bytes, &version).unwrap();
        assert_eq!(block.as_bytes(), &bytes);
    }

    #[rstest]
    #[case(SpeckVersion::Speck32_64, vec![0xAA; 3], 4, 3)]
    #[case(SpeckVersion::Speck48_72, vec![0xAA; 5], 6, 5)]
    #[case(SpeckVersion::Speck64_96, vec![0xAA; 7], 8, 7)]
    #[case(SpeckVersion::Speck96_96, vec![0xAA; 11], 12, 11)]
    #[case(SpeckVersion::Speck128_128, vec![0xAA; 15], 16, 15)]
    fn new_rejects_invalid_length_for_version(
        #[case] version: SpeckVersion,
        #[case] bytes: Vec<u8>,
        #[case] expected: usize,
        #[case] got: usize,
    ) {
        let err = Block::new(&bytes, &version).unwrap_err();
        assert_eq!(err, BlockError::InvalidLength { expected, got });
    }

    #[rstest]
    #[case(block(&[0x34, 0x12, 0x78, 0x56]), [0x1234, 0x5678])]
    #[case(block(&[0x00, 0x00, 0xFF, 0xFF]), [0x0000, 0xFFFF])]
    #[case(block(&[0xCD, 0xAB, 0x34, 0x12]), [0xABCD, 0x1234])]
    fn as_u16x2_le_parses_values(#[case] block: Block, #[case] expected: [u16; 2]) {
        assert_eq!(block.as_u16x2_le().unwrap(), expected);
    }

    #[rstest]
    #[case(block(&[0x03, 0x02, 0x01, 0x06, 0x05, 0x04]), [0x010203, 0x040506])]
    #[case(block(&[0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF]), [0x000000, 0xFFFFFF])]
    #[case(block(&[0x56, 0x34, 0x12, 0xBC, 0x9A, 0x78]), [0x123456, 0x789ABC])]
    fn as_u24x2_le_parses_values(#[case] block: Block, #[case] expected: [u32; 2]) {
        assert_eq!(block.as_u24x2_le().unwrap(), expected);
    }

    #[rstest]
    #[case(block(&[0x04, 0x03, 0x02, 0x01, 0x08, 0x07, 0x06, 0x05]), [0x01020304, 0x05060708])]
    #[case(block(&[0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF]), [0x00000000, 0xFFFFFFFF])]
    #[case(block(&[0x67, 0x45, 0x23, 0x01, 0xEF, 0xCD, 0xAB, 0x89]), [0x01234567, 0x89ABCDEF])]
    fn as_u32x2_le_parses_values(#[case] block: Block, #[case] expected: [u32; 2]) {
        assert_eq!(block.as_u32x2_le().unwrap(), expected);
    }

    #[rstest]
    #[case(block(&[0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x0C, 0x0B, 0x0A, 0x09, 0x08, 0x07,]),
        [0x010203040506, 0x0708090A0B0C])]
    #[case(block(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,]),
        [0x000000000000, 0xFFFFFFFFFFFF])]
    #[case(block(&[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12,]),
        [0x567890123456, 0x123456789ABC])]
    fn as_u48x2_le_parses_values(#[case] block: Block, #[case] expected: [u64; 2]) {
        assert_eq!(block.as_u48x2_le().unwrap(), expected);
    }

    #[rstest]
    #[case(block(&[0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x10, 0x0F, 0x0E, 0x0D, 0x0C, 0x0B, 0x0A, 0x09,]),
        [0x0102030405060708, 0x090A0B0C0D0E0F10])]
    #[case(block(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,]),
        [0x0000000000000000, 0xFFFFFFFFFFFFFFFF])]
    #[case(block(&[0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x18, 0x17, 0x16, 0x15, 0x14, 0x13, 0x12, 0x11,]),
        [0x0102030405060708, 0x1112131415161718])]
    fn as_u64x2_le_parses_values(#[case] block: Block, #[case] expected: [u64; 2]) {
        assert_eq!(block.as_u64x2_le().unwrap(), expected);
    }

    #[rstest]
    #[case(block(&[0x00, 0x01, 0x02]), 4, 3)]
    fn as_u16x2_le_rejects_invalid_length(
        #[case] block: Block,
        #[case] expected: usize,
        #[case] got: usize,
    ) {
        assert_eq!(
            block.as_u16x2_le().unwrap_err(),
            BlockError::InvalidLength { expected, got }
        );
    }

    #[rstest]
    #[case(block(&[0x00; 5]), 6, 5)]
    fn as_u24x2_le_rejects_invalid_length(
        #[case] block: Block,
        #[case] expected: usize,
        #[case] got: usize,
    ) {
        assert_eq!(
            block.as_u24x2_le().unwrap_err(),
            BlockError::InvalidLength { expected, got }
        );
    }

    #[rstest]
    #[case(block(&[0x00; 7]), 8, 7)]
    fn as_u32x2_le_rejects_invalid_length(
        #[case] block: Block,
        #[case] expected: usize,
        #[case] got: usize,
    ) {
        assert_eq!(
            block.as_u32x2_le().unwrap_err(),
            BlockError::InvalidLength { expected, got }
        );
    }

    #[rstest]
    #[case(block(&[0x00; 11]), 12, 11)]
    fn as_u48x2_le_rejects_invalid_length(
        #[case] block: Block,
        #[case] expected: usize,
        #[case] got: usize,
    ) {
        assert_eq!(
            block.as_u48x2_le().unwrap_err(),
            BlockError::InvalidLength { expected, got }
        );
    }

    #[rstest]
    #[case(block(&[0x00; 15]), 16, 15)]
    fn as_u64x2_le_rejects_invalid_length(
        #[case] block: Block,
        #[case] expected: usize,
        #[case] got: usize,
    ) {
        assert_eq!(
            block.as_u64x2_le().unwrap_err(),
            BlockError::InvalidLength { expected, got }
        );
    }
}
