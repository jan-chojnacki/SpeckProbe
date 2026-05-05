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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(&[0x34, 0x12], 0x1234u16)]
    #[case(&[0x00, 0x00], 0x0000u16)]
    #[case(&[0xFF, 0xFF], 0xFFFFu16)]
    fn read_u16_le_correct(#[case] bytes: &[u8], #[case] expected: u16) {
        assert_eq!(read_u16_le(bytes), expected);
    }

    #[rstest]
    #[case(0x1234u16, vec![0x34, 0x12])]
    #[case(0x0000u16, vec![0x00, 0x00])]
    #[case(0xFFFFu16, vec![0xFF, 0xFF])]
    fn write_u16_le_correct(#[case] value: u16, #[case] expected: Vec<u8>) {
        let mut out = Vec::new();
        write_u16_le(value, &mut out);
        assert_eq!(out, expected);
    }

    #[rstest]
    #[case(0x1234u16)]
    #[case(0x0000u16)]
    #[case(0xFFFFu16)]
    fn u16_roundtrip(#[case] value: u16) {
        let mut out = Vec::new();
        write_u16_le(value, &mut out);
        assert_eq!(read_u16_le(&out), value);
    }

    #[rstest]
    #[case(&[0xEF, 0xCD, 0xAB], 0x00ABCDEFu32)]
    #[case(&[0x00, 0x00, 0x00], 0x00000000u32)]
    #[case(&[0xFF, 0xFF, 0xFF], 0x00FFFFFFu32)]
    fn read_u24_le_correct(#[case] bytes: &[u8], #[case] expected: u32) {
        assert_eq!(read_u24_le(bytes), expected);
    }

    #[rstest]
    #[case(0x00ABCDEFu32, vec![0xEF, 0xCD, 0xAB])]
    #[case(0x00000000u32, vec![0x00, 0x00, 0x00])]
    #[case(0x00FFFFFFu32, vec![0xFF, 0xFF, 0xFF])]
    fn write_u24_le_correct(#[case] value: u32, #[case] expected: Vec<u8>) {
        let mut out = Vec::new();
        write_u24_le(value, &mut out);
        assert_eq!(out, expected);
    }

    #[rstest]
    #[case(0x00ABCDEFu32)]
    #[case(0x00000000u32)]
    #[case(0x00FFFFFFu32)]
    fn u24_roundtrip(#[case] value: u32) {
        let mut out = Vec::new();
        write_u24_le(value, &mut out);
        assert_eq!(read_u24_le(&out), value);
    }

    #[rstest]
    #[case(&[0x78, 0x56, 0x34, 0x12], 0x12345678u32)]
    #[case(&[0x00, 0x00, 0x00, 0x00], 0x00000000u32)]
    #[case(&[0xFF, 0xFF, 0xFF, 0xFF], 0xFFFFFFFFu32)]
    fn read_u32_le_correct(#[case] bytes: &[u8], #[case] expected: u32) {
        assert_eq!(read_u32_le(bytes), expected);
    }

    #[rstest]
    #[case(0x12345678u32, vec![0x78, 0x56, 0x34, 0x12])]
    #[case(0x00000000u32, vec![0x00, 0x00, 0x00, 0x00])]
    #[case(0xFFFFFFFFu32, vec![0xFF, 0xFF, 0xFF, 0xFF])]
    fn write_u32_le_correct(#[case] value: u32, #[case] expected: Vec<u8>) {
        let mut out = Vec::new();
        write_u32_le(value, &mut out);
        assert_eq!(out, expected);
    }

    #[rstest]
    #[case(0x12345678u32)]
    #[case(0x00000000u32)]
    #[case(0xFFFFFFFFu32)]
    fn u32_roundtrip(#[case] value: u32) {
        let mut out = Vec::new();
        write_u32_le(value, &mut out);
        assert_eq!(read_u32_le(&out), value);
    }

    #[rstest]
    #[case(&[0xFF, 0xEE, 0xDD, 0xCC, 0xBB, 0xAA], 0x0000AABBCCDDEEFFu64)]
    #[case(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00], 0x0000000000000000u64)]
    #[case(&[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF], 0x0000FFFFFFFFFFFFu64)]
    fn read_u48_le_correct(#[case] bytes: &[u8], #[case] expected: u64) {
        assert_eq!(read_u48_le(bytes), expected);
    }

    #[rstest]
    #[case(0x0000AABBCCDDEEFFu64, vec![0xFF, 0xEE, 0xDD, 0xCC, 0xBB, 0xAA])]
    #[case(0x0000000000000000u64, vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00])]
    #[case(0x0000FFFFFFFFFFFFu64, vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF])]
    fn write_u48_le_correct(#[case] value: u64, #[case] expected: Vec<u8>) {
        let mut out = Vec::new();
        write_u48_le(value, &mut out);
        assert_eq!(out, expected);
    }

    #[rstest]
    #[case(0x0000AABBCCDDEEFFu64)]
    #[case(0x0000000000000000u64)]
    #[case(0x0000FFFFFFFFFFFFu64)]
    fn u48_roundtrip(#[case] value: u64) {
        let mut out = Vec::new();
        write_u48_le(value, &mut out);
        assert_eq!(read_u48_le(&out), value);
    }

    #[rstest]
    #[case(&[0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01], 0x0102030405060708u64)]
    #[case(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], 0x0000000000000000u64)]
    #[case(&[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF], 0xFFFFFFFFFFFFFFFFu64)]
    fn read_u64_le_correct(#[case] bytes: &[u8], #[case] expected: u64) {
        assert_eq!(read_u64_le(bytes), expected);
    }

    #[rstest]
    #[case(0x0102030405060708u64, vec![0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01])]
    #[case(0x0000000000000000u64, vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])]
    #[case(0xFFFFFFFFFFFFFFFFu64, vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF])]
    fn write_u64_le_correct(#[case] value: u64, #[case] expected: Vec<u8>) {
        let mut out = Vec::new();
        write_u64_le(value, &mut out);
        assert_eq!(out, expected);
    }

    #[rstest]
    #[case(0x0102030405060708u64)]
    #[case(0x0000000000000000u64)]
    #[case(0xFFFFFFFFFFFFFFFFu64)]
    fn u64_roundtrip(#[case] value: u64) {
        let mut out = Vec::new();
        write_u64_le(value, &mut out);
        assert_eq!(read_u64_le(&out), value);
    }
}
