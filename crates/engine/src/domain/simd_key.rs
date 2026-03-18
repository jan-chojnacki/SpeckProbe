#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SimdKey<const T: usize> {
    bytes: [[u8; 32]; T],
    len: usize,
    prefix_len: usize,
}

impl<const T: usize> SimdKey<T> {
    pub fn new(prefix: &[u8], prefix_len: usize, v: [u64; T]) -> Self {
        debug_assert!(prefix.len() <= 24);
        let mut bytes = [[0u8; 32]; T];

        let p = prefix_len;
        let len = p + 8;

        for i in 0..T {
            bytes[i][..p].copy_from_slice(prefix);
            bytes[i][p..len].copy_from_slice(&v[i].to_le_bytes());
        }

        let prefix_len = p; //TODO

        Self {
            bytes,
            len,
            prefix_len,
        }
    }

    pub fn update(&mut self, v: [u64; T]) {
        let p = self.prefix_len;
        let len = self.len;

        for i in 0..T {
            self.bytes[i][p..len].copy_from_slice(&v[i].to_le_bytes());
        }
    }

    pub fn as_bytes(&self) -> [&[u8]; T] {
        self.bytes.each_ref().map(|b| &b[..self.len])
    }

    pub fn to_vec(&self) -> [Vec<u8>; T] {
        self.as_bytes().map(|b| b.to_vec())
    }

    pub fn as_u16x4_le(&self) -> [[[u8; 2]; T]; 4] {
        debug_assert_eq!(self.len, 8);

        let b = &self.bytes;

        let mut result = [[[0u8; 2]; T]; 4];

        for t in 0..T {
            result[0][t] = [b[t][0], b[t][1]];
            result[1][t] = [b[t][2], b[t][3]];
            result[2][t] = [b[t][4], b[t][5]];
            result[3][t] = [b[t][6], b[t][7]];
        }

        result
    }

    pub fn as_u24x3_le(&self) -> [[[u8; 4]; T]; 3] {
        debug_assert_eq!(self.len, 9);

        let b = &self.bytes;

        let mut result = [[[0u8; 4]; T]; 3];

        for t in 0..T {
            result[0][t] = [b[t][0], b[t][1], b[t][2], 0];
            result[1][t] = [b[t][3], b[t][4], b[t][5], 0];
            result[2][t] = [b[t][6], b[t][7], b[t][8], 0];
        }

        result
    }

    pub fn as_u24x4_le(&self) -> [[[u8; 4]; T]; 4] {
        debug_assert_eq!(self.len, 12);

        let b = &self.bytes;

        let mut result = [[[0u8; 4]; T]; 4];

        for t in 0..T {
            result[0][t] = [b[t][0], b[t][1], b[t][2], 0];
            result[1][t] = [b[t][3], b[t][4], b[t][5], 0];
            result[2][t] = [b[t][6], b[t][7], b[t][8], 0];
            result[3][t] = [b[t][9], b[t][10], b[t][11], 0];
        }

        result
    }

    pub fn as_u32x3_le(&self) -> [[[u8; 4]; T]; 3] {
        debug_assert_eq!(self.len, 12);

        let b = &self.bytes;

        let mut result = [[[0u8; 4]; T]; 3];

        for t in 0..T {
            result[0][t] = [b[t][0], b[t][1], b[t][2], b[t][3]];
            result[1][t] = [b[t][4], b[t][5], b[t][6], b[t][7]];
            result[2][t] = [b[t][8], b[t][9], b[t][10], b[t][11]];
        }

        result
    }

    pub fn as_u32x4_le(&self) -> [[[u8; 4]; T]; 4] {
        debug_assert_eq!(self.len, 16);

        let b = &self.bytes;

        let mut result = [[[0u8; 4]; T]; 4];

        for t in 0..T {
            result[0][t] = [b[t][0], b[t][1], b[t][2], b[t][3]];
            result[1][t] = [b[t][4], b[t][5], b[t][6], b[t][7]];
            result[2][t] = [b[t][8], b[t][9], b[t][10], b[t][11]];
            result[3][t] = [b[t][12], b[t][13], b[t][14], b[t][15]];
        }

        result
    }

    pub fn as_u48x2_le(&self) -> [[[u8; 8]; T]; 2] {
        debug_assert_eq!(self.len, 12);

        let b = &self.bytes;

        let mut result = [[[0u8; 8]; T]; 2];

        for t in 0..T {
            result[0][t] = [b[t][0], b[t][1], b[t][2], b[t][3], b[t][4], b[t][5], 0, 0];
            result[1][t] = [b[t][6], b[t][7], b[t][8], b[t][9], b[t][10], b[t][11], 0, 0];
        }

        result
    }

