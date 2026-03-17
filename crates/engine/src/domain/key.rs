#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Key {
    bytes: [u8; 32],
    len: usize,
    prefix_len: usize,
}

impl Key {
    pub fn new(prefix: &[u8], prefix_len: usize, v: u64) -> Self {
        debug_assert!(prefix.len() <= 24);
        let mut bytes = [0u8; 32];

        let p = prefix_len;
        bytes[..p].copy_from_slice(prefix);

        let len = p + 8;
        bytes[p..len].copy_from_slice(&v.to_le_bytes());

        let prefix_len = p;

        Self {
            bytes,
            len,
            prefix_len,
        }
    }

    pub fn update(&mut self, v: u64) {
        let p = self.prefix_len;
        let len = self.len;

        self.bytes[p..len].copy_from_slice(&v.to_le_bytes());
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.len]
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }

    pub fn as_u16x4_le(&self) -> [u16; 4] {
        debug_assert_eq!(self.len, 8);

        let b = &self.bytes;

        [
            u16::from_le_bytes([b[0], b[1]]),
            u16::from_le_bytes([b[2], b[3]]),
            u16::from_le_bytes([b[4], b[5]]),
            u16::from_le_bytes([b[6], b[7]]),
        ]
    }

    pub fn as_u24x3_le(&self) -> [u32; 3] {
        debug_assert_eq!(self.len, 9);

        let b = &self.bytes;

        [
            u32::from_le_bytes([b[0], b[1], b[2], 0]),
            u32::from_le_bytes([b[3], b[4], b[5], 0]),
            u32::from_le_bytes([b[6], b[7], b[8], 0]),
        ]
    }

    pub fn as_u24x4_le(&self) -> [u32; 4] {
        debug_assert_eq!(self.len, 12);

        let b = &self.bytes;

        [
            u32::from_le_bytes([b[0], b[1], b[2], 0]),
            u32::from_le_bytes([b[3], b[4], b[5], 0]),
            u32::from_le_bytes([b[6], b[7], b[8], 0]),
            u32::from_le_bytes([b[9], b[10], b[11], 0]),
        ]
    }

    pub fn as_u32x3_le(&self) -> [u32; 3] {
        debug_assert_eq!(self.len, 12);

        let b = &self.bytes;

        [
            u32::from_le_bytes([b[0], b[1], b[2], b[3]]),
            u32::from_le_bytes([b[4], b[5], b[6], b[7]]),
            u32::from_le_bytes([b[8], b[9], b[10], b[11]]),
        ]
    }

    pub fn as_u32x4_le(&self) -> [u32; 4] {
        debug_assert_eq!(self.len, 16);

        let b = &self.bytes;

        [
            u32::from_le_bytes([b[0], b[1], b[2], b[3]]),
            u32::from_le_bytes([b[4], b[5], b[6], b[7]]),
            u32::from_le_bytes([b[8], b[9], b[10], b[11]]),
            u32::from_le_bytes([b[12], b[13], b[14], b[15]]),
        ]
    }

    pub fn as_u48x2_le(&self) -> [u64; 2] {
        debug_assert_eq!(self.len, 12);

        let b = &self.bytes;

        [
            u64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], 0, 0]),
            u64::from_le_bytes([b[6], b[7], b[8], b[9], b[10], b[11], 0, 0]),
        ]
    }

    pub fn as_u48x3_le(&self) -> [u64; 3] {
        debug_assert_eq!(self.len, 18);

        let b = &self.bytes;

        [
            u64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], 0, 0]),
            u64::from_le_bytes([b[6], b[7], b[8], b[9], b[10], b[11], 0, 0]),
            u64::from_le_bytes([b[12], b[13], b[14], b[15], b[16], b[17], 0, 0]),
        ]
    }

    pub fn as_u64x2_le(&self) -> [u64; 2] {
        debug_assert_eq!(self.len, 16);

        let b = &self.bytes;

        [
            u64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]),
            u64::from_le_bytes([b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15]]),
        ]
    }

    pub fn as_u64x3_le(&self) -> [u64; 3] {
        debug_assert_eq!(self.len, 24);

        let b = &self.bytes;

        [
            u64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]),
            u64::from_le_bytes([b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15]]),
            u64::from_le_bytes([b[16], b[17], b[18], b[19], b[20], b[21], b[22], b[23]]),
        ]
    }

    pub fn as_u64x4_le(&self) -> [u64; 4] {
        debug_assert_eq!(self.len, 32);

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
    use super::*;
    use rstest::rstest;

    fn key(bytes: &[u8]) -> Key {
        assert!(bytes.len() >= 8);
        assert!(bytes.len() <= 32);

        let prefix_len = bytes.len() - 8;
        let prefix = &bytes[..prefix_len];
        let value = u64::from_le_bytes(bytes[prefix_len..].try_into().unwrap());

        Key::new(prefix, prefix_len, value)
    }

    #[rstest]
    #[case(vec![0xAA, 0xBB, 0xCC, 0xDD], 0x1122334455667788,
        vec![0xAA, 0xBB, 0xCC, 0xDD, 0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11])]
    #[case(vec![], 0x0807060504030201,
        vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08])]
    #[case(vec![0xA5; 24], 0x1817161514131211,
        vec![0xA5, 0xA5, 0xA5, 0xA5, 0xA5, 0xA5, 0xA5, 0xA5, 0xA5, 0xA5, 0xA5, 0xA5, 0xA5, 0xA5, 0xA5, 0xA5,
             0xA5, 0xA5, 0xA5, 0xA5, 0xA5, 0xA5, 0xA5, 0xA5, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18])]
    fn new_builds_key_from_prefix_and_value(
        #[case] prefix: Vec<u8>,
        #[case] value: u64,
        #[case] expected: Vec<u8>,
    ) {
        let key = Key::new(&prefix, prefix.len(), value);
        assert_eq!(key.as_bytes(), expected);
        assert_eq!(key.to_vec(), key.as_bytes().to_vec());
    }

    #[rstest]
    #[case(vec![0x10, 0x20, 0x30, 0x40], 0x0102030405060708, 0x1112131415161718,
        vec![0x10, 0x20, 0x30, 0x40, 0x18, 0x17, 0x16, 0x15, 0x14, 0x13, 0x12, 0x11])]
    #[case(vec![], 0x0000000000000000, 0x0807060504030201,
        vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08])]
    #[case(vec![0x7F; 24], 0x0102030405060708, 0xA8A7A6A5A4A3A2A1,
        vec![0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F,
             0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0x7F, 0xA1, 0xA2, 0xA3, 0xA4, 0xA5, 0xA6, 0xA7, 0xA8])]
    fn update_replaces_only_value_bytes(
        #[case] prefix: Vec<u8>,
        #[case] initial: u64,
        #[case] updated: u64,
        #[case] expected: Vec<u8>,
    ) {
        let mut key = Key::new(&prefix, prefix.len(), initial);
        key.update(updated);
        assert_eq!(key.as_bytes(), expected);
    }

    #[rstest]
    #[case(key(&[0x34, 0x12, 0x78, 0x56, 0xBC, 0x9A, 0xF0, 0xDE]), [0x1234, 0x5678, 0x9ABC, 0xDEF0])]
    #[case(key(&[0x00, 0x00, 0x01, 0x00, 0xFE, 0xFF, 0x10, 0x27]), [0x0000, 0x0001, 0xFFFE, 0x2710])]
    fn as_u16x4_le_parses_values(#[case] key: Key, #[case] expected: [u16; 4]) {
        assert_eq!(key.as_u16x4_le(), expected);
    }

    #[rstest]
    #[case(key(&[0x03, 0x02, 0x01, 0x06, 0x05, 0x04, 0x09, 0x08, 0x07]), [0x010203, 0x040506, 0x070809])]
    #[case(key(&[0x56, 0x34, 0x12, 0xBC, 0x9A, 0x78, 0x21, 0x43, 0x65]), [0x123456, 0x789ABC, 0x654321])]
    fn as_u24x3_le_parses_values(#[case] key: Key, #[case] expected: [u32; 3]) {
        assert_eq!(key.as_u24x3_le(), expected);
    }

    #[rstest]
    #[case(key(&[0x03, 0x02, 0x01, 0x06, 0x05, 0x04, 0x09, 0x08, 0x07, 0x0C, 0x0B, 0x0A]),
        [0x010203, 0x040506, 0x070809, 0x0A0B0C])]
    #[case(key(&[0x01, 0x00, 0x00, 0x34, 0x12, 0x00, 0xCD, 0xAB, 0x00, 0xFF, 0xEE, 0xDD]),
        [0x000001, 0x001234, 0x00ABCD, 0xDDEEFF])]
    fn as_u24x4_le_parses_values(#[case] key: Key, #[case] expected: [u32; 4]) {
        assert_eq!(key.as_u24x4_le(), expected);
    }

    #[rstest]
    #[case(key(&[0x04, 0x03, 0x02, 0x01, 0x08, 0x07, 0x06, 0x05, 0x0C, 0x0B, 0x0A, 0x09]),
        [0x01020304, 0x05060708, 0x090A0B0C])]
    #[case(key(&[0xFF, 0x00, 0x00, 0x00, 0x10, 0x32, 0x54, 0x76, 0x89, 0xAB, 0xCD, 0xEF]),
        [0x000000FF, 0x76543210, 0xEFCDAB89])]
    fn as_u32x3_le_parses_values(#[case] key: Key, #[case] expected: [u32; 3]) {
        assert_eq!(key.as_u32x3_le(), expected);
    }

    #[rstest]
    #[case(key(&[0x04, 0x03, 0x02, 0x01, 0x08, 0x07, 0x06, 0x05, 0x0C, 0x0B, 0x0A, 0x09, 0x10, 0x0F, 0x0E, 0x0D]),
        [0x01020304, 0x05060708, 0x090A0B0C, 0x0D0E0F10])]
    #[case(key(&[0x01, 0x00, 0x00, 0x80, 0xFF, 0xFF, 0xFF, 0x7F, 0x11, 0x22, 0x33, 0x44, 0x78, 0x56, 0x34, 0x12]),
        [0x80000001, 0x7FFFFFFF, 0x44332211, 0x12345678])]
    fn as_u32x4_le_parses_values(#[case] key: Key, #[case] expected: [u32; 4]) {
        assert_eq!(key.as_u32x4_le(), expected);
    }

    #[rstest]
    #[case(key(&[0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x0C, 0x0B, 0x0A, 0x09, 0x08, 0x07]),
        [0x010203040506, 0x0708090A0B0C])]
    #[case(key(&[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12]),
        [0x567890123456, 0x123456789ABC])]
    fn as_u48x2_le_parses_values(#[case] key: Key, #[case] expected: [u64; 2]) {
        assert_eq!(key.as_u48x2_le(), expected);
    }

    #[rstest]
    #[case(key(&[0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x0C, 0x0B, 0x0A,
                 0x09, 0x08, 0x07, 0x12, 0x11, 0x10, 0x0F, 0x0E, 0x0D,]),
        [0x010203040506, 0x0708090A0B0C, 0x0D0E0F101112])]
    #[case(key(&[0xFF, 0xEE, 0xDD, 0xCC, 0xBB, 0xAA, 0x01, 0x23, 0x45,
                 0x67, 0x89, 0xAB, 0x10, 0x32, 0x54, 0x76, 0x98, 0xBA,]),
        [0xAABBCCDDEEFF, 0xAB8967452301, 0xBA9876543210])]
    fn as_u48x3_le_parses_values(#[case] key: Key, #[case] expected: [u64; 3]) {
        assert_eq!(key.as_u48x3_le(), expected);
    }

    #[rstest]
    #[case(key(&[0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01,
                 0x10, 0x0F, 0x0E, 0x0D, 0x0C, 0x0B, 0x0A, 0x09,]),
        [0x0102030405060708, 0x090A0B0C0D0E0F10])]
    #[case(key(&[0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77,
                 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF,]),
        [0x7766554433221100, 0xFFEEDDCCBBAA9988])]
    fn as_u64x2_le_parses_values(#[case] key: Key, #[case] expected: [u64; 2]) {
        assert_eq!(key.as_u64x2_le(), expected);
    }

    #[rstest]
    #[case(key(&[0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x10, 0x0F, 0x0E, 0x0D,
                 0x0C, 0x0B, 0x0A, 0x09, 0x18, 0x17, 0x16, 0x15, 0x14, 0x13, 0x12, 0x11,]),
        [0x0102030405060708, 0x090A0B0C0D0E0F10, 0x1112131415161718])]
    #[case(key(&[0xAA, 0x00, 0xBB, 0x11, 0xCC, 0x22, 0xDD, 0x33, 0xEE, 0x44, 0xFF, 0x55,
                 0x10, 0x66, 0x20, 0x77, 0x30, 0x88, 0x40, 0x99, 0x50, 0xAA, 0x60, 0xBB,]),
        [0x33DD22CC11BB00AA, 0x7720661055FF44EE, 0xBB60AA5099408830])]
    fn as_u64x3_le_parses_values(#[case] key: Key, #[case] expected: [u64; 3]) {
        assert_eq!(key.as_u64x3_le(), expected);
    }

    #[rstest]
    #[case(key(&[0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x10, 0x0F, 0x0E, 0x0D, 0x0C, 0x0B, 0x0A, 0x09,
                 0x18, 0x17, 0x16, 0x15, 0x14, 0x13, 0x12, 0x11, 0x20, 0x1F, 0x1E, 0x1D, 0x1C, 0x1B, 0x1A, 0x19,]),
        [0x0102030405060708, 0x090A0B0C0D0E0F10, 0x1112131415161718, 0x191A1B1C1D1E1F20])]
    #[case(key(&[0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
                 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F,]),
        [0x0706050403020100, 0x0F0E0D0C0B0A0908, 0x1716151413121110, 0x1F1E1D1C1B1A1918])]
    fn as_u64x4_le_parses_values(#[case] key: Key, #[case] expected: [u64; 4]) {
        assert_eq!(key.as_u64x4_le(), expected);
    }
}
