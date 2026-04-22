#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Key<const BYTES: usize, const PREFIX: usize> {
    bytes: [u8; BYTES],
}

impl<const BYTES: usize, const PREFIX: usize> Key<BYTES, PREFIX> {
    const SUFFIX: usize = BYTES - PREFIX;

    #[inline(always)]
    pub fn new(prefix: &[u8; PREFIX], v: u64) -> Self {
        let mut bytes = [0u8; BYTES];

        bytes[Self::SUFFIX..].copy_from_slice(prefix);
        let suffix = v.to_le_bytes();
        bytes[..Self::SUFFIX].copy_from_slice(&suffix[..Self::SUFFIX]);

        Self { bytes }
    }

    #[inline(always)]
    pub fn new_from_bytes(bytes: &[u8; BYTES]) -> Self {
        Self { bytes: *bytes }
    }

    #[inline(always)]
    pub fn update(&mut self, v: u64) {
        let suffix = v.to_le_bytes();
        self.bytes[..Self::SUFFIX].copy_from_slice(&suffix[..Self::SUFFIX]);
    }

    #[inline(always)]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    #[inline(always)]
    pub fn to_vec(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl<const PREFIX: usize> Key<8, PREFIX> {
    #[inline(always)]
    pub fn as_u16x4_le(&self) -> [u16; 4] {
        let b = &self.bytes;

        [
            u16::from_le_bytes([b[0], b[1]]),
            u16::from_le_bytes([b[2], b[3]]),
            u16::from_le_bytes([b[4], b[5]]),
            u16::from_le_bytes([b[6], b[7]]),
        ]
    }
}

impl<const PREFIX: usize> Key<9, PREFIX> {
    #[inline(always)]
    pub fn as_u24x3_le(&self) -> [u32; 3] {
        let b = &self.bytes;

        [
            u32::from_le_bytes([b[0], b[1], b[2], 0]),
            u32::from_le_bytes([b[3], b[4], b[5], 0]),
            u32::from_le_bytes([b[6], b[7], b[8], 0]),
        ]
    }
}

impl<const PREFIX: usize> Key<12, PREFIX> {
    #[inline(always)]
    pub fn as_u24x4_le(&self) -> [u32; 4] {
        let b = &self.bytes;

        [
            u32::from_le_bytes([b[0], b[1], b[2], 0]),
            u32::from_le_bytes([b[3], b[4], b[5], 0]),
            u32::from_le_bytes([b[6], b[7], b[8], 0]),
            u32::from_le_bytes([b[9], b[10], b[11], 0]),
        ]
    }

    #[inline(always)]
    pub fn as_u32x3_le(&self) -> [u32; 3] {
        let b = &self.bytes;

        [
            u32::from_le_bytes([b[0], b[1], b[2], b[3]]),
            u32::from_le_bytes([b[4], b[5], b[6], b[7]]),
            u32::from_le_bytes([b[8], b[9], b[10], b[11]]),
        ]
    }

    #[inline(always)]
    pub fn as_u48x2_le(&self) -> [u64; 2] {
        let b = &self.bytes;

        [
            u64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], 0, 0]),
            u64::from_le_bytes([b[6], b[7], b[8], b[9], b[10], b[11], 0, 0]),
        ]
    }
}
impl<const PREFIX: usize> Key<16, PREFIX> {
    #[inline(always)]
    pub fn as_u32x4_le(&self) -> [u32; 4] {
        let b = &self.bytes;

        [
            u32::from_le_bytes([b[0], b[1], b[2], b[3]]),
            u32::from_le_bytes([b[4], b[5], b[6], b[7]]),
            u32::from_le_bytes([b[8], b[9], b[10], b[11]]),
            u32::from_le_bytes([b[12], b[13], b[14], b[15]]),
        ]
    }

    #[inline(always)]
    pub fn as_u64x2_le(&self) -> [u64; 2] {
        let b = &self.bytes;

        [
            u64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]),
            u64::from_le_bytes([b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15]]),
        ]
    }
}

impl<const PREFIX: usize> Key<18, PREFIX> {
    #[inline(always)]
    pub fn as_u48x3_le(&self) -> [u64; 3] {
        let b = &self.bytes;

        [
            u64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], 0, 0]),
            u64::from_le_bytes([b[6], b[7], b[8], b[9], b[10], b[11], 0, 0]),
            u64::from_le_bytes([b[12], b[13], b[14], b[15], b[16], b[17], 0, 0]),
        ]
    }
}

