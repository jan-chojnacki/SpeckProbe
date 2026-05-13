pub(crate) fn read_u16_le(b: &[u8]) -> u16 {
    u16::from_le_bytes([b[0], b[1]])
}

pub(crate) fn write_u16_le(v: u16, out: &mut Vec<u8>) {
    out.extend_from_slice(&v.to_le_bytes());
}

pub(crate) fn read_u24_le(b: &[u8]) -> u32 {
    u32::from_le_bytes([b[0], b[1], b[2], 0])
}

pub(crate) fn write_u24_le(v: u32, out: &mut Vec<u8>) {
    out.extend_from_slice(&v.to_le_bytes()[..3]);
}

pub(crate) fn read_u32_le(b: &[u8]) -> u32 {
    u32::from_le_bytes([b[0], b[1], b[2], b[3]])
}

pub(crate) fn write_u32_le(v: u32, out: &mut Vec<u8>) {
    out.extend_from_slice(&v.to_le_bytes());
}

pub(crate) fn read_u48_le(b: &[u8]) -> u64 {
    u64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], 0, 0])
}

pub(crate) fn write_u48_le(v: u64, out: &mut Vec<u8>) {
    out.extend_from_slice(&v.to_le_bytes()[..6]);
}

pub(crate) fn read_u64_le(b: &[u8]) -> u64 {
    u64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]])
}

pub(crate) fn write_u64_le(v: u64, out: &mut Vec<u8>) {
    out.extend_from_slice(&v.to_le_bytes());
}
