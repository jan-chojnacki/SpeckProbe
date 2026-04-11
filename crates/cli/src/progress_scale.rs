use crate::helpers::{le_to_u256, significant_bytes};
use primitive_types::U256;

pub struct ProgressScale {
    bytes: u32,
}

impl ProgressScale {
    pub fn from_range(start: &[u8], end: &[u8]) -> (Self, u64) {
        let start = le_to_u256(start);
        let end = le_to_u256(end);
        let count = end - start;
        let pb_len = count.try_into().unwrap_or(u64::MAX);
        let bytes = significant_bytes(count);
        (Self { bytes }, pb_len)
    }

    pub fn scale(&self, state: U256) -> u64 {
        let shift = (8 * self.bytes).saturating_sub(64) as usize;
        (state >> shift).low_u64()
    }
}
