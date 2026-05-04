use primitive_types::U256;

/// Maps an arbitrary-precision key-space count onto a `u64` progress bar length.
pub struct ProgressScale {
    bytes: u32,
}

impl ProgressScale {
    /// Computes a scale from the inclusive `[start, end]` key range.
    ///
    /// Returns the scale and the progress bar length (capped at `u64::MAX`).
    pub fn from_range(start: &[u8], end: &[u8]) -> (Self, u64) {
        let start = le_to_u256(start);
        let end = le_to_u256(end);
        let count = (end - start) + 1;
        let pb_len = count.try_into().unwrap_or(u64::MAX);
        let bytes = significant_bytes(count);
        (Self { bytes }, pb_len)
    }

    /// Scales `state` (accumulated progress) into a `u64` progress bar position.
    pub fn scale(&self, state: U256) -> u64 {
        let shift = (8 * self.bytes).saturating_sub(64) as usize;
        (state >> shift).low_u64()
    }
}

fn le_to_u256(bytes: &[u8]) -> U256 {
    let mut buf = [0u8; 32];
    buf[..bytes.len()].copy_from_slice(bytes);
    U256::from_little_endian(&buf)
}

fn significant_bytes(value: U256) -> u32 {
    let le = value.to_little_endian();
    le.iter()
        .rposition(|&b| b != 0)
        .map(|i| i as u32 + 1)
        .unwrap_or(1)
}
