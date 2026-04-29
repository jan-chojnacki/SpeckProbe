/// Reads a little-endian `u16` from the first 2 bytes of `b`.
pub(crate) fn read_u16_le(b: &[u8]) -> u16 {
    u16::from_le_bytes([b[0], b[1]])
}

/// Writes a little-endian `u16` to `out`.
pub(crate) fn write_u16_le(v: u16, out: &mut Vec<u8>) {
    out.extend_from_slice(&v.to_le_bytes());
}

/// Reads a little-endian 24-bit value from the first 3 bytes of `b` into a `u32`.
pub(crate) fn read_u24_le(b: &[u8]) -> u32 {
    u32::from_le_bytes([b[0], b[1], b[2], 0])
}

/// Writes the low 3 bytes of `v` as a little-endian 24-bit value to `out`.
pub(crate) fn write_u24_le(v: u32, out: &mut Vec<u8>) {
    out.extend_from_slice(&v.to_le_bytes()[..3]);
}

/// Reads a little-endian `u32` from the first 4 bytes of `b`.
pub(crate) fn read_u32_le(b: &[u8]) -> u32 {
    u32::from_le_bytes([b[0], b[1], b[2], b[3]])
}

/// Writes a little-endian `u32` to `out`.
pub(crate) fn write_u32_le(v: u32, out: &mut Vec<u8>) {
    out.extend_from_slice(&v.to_le_bytes());
}

/// Reads a little-endian 48-bit value from the first 6 bytes of `b` into a `u64`.
pub(crate) fn read_u48_le(b: &[u8]) -> u64 {
    u64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], 0, 0])
}

/// Writes the low 6 bytes of `v` as a little-endian 48-bit value to `out`.
pub(crate) fn write_u48_le(v: u64, out: &mut Vec<u8>) {
    out.extend_from_slice(&v.to_le_bytes()[..6]);
}

/// Reads a little-endian `u64` from the first 8 bytes of `b`.
pub(crate) fn read_u64_le(b: &[u8]) -> u64 {
    u64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]])
}

/// Writes a little-endian `u64` to `out`.
pub(crate) fn write_u64_le(v: u64, out: &mut Vec<u8>) {
    out.extend_from_slice(&v.to_le_bytes());
}
