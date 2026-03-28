#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Key<const BYTES: usize, const PREFIX: usize> {
    bytes: [u8; BYTES],
}

impl<const BYTES: usize, const PREFIX: usize> Key<BYTES, PREFIX> {
    const SUFFIX: usize = BYTES - PREFIX;

    pub fn new(prefix: &[u8; PREFIX], v: u64) -> Self {
        let mut bytes = [0u8; BYTES];

        bytes[Self::SUFFIX..].copy_from_slice(prefix);
        let suffix = v.to_le_bytes();
        bytes[..Self::SUFFIX].copy_from_slice(&suffix[..Self::SUFFIX]);

        Self { bytes }
    }

    pub fn new_from_bytes(bytes: &[u8; BYTES]) -> Self {
        Self { bytes: *bytes }
    }

    pub fn update(&mut self, v: u64) {
        let suffix = v.to_le_bytes();
        self.bytes[..Self::SUFFIX].copy_from_slice(&suffix[..Self::SUFFIX]);
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl<const PREFIX: usize> Key<8, PREFIX> {
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
    pub fn as_u24x4_le(&self) -> [u32; 4] {
        let b = &self.bytes;

        [
            u32::from_le_bytes([b[0], b[1], b[2], 0]),
            u32::from_le_bytes([b[3], b[4], b[5], 0]),
            u32::from_le_bytes([b[6], b[7], b[8], 0]),
            u32::from_le_bytes([b[9], b[10], b[11], 0]),
        ]
    }

    pub fn as_u32x3_le(&self) -> [u32; 3] {
        let b = &self.bytes;

        [
            u32::from_le_bytes([b[0], b[1], b[2], b[3]]),
            u32::from_le_bytes([b[4], b[5], b[6], b[7]]),
            u32::from_le_bytes([b[8], b[9], b[10], b[11]]),
        ]
    }

    pub fn as_u48x2_le(&self) -> [u64; 2] {
        let b = &self.bytes;

        [
            u64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], 0, 0]),
            u64::from_le_bytes([b[6], b[7], b[8], b[9], b[10], b[11], 0, 0]),
        ]
    }
}
impl<const PREFIX: usize> Key<16, PREFIX> {
    pub fn as_u32x4_le(&self) -> [u32; 4] {
        let b = &self.bytes;

        [
            u32::from_le_bytes([b[0], b[1], b[2], b[3]]),
            u32::from_le_bytes([b[4], b[5], b[6], b[7]]),
            u32::from_le_bytes([b[8], b[9], b[10], b[11]]),
            u32::from_le_bytes([b[12], b[13], b[14], b[15]]),
        ]
    }

    pub fn as_u64x2_le(&self) -> [u64; 2] {
        let b = &self.bytes;

        [
            u64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]),
            u64::from_le_bytes([b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15]]),
        ]
    }
}

impl<const PREFIX: usize> Key<18, PREFIX> {
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