    pub fn as_u48x3_le(&self) -> [[[u8; 8]; T]; 3] {
        debug_assert_eq!(self.len, 18);

        let b = &self.bytes;

        let mut result = [[[0u8; 8]; T]; 3];

        for t in 0..T {
            result[0][t] = [b[t][0], b[t][1], b[t][2], b[t][3], b[t][4], b[t][5], 0, 0];
            result[1][t] = [b[t][6], b[t][7], b[t][8], b[t][9], b[t][10], b[t][11], 0, 0];
            result[2][t] = [
                b[t][12], b[t][13], b[t][14], b[t][15], b[t][16], b[t][17], 0, 0,
            ];
        }

        result
    }

    pub fn as_u64x2_le(&self) -> [[[u8; 8]; T]; 2] {
        debug_assert_eq!(self.len, 16);

        let b = &self.bytes;

        let mut result = [[[0u8; 8]; T]; 2];

        for t in 0..T {
            result[0][t] = [
                b[t][0], b[t][1], b[t][2], b[t][3], b[t][4], b[t][5], b[t][6], b[t][7],
            ];
            result[1][t] = [
                b[t][8], b[t][9], b[t][10], b[t][11], b[t][12], b[t][13], b[t][14], b[t][15],
            ];
        }

        result
    }

    pub fn as_u64x3_le(&self) -> [[[u8; 8]; T]; 3] {
        debug_assert_eq!(self.len, 24);

        let b = &self.bytes;

        let mut result = [[[0u8; 8]; T]; 3];

        for t in 0..T {
            result[0][t] = [
                b[t][0], b[t][1], b[t][2], b[t][3], b[t][4], b[t][5], b[t][6], b[t][7],
            ];
            result[1][t] = [
                b[t][8], b[t][9], b[t][10], b[t][11], b[t][12], b[t][13], b[t][14], b[t][15],
            ];
            result[2][t] = [
                b[t][16], b[t][17], b[t][18], b[t][19], b[t][20], b[t][21], b[t][22], b[t][23],
            ];
        }

        result
    }