impl<const PREFIX: usize> Key<24, PREFIX> {
    #[inline(always)]
    pub fn as_u64x3_le(&self) -> [u64; 3] {
        let b = &self.bytes;

        [
            u64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]),
            u64::from_le_bytes([b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15]]),
            u64::from_le_bytes([b[16], b[17], b[18], b[19], b[20], b[21], b[22], b[23]]),
        ]
    }
}
impl<const PREFIX: usize> Key<32, PREFIX> {
    #[inline(always)]
    pub fn as_u64x4_le(&self) -> [u64; 4] {
        let b = &self.bytes;

        [
            u64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]),
            u64::from_le_bytes([b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15]]),
            u64::from_le_bytes([b[16], b[17], b[18], b[19], b[20], b[21], b[22], b[23]]),
            u64::from_le_bytes([b[24], b[25], b[26], b[27], b[28], b[29], b[30], b[31]]),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::Key;
    use rstest::rstest;

    #[rstest]
    #[case(
        [0x18, 0x19, 0x10, 0x11, 0x08, 0x09, 0x00, 0x01],
        [0x1918u16, 0x1110, 0x0908, 0x0100]
    )]
    fn key_conversion_32_64(#[case] bytes: [u8; 8], #[case] expected: [u16; 4]) {
        let key: Key<8, 0> = Key::new_from_bytes(&bytes);
        assert_eq!(key.as_u16x4_le(), expected);
    }

    #[rstest]
    #[case(
        [0x10, 0x11, 0x12, 0x08, 0x09, 0x0a, 0x00, 0x01, 0x02],
        [0x121110u32, 0x0a0908, 0x020100]
    )]
    fn key_conversion_48_72(#[case] bytes: [u8; 9], #[case] expected: [u32; 3]) {
        let key: Key<9, 1> = Key::new_from_bytes(&bytes);
        assert_eq!(key.as_u24x3_le(), expected);
    }

    #[rstest]
    #[case(
        [0x18, 0x19, 0x1a, 0x10, 0x11, 0x12, 0x08, 0x09, 0x0a, 0x00, 0x01, 0x02],
        [0x1a1918u32, 0x121110, 0x0a0908, 0x020100]
    )]
    fn key_conversion_48_96(#[case] bytes: [u8; 12], #[case] expected: [u32; 4]) {
        let key: Key<12, 4> = Key::new_from_bytes(&bytes);
        assert_eq!(key.as_u24x4_le(), expected);
    }

    #[rstest]
    #[case(
        [0x10, 0x11, 0x12, 0x13, 0x08, 0x09, 0x0a, 0x0b, 0x00, 0x01, 0x02, 0x03],
        [0x13121110u32, 0x0b0a0908, 0x03020100]
    )]
    fn key_conversion_64_96(#[case] bytes: [u8; 12], #[case] expected: [u32; 3]) {
        let key: Key<12, 4> = Key::new_from_bytes(&bytes);
        assert_eq!(key.as_u32x3_le(), expected);
    }

    #[rstest]
    #[case(
        [0x18, 0x19, 0x1a, 0x1b, 0x10, 0x11, 0x12, 0x13, 0x08, 0x09, 0x0a, 0x0b, 0x00, 0x01, 0x02, 0x03],
        [0x1b1a1918u32, 0x13121110, 0x0b0a0908, 0x03020100]
    )]
    fn key_conversion_64_128(#[case] bytes: [u8; 16], #[case] expected: [u32; 4]) {
        let key: Key<16, 8> = Key::new_from_bytes(&bytes);
        assert_eq!(key.as_u32x4_le(), expected);
    }

    #[rstest]
    #[case(
        [0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05],
        [0x0d0c0b0a0908u64, 0x050403020100]
    )]
    fn key_conversion_96_96(#[case] bytes: [u8; 12], #[case] expected: [u64; 2]) {
        let key: Key<12, 4> = Key::new_from_bytes(&bytes);
        assert_eq!(key.as_u48x2_le(), expected);
    }

    #[rstest]
    #[case(
        [0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05],
        [0x151413121110u64, 0x0d0c0b0a0908, 0x050403020100]
    )]
    fn key_conversion_96_144(#[case] bytes: [u8; 18], #[case] expected: [u64; 3]) {
        let key: Key<18, 10> = Key::new_from_bytes(&bytes);
        assert_eq!(key.as_u48x3_le(), expected);
    }

    #[rstest]
    #[case(
        [0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07],
        [0x0f0e0d0c0b0a0908u64, 0x0706050403020100]
    )]
    fn key_conversion_128_128(#[case] bytes: [u8; 16], #[case] expected: [u64; 2]) {
        let key: Key<16, 8> = Key::new_from_bytes(&bytes);
        assert_eq!(key.as_u64x2_le(), expected);
    }

    #[rstest]
    #[case(
        [0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07],
        [0x1716151413121110u64, 0x0f0e0d0c0b0a0908, 0x0706050403020100]
    )]
    fn key_conversion_128_192(#[case] bytes: [u8; 24], #[case] expected: [u64; 3]) {
        let key: Key<24, 16> = Key::new_from_bytes(&bytes);
        assert_eq!(key.as_u64x3_le(), expected);
    }

    #[rstest]
    #[case(
        [0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07],
        [0x1f1e1d1c1b1a1918u64, 0x1716151413121110, 0x0f0e0d0c0b0a0908, 0x0706050403020100]
    )]
    fn key_conversion_128_256(#[case] bytes: [u8; 32], #[case] expected: [u64; 4]) {
        let key: Key<32, 24> = Key::new_from_bytes(&bytes);
        assert_eq!(key.as_u64x4_le(), expected);
    }
}
