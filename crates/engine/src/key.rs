#[derive(Debug, Clone, Eq, PartialEq)]
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