    pub fn as_u64x4_le(&self) -> [[[u8; 8]; T]; 4] {
        debug_assert_eq!(self.len, 32);

        let b = &self.bytes;

        let mut result = [[[0u8; 8]; T]; 4];

        for t in 0..T {
            result[0][t] = [
                b[t][0], b[t][1], b[t][2], b[t][3], b[t][4], b[t][5], b[t][6], b[t][7],
            ];
            result[1][t] = [
                b[t][8], b[t][9], b[t][10], b[t][11], b[t][12], b[t][13], b[t][14], b[t][15],
            ];
            result[2][t] = [
                b[t][16], b[t][17], b[t][18], b[t][19], b[t][20], b[t][21], b[t][22], b[t][23],
            ];
            result[3][t] = [
                b[t][24], b[t][25], b[t][26], b[t][27], b[t][28], b[t][29], b[t][30], b[t][31],
            ];
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::key::Key;
    use rstest::rstest;

    fn key(bytes: &[u8]) -> SimdKey<1> {
        assert!(bytes.len() >= 8);
        assert!(bytes.len() <= 32);

        let prefix_len = bytes.len() - 8;
        let prefix = &bytes[..prefix_len];
        let value = u64::from_le_bytes(bytes[prefix_len..].try_into().unwrap());

        SimdKey::new(prefix, prefix_len, [value])
    }

    fn broadcast_key<const T: usize>(bytes: &[u8]) -> SimdKey<T> {
        assert!(bytes.len() >= 8);
        assert!(bytes.len() <= 32);

        let prefix_len = bytes.len() - 8;
        let prefix = &bytes[..prefix_len];
        let value = u64::from_le_bytes(bytes[prefix_len..].try_into().unwrap());

        SimdKey::new(prefix, prefix_len, [value; T])
    }

    fn scalar_key(bytes: &[u8]) -> Key {
        assert!(bytes.len() >= 8);
        assert!(bytes.len() <= 32);

        let prefix_len = bytes.len() - 8;
        let prefix = &bytes[..prefix_len];
        let value = u64::from_le_bytes(bytes[prefix_len..].try_into().unwrap());

        Key::new(prefix, prefix_len, value)
    }

    fn assert_lane_matches_scalar<const T: usize>(simd: &SimdKey<T>, lane: usize, scalar: &Key) {
        assert_eq!(simd.as_bytes()[lane], scalar.as_bytes());
        assert_eq!(simd.to_vec()[lane], scalar.to_vec());
        match scalar.as_bytes().len() {
            8 => {
                let parsed = simd.as_u16x4_le();
                let lane_values = [
                    u16::from_le_bytes(parsed[0][lane]),
                    u16::from_le_bytes(parsed[1][lane]),
                    u16::from_le_bytes(parsed[2][lane]),
                    u16::from_le_bytes(parsed[3][lane]),
                ];
                assert_eq!(lane_values, scalar.as_u16x4_le());
            }
            9 => {
                let parsed = simd.as_u24x3_le();
                let lane_values = [
                    u32::from_le_bytes(parsed[0][lane]),
                    u32::from_le_bytes(parsed[1][lane]),
                    u32::from_le_bytes(parsed[2][lane]),
                ];
                assert_eq!(lane_values, scalar.as_u24x3_le());
            }
            12 => {
                let parsed_u24 = simd.as_u24x4_le();
                let parsed_u32 = simd.as_u32x3_le();
                let parsed_u48 = simd.as_u48x2_le();
                let lane_u24 = [
                    u32::from_le_bytes(parsed_u24[0][lane]),
                    u32::from_le_bytes(parsed_u24[1][lane]),
                    u32::from_le_bytes(parsed_u24[2][lane]),
                    u32::from_le_bytes(parsed_u24[3][lane]),
                ];
                let lane_u32 = [
                    u32::from_le_bytes(parsed_u32[0][lane]),
                    u32::from_le_bytes(parsed_u32[1][lane]),
                    u32::from_le_bytes(parsed_u32[2][lane]),
                ];
                let lane_u48 = [
                    u64::from_le_bytes(parsed_u48[0][lane]),
                    u64::from_le_bytes(parsed_u48[1][lane]),
                ];
                assert_eq!(lane_u24, scalar.as_u24x4_le());
                assert_eq!(lane_u32, scalar.as_u32x3_le());
                assert_eq!(lane_u48, scalar.as_u48x2_le());
            }
            16 => {
                let parsed_u32 = simd.as_u32x4_le();
                let parsed_u64 = simd.as_u64x2_le();
                let lane_u32 = [
                    u32::from_le_bytes(parsed_u32[0][lane]),
                    u32::from_le_bytes(parsed_u32[1][lane]),
                    u32::from_le_bytes(parsed_u32[2][lane]),
                    u32::from_le_bytes(parsed_u32[3][lane]),
                ];
                let lane_u64 = [
                    u64::from_le_bytes(parsed_u64[0][lane]),
                    u64::from_le_bytes(parsed_u64[1][lane]),
                ];
                assert_eq!(lane_u32, scalar.as_u32x4_le());
                assert_eq!(lane_u64, scalar.as_u64x2_le());
            }
            18 => {
                let parsed = simd.as_u48x3_le();
                let lane_values = [
                    u64::from_le_bytes(parsed[0][lane]),
                    u64::from_le_bytes(parsed[1][lane]),
                    u64::from_le_bytes(parsed[2][lane]),
                ];
                assert_eq!(lane_values, scalar.as_u48x3_le());
            }
            24 => {
                let parsed = simd.as_u64x3_le();
                let lane_values = [
                    u64::from_le_bytes(parsed[0][lane]),
                    u64::from_le_bytes(parsed[1][lane]),
                    u64::from_le_bytes(parsed[2][lane]),
                ];
                assert_eq!(lane_values, scalar.as_u64x3_le());
            }
            32 => {
                let parsed = simd.as_u64x4_le();
                let lane_values = [
                    u64::from_le_bytes(parsed[0][lane]),
                    u64::from_le_bytes(parsed[1][lane]),
                    u64::from_le_bytes(parsed[2][lane]),
                    u64::from_le_bytes(parsed[3][lane]),
                ];
                assert_eq!(lane_values, scalar.as_u64x4_le());
            }
            _ => panic!("unsupported test key length"),
        }
    }

    fn assert_all_lanes_match_scalar<const T: usize>(simd: &SimdKey<T>, scalar: &Key) {
        for lane in 0..T {
            assert_lane_matches_scalar(simd, lane, scalar);
        }
    }

    #[rstest]
    #[case(vec![0x34, 0x12, 0x78, 0x56, 0xBC, 0x9A, 0xF0, 0xDE])]
    #[case(vec![0x03, 0x02, 0x01, 0x06, 0x05, 0x04, 0x09, 0x08, 0x07])]
    #[case(vec![0x03, 0x02, 0x01, 0x06, 0x05, 0x04, 0x09, 0x08, 0x07, 0x0C, 0x0B, 0x0A])]
    #[case(vec![0x04, 0x03, 0x02, 0x01, 0x08, 0x07, 0x06, 0x05, 0x0C, 0x0B, 0x0A, 0x09, 0x10, 0x0F, 0x0E, 0x0D])]
    #[case(vec![
        0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x0C, 0x0B, 0x0A, 0x09, 0x08, 0x07, 0x12, 0x11,
        0x10, 0x0F, 0x0E, 0x0D,
    ])]
    #[case(vec![
        0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x10, 0x0F, 0x0E, 0x0D, 0x0C, 0x0B,
        0x0A, 0x09, 0x18, 0x17, 0x16, 0x15, 0x14, 0x13, 0x12, 0x11,
    ])]
    #[case(vec![
        0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x10, 0x0F, 0x0E, 0x0D, 0x0C, 0x0B,
        0x0A, 0x09, 0x18, 0x17, 0x16, 0x15, 0x14, 0x13, 0x12, 0x11, 0x20, 0x1F, 0x1E, 0x1D,
        0x1C, 0x1B, 0x1A, 0x19,
    ])]
    fn simd_key_lane0_matches_key_views(#[case] bytes: Vec<u8>) {
        let simd = key(&bytes);
        let scalar = scalar_key(&bytes);

        assert_all_lanes_match_scalar(&simd, &scalar);
    }

    #[rstest]
    #[case(vec![0x34, 0x12, 0x78, 0x56, 0xBC, 0x9A, 0xF0, 0xDE])]
    #[case(vec![0x03, 0x02, 0x01, 0x06, 0x05, 0x04, 0x09, 0x08, 0x07])]
    #[case(vec![0x03, 0x02, 0x01, 0x06, 0x05, 0x04, 0x09, 0x08, 0x07, 0x0C, 0x0B, 0x0A])]
    #[case(vec![0x04, 0x03, 0x02, 0x01, 0x08, 0x07, 0x06, 0x05, 0x0C, 0x0B, 0x0A, 0x09, 0x10, 0x0F, 0x0E, 0x0D])]
    #[case(vec![
        0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x0C, 0x0B, 0x0A, 0x09, 0x08, 0x07, 0x12, 0x11,
        0x10, 0x0F, 0x0E, 0x0D,
    ])]
    #[case(vec![
        0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x10, 0x0F, 0x0E, 0x0D, 0x0C, 0x0B,
        0x0A, 0x09, 0x18, 0x17, 0x16, 0x15, 0x14, 0x13, 0x12, 0x11,
    ])]
    #[case(vec![
        0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x10, 0x0F, 0x0E, 0x0D, 0x0C, 0x0B,
        0x0A, 0x09, 0x18, 0x17, 0x16, 0x15, 0x14, 0x13, 0x12, 0x11, 0x20, 0x1F, 0x1E, 0x1D,
        0x1C, 0x1B, 0x1A, 0x19,
    ])]
    fn simd_key_broadcast_matches_key_on_all_lanes(#[case] bytes: Vec<u8>) {
        let simd = broadcast_key::<4>(&bytes);
        let scalar = scalar_key(&bytes);

        assert_all_lanes_match_scalar(&simd, &scalar);
    }

    #[rstest]
    #[case(
        vec![0xAA, 0xBB, 0xCC, 0xDD],
        [
            0x0102030405060708u64,
            0x1112131415161718u64,
            0xFFEEDDCCBBAA9988u64,
            0x8877665544332211u64,
        ]
    )]
    #[case(
        vec![],
        [
            0x0000000000000000u64,
            0x0001020304050607u64,
            0xD9E8F7A6B5C4D3E2u64,
            0xFFFFFFFFFFFFFFFFu64,
        ]
    )]
    #[case(
        vec![0x5A; 24],
        [
            0x0101010101010101u64,
            0x1234567890ABCDEFu64,
            0x0F1E2D3C4B5A6978u64,
            0x8877665544332211u64,
        ]
    )]
    fn simd_key_multi_lane_different_values_matches_scalar_per_lane(
        #[case] prefix: Vec<u8>,
        #[case] values: [u64; 4],
    ) {
        let simd = SimdKey::<4>::new(&prefix, prefix.len(), values);
        let scalars = values.map(|v| Key::new(&prefix, prefix.len(), v));

        for (lane, scalar) in scalars.iter().enumerate() {
            assert_lane_matches_scalar(&simd, lane, scalar);
        }
    }
}
