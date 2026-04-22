use crate::domain::key::Key;
use crate::domain::simd_key::SimdKey;
use speck::SpeckVersion;
use std::arch::x86_64::{__m128i, _mm_load_si128, _mm_setzero_si128};

#[repr(align(16))]
struct Align16<T>(T);

#[derive(Debug, Copy, Clone)]
#[repr(C, align(16))]
pub struct SSE2Key<const LANES: usize, const BYTES: usize, const PREFIX: usize> {
    bytes: [[u8; BYTES]; LANES],
    pa: __m128i,
    pb: __m128i,
    pc: __m128i,
}

impl<const LANES: usize, const BYTES: usize, const PREFIX: usize> SimdKey<LANES>
    for SSE2Key<LANES, BYTES, PREFIX>
{
    fn update(&mut self, v: [u64; LANES]) {
        self.update(v);
    }
}

impl<const LANES: usize, const BYTES: usize, const PREFIX: usize> SSE2Key<LANES, BYTES, PREFIX> {
    const SUFFIX: usize = BYTES - PREFIX;

    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "sse2")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `sse2` before calling this function."]
    pub fn new(prefix: &[u8], v: [u64; LANES], speck_version: SpeckVersion) -> Self {
        let mut bytes = [[0u8; BYTES]; LANES];

        for i in 0..LANES {
            bytes[i][Self::SUFFIX..].copy_from_slice(prefix);
            let suffix = v[i].to_le_bytes();
            bytes[i][..Self::SUFFIX].copy_from_slice(&suffix[..Self::SUFFIX]);
        }

        let mut pa = _mm_setzero_si128();
        let mut pb = _mm_setzero_si128();
        let mut pc = _mm_setzero_si128();

        match speck_version {
            SpeckVersion::Speck48_96 => {
                let a = Align16(bytes.map(|b| [b[0], b[1], b[2], 0]));
                unsafe {
                    pa = _mm_load_si128(a.0.as_ptr().cast());
                }
            }
            SpeckVersion::Speck64_96 => {
                let a = Align16(bytes.map(|b| [b[0], b[1], b[2], b[3]]));
                unsafe {
                    pa = _mm_load_si128(a.0.as_ptr().cast());
                }
            }
            SpeckVersion::Speck64_128 => {
                let a = Align16(bytes.map(|b| [b[0], b[1], b[2], b[3]]));
                let b = Align16(bytes.map(|b| [b[4], b[5], b[6], b[7]]));
                unsafe {
                    pa = _mm_load_si128(a.0.as_ptr().cast());
                    pb = _mm_load_si128(b.0.as_ptr().cast());
                }
            }
            SpeckVersion::Speck96_144 => {
                let a = Align16(bytes.map(|b| [b[0], b[1], b[2], b[3], b[4], b[5], 0, 0]));
                unsafe {
                    pa = _mm_load_si128(a.0.as_ptr().cast());
                }
            }
            SpeckVersion::Speck128_128 => {
                let a = Align16(bytes.map(|b| [b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]));
                unsafe {
                    pa = _mm_load_si128(a.0.as_ptr().cast());
                }
            }
            SpeckVersion::Speck128_192 => {
                let a = Align16(bytes.map(|b| [b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]));
                let b =
                    Align16(bytes.map(|b| [b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15]]));
                unsafe {
                    pa = _mm_load_si128(a.0.as_ptr().cast());
                    pb = _mm_load_si128(b.0.as_ptr().cast());
                }
            }
            SpeckVersion::Speck128_256 => {
                let a = Align16(bytes.map(|b| [b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]));
                let b =
                    Align16(bytes.map(|b| [b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15]]));
                let c = Align16(
                    bytes.map(|b| [b[16], b[17], b[18], b[19], b[20], b[21], b[22], b[23]]),
                );
                unsafe {
                    pa = _mm_load_si128(a.0.as_ptr().cast());
                    pb = _mm_load_si128(b.0.as_ptr().cast());
                    pc = _mm_load_si128(c.0.as_ptr().cast());
                }
            }
            _ => {}
        }

        Self { bytes, pa, pb, pc }
    }

    pub fn update(&mut self, v: [u64; LANES]) {
        for (i, v) in v.iter().enumerate().take(LANES) {
            let suffix = v.to_le_bytes();
            self.bytes[i][..Self::SUFFIX].copy_from_slice(&suffix[..Self::SUFFIX]);
        }
    }

    pub fn get(&self, i: usize) -> Key<BYTES, PREFIX> {
        let row = &self.bytes[i];
        Key::new_from_bytes(row)
    }

    pub fn as_bytes(&self) -> &[[u8; BYTES]; LANES] {
        &self.bytes
    }
    pub fn to_vec(&self) -> [Vec<u8>; LANES] {
        self.as_bytes().map(|b| b.to_vec())
    }
}

impl<const PREFIX: usize> SSE2Key<8, 8, PREFIX> {
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "sse2")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `sse2` before calling this function."]
    pub fn sse2_u16x4_key(&self) -> [__m128i; 4] {
        let a = Align16(self.bytes.map(|b| [b[0], b[1]]));
        let b = Align16(self.bytes.map(|b| [b[2], b[3]]));
        let c = Align16(self.bytes.map(|b| [b[4], b[5]]));
        let d = Align16(self.bytes.map(|b| [b[6], b[7]]));
        unsafe {
            [
                _mm_load_si128(a.0.as_ptr().cast()),
                _mm_load_si128(b.0.as_ptr().cast()),
                _mm_load_si128(c.0.as_ptr().cast()),
                _mm_load_si128(d.0.as_ptr().cast()),
            ]
        }
    }
}

impl<const PREFIX: usize> SSE2Key<4, 9, PREFIX> {
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "sse2")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `sse2` before calling this function."]
    pub fn sse2_u24x3_key(&self) -> [__m128i; 3] {
        let a = Align16(self.bytes.map(|b| [b[0], b[1], b[2], 0]));
        let b = Align16(self.bytes.map(|b| [b[3], b[4], b[5], 0]));
        let c = Align16(self.bytes.map(|b| [b[6], b[7], b[8], 0]));
        unsafe {
            [
                _mm_load_si128(a.0.as_ptr().cast()),
                _mm_load_si128(b.0.as_ptr().cast()),
                _mm_load_si128(c.0.as_ptr().cast()),
            ]
        }
    }
}

impl<const PREFIX: usize> SSE2Key<4, 12, PREFIX> {
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "sse2")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `sse2` before calling this function."]
    pub fn sse2_u24x4_key(&self) -> [__m128i; 4] {
        let a = Align16(self.bytes.map(|b| [b[0], b[1], b[2], 0]));
        let b = Align16(self.bytes.map(|b| [b[3], b[4], b[5], 0]));
        let c = Align16(self.bytes.map(|b| [b[6], b[7], b[8], 0]));
        let d = Align16(self.bytes.map(|b| [b[9], b[10], b[11], 0]));
        unsafe {
            [
                _mm_load_si128(a.0.as_ptr().cast()),
                _mm_load_si128(b.0.as_ptr().cast()),
                _mm_load_si128(c.0.as_ptr().cast()),
                _mm_load_si128(d.0.as_ptr().cast()),
            ]
        }
    }

    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "sse2")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `sse2` before calling this function."]
    pub fn sse2_u32x3_key(&self) -> [__m128i; 3] {
        let a = Align16(self.bytes.map(|b| [b[0], b[1], b[2], b[3]]));
        let b = Align16(self.bytes.map(|b| [b[4], b[5], b[6], b[7]]));
        let c = Align16(self.bytes.map(|b| [b[8], b[9], b[10], b[11]]));
        unsafe {
            [
                _mm_load_si128(a.0.as_ptr().cast()),
                _mm_load_si128(b.0.as_ptr().cast()),
                _mm_load_si128(c.0.as_ptr().cast()),
            ]
        }
    }
}

impl<const PREFIX: usize> SSE2Key<4, 16, PREFIX> {
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "sse2")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `sse2` before calling this function."]
    pub fn sse2_u32x4_key(&self) -> [__m128i; 4] {
        let a = Align16(self.bytes.map(|b| [b[0], b[1], b[2], b[3]]));
        let b = Align16(self.bytes.map(|b| [b[4], b[5], b[6], b[7]]));
        let c = Align16(self.bytes.map(|b| [b[8], b[9], b[10], b[11]]));
        let d = Align16(self.bytes.map(|b| [b[12], b[13], b[14], b[15]]));
        unsafe {
            [
                _mm_load_si128(a.0.as_ptr().cast()),
                _mm_load_si128(b.0.as_ptr().cast()),
                _mm_load_si128(c.0.as_ptr().cast()),
                _mm_load_si128(d.0.as_ptr().cast()),
            ]
        }
    }
}

impl<const PREFIX: usize> SSE2Key<2, 12, PREFIX> {
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "sse2")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `sse2` before calling this function."]
    pub fn sse2_u48x2_key(&self) -> [__m128i; 2] {
        let a = Align16(
            self.bytes
                .map(|b| [b[0], b[1], b[2], b[3], b[4], b[5], 0, 0]),
        );
        let b = Align16(
            self.bytes
                .map(|b| [b[6], b[7], b[8], b[9], b[10], b[11], 0, 0]),
        );
        unsafe {
            [
                _mm_load_si128(a.0.as_ptr().cast()),
                _mm_load_si128(b.0.as_ptr().cast()),
            ]
        }
    }
}

impl<const PREFIX: usize> SSE2Key<2, 18, PREFIX> {
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "sse2")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `sse2` before calling this function."]
    pub fn sse2_u48x3_key(&self) -> [__m128i; 3] {
        let a = Align16(
            self.bytes
                .map(|b| [b[0], b[1], b[2], b[3], b[4], b[5], 0, 0]),
        );
        let b = Align16(
            self.bytes
                .map(|b| [b[6], b[7], b[8], b[9], b[10], b[11], 0, 0]),
        );
        let c = Align16(
            self.bytes
                .map(|b| [b[12], b[13], b[14], b[15], b[16], b[17], 0, 0]),
        );
        unsafe {
            [
                _mm_load_si128(a.0.as_ptr().cast()),
                _mm_load_si128(b.0.as_ptr().cast()),
                _mm_load_si128(c.0.as_ptr().cast()),
            ]
        }
    }
}

impl<const PREFIX: usize> SSE2Key<2, 16, PREFIX> {
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "sse2")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `sse2` before calling this function."]
    pub fn sse2_u64x2_key(&self) -> [__m128i; 2] {
        let a = Align16(
            self.bytes
                .map(|b| [b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]),
        );
        let b = Align16(
            self.bytes
                .map(|b| [b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15]]),
        );
        unsafe {
            [
                _mm_load_si128(a.0.as_ptr().cast()),
                _mm_load_si128(b.0.as_ptr().cast()),
            ]
        }
    }
}

impl<const PREFIX: usize> SSE2Key<2, 24, PREFIX> {
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "sse2")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `sse2` before calling this function."]
    pub fn sse2_u64x3_key(&self) -> [__m128i; 3] {
        let a = Align16(
            self.bytes
                .map(|b| [b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]),
        );
        let b = Align16(
            self.bytes
                .map(|b| [b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15]]),
        );
        let c = Align16(
            self.bytes
                .map(|b| [b[16], b[17], b[18], b[19], b[20], b[21], b[22], b[23]]),
        );
        unsafe {
            [
                _mm_load_si128(a.0.as_ptr().cast()),
                _mm_load_si128(b.0.as_ptr().cast()),
                _mm_load_si128(c.0.as_ptr().cast()),
            ]
        }
    }
}

impl<const PREFIX: usize> SSE2Key<2, 32, PREFIX> {
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "sse2")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `sse2` before calling this function."]
    pub fn sse2_u64x4_key(&self) -> [__m128i; 4] {
        let a = Align16(
            self.bytes
                .map(|b| [b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]),
        );
        let b = Align16(
            self.bytes
                .map(|b| [b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15]]),
        );
        let c = Align16(
            self.bytes
                .map(|b| [b[16], b[17], b[18], b[19], b[20], b[21], b[22], b[23]]),
        );
        let d = Align16(
            self.bytes
                .map(|b| [b[24], b[25], b[26], b[27], b[28], b[29], b[30], b[31]]),
        );
        unsafe {
            [
                _mm_load_si128(a.0.as_ptr().cast()),
                _mm_load_si128(b.0.as_ptr().cast()),
                _mm_load_si128(c.0.as_ptr().cast()),
                _mm_load_si128(d.0.as_ptr().cast()),
            ]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use speck::SpeckVersion;

    #[rstest]
    #[case([], 0x0100_0908_1110_1918u64, [[0x1918u16; 8], [0x1110u16; 8], [0x0908u16; 8], [0x0100u16; 8]])]
    fn sse2_key_conversion_32_64(
        #[case] prefix: [u8; 0],
        #[case] value: u64,
        #[case] expected: [[u16; 8]; 4],
    ) {
        if !is_x86_feature_detected!("sse2") {
            return;
        }
        unsafe {
            let key = SSE2Key::<8, 8, 0>::new(&prefix, [value; 8], SpeckVersion::Speck32_64);
            let w: [[u16; 8]; 4] = key.sse2_u16x4_key().map(|r| std::mem::transmute(r));
            assert_eq!(w[0], expected[0]);
            assert_eq!(w[1], expected[1]);
            assert_eq!(w[2], expected[2]);
            assert_eq!(w[3], expected[3]);
        }
    }

    #[rstest]
    #[case([0x02], 0x0100_0a09_0812_1110u64, [[0x0012_1110u32; 4], [0x000a_0908u32; 4], [0x0002_0100u32; 4]])]
    fn sse2_key_conversion_48_72(
        #[case] prefix: [u8; 1],
        #[case] value: u64,
        #[case] expected: [[u32; 4]; 3],
    ) {
        if !is_x86_feature_detected!("sse2") {
            return;
        }
        unsafe {
            let key = SSE2Key::<4, 9, 1>::new(&prefix, [value; 4], SpeckVersion::Speck48_72);
            let w: [[u32; 4]; 3] = key.sse2_u24x3_key().map(|r| std::mem::transmute(r));
            assert_eq!(w[0], expected[0]);
            assert_eq!(w[1], expected[1]);
            assert_eq!(w[2], expected[2]);
        }
    }

    #[rstest]
    #[case([0x0a, 0x00, 0x01, 0x02], 0x0908_1211_101a_1918u64, [[0x001a_1918u32; 4], [0x0012_1110u32; 4], [0x000a_0908u32; 4], [0x0002_0100u32; 4]])]
    fn sse2_key_conversion_48_96(
        #[case] prefix: [u8; 4],
        #[case] value: u64,
        #[case] expected: [[u32; 4]; 4],
    ) {
        if !is_x86_feature_detected!("sse2") {
            return;
        }
        unsafe {
            let key = SSE2Key::<4, 12, 4>::new(&prefix, [value; 4], SpeckVersion::Speck48_96);
            let w: [[u32; 4]; 4] = key.sse2_u24x4_key().map(|r| std::mem::transmute(r));
            assert_eq!(w[0], expected[0]);
            assert_eq!(w[1], expected[1]);
            assert_eq!(w[2], expected[2]);
            assert_eq!(w[3], expected[3]);
        }
    }

    #[rstest]
    #[case([0x00, 0x01, 0x02, 0x03], 0x0b0a_0908_1312_1110u64, [[0x1312_1110u32; 4], [0x0b0a_0908u32; 4], [0x0302_0100u32; 4]])]
    fn sse2_key_conversion_64_96(
        #[case] prefix: [u8; 4],
        #[case] value: u64,
        #[case] expected: [[u32; 4]; 3],
    ) {
        if !is_x86_feature_detected!("sse2") {
            return;
        }
        unsafe {
            let key = SSE2Key::<4, 12, 4>::new(&prefix, [value; 4], SpeckVersion::Speck64_96);
            let w: [[u32; 4]; 3] = key.sse2_u32x3_key().map(|r| std::mem::transmute(r));
            assert_eq!(w[0], expected[0]);
            assert_eq!(w[1], expected[1]);
            assert_eq!(w[2], expected[2]);
        }
    }

    #[rstest]
    #[case([0x08, 0x09, 0x0a, 0x0b, 0x00, 0x01, 0x02, 0x03], 0x1312_1110_1b1a_1918u64, [[0x1b1a_1918u32; 4], [0x1312_1110u32; 4], [0x0b0a_0908u32; 4], [0x0302_0100u32; 4]])]
    fn sse2_key_conversion_64_128(
        #[case] prefix: [u8; 8],
        #[case] value: u64,
        #[case] expected: [[u32; 4]; 4],
    ) {
        if !is_x86_feature_detected!("sse2") {
            return;
        }
        unsafe {
            let key = SSE2Key::<4, 16, 8>::new(&prefix, [value; 4], SpeckVersion::Speck64_128);
            let w: [[u32; 4]; 4] = key.sse2_u32x4_key().map(|r| std::mem::transmute(r));
            assert_eq!(w[0], expected[0]);
            assert_eq!(w[1], expected[1]);
            assert_eq!(w[2], expected[2]);
            assert_eq!(w[3], expected[3]);
        }
    }

    #[rstest]
    #[case([0x02, 0x03, 0x04, 0x05], 0x0100_0d0c_0b0a_0908u64, [[0x0000_0d0c_0b0a_0908u64; 2], [0x0000_0504_0302_0100u64; 2]])]
    fn sse2_key_conversion_96_96(
        #[case] prefix: [u8; 4],
        #[case] value: u64,
        #[case] expected: [[u64; 2]; 2],
    ) {
        if !is_x86_feature_detected!("sse2") {
            return;
        }
        unsafe {
            let key = SSE2Key::<2, 12, 4>::new(&prefix, [value; 2], SpeckVersion::Speck96_96);
            let w: [[u64; 2]; 2] = key.sse2_u48x2_key().map(|r| std::mem::transmute(r));
            assert_eq!(w[0], expected[0]);
            assert_eq!(w[1], expected[1]);
        }
    }

    #[rstest]
    #[case(
        [0x0a, 0x0b, 0x0c, 0x0d, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05],
        0x0908_1514_1312_1110u64,
        [[0x0000_1514_1312_1110u64; 2], [0x0000_0d0c_0b0a_0908u64; 2], [0x0000_0504_0302_0100u64; 2]],
    )]
    fn sse2_key_conversion_96_144(
        #[case] prefix: [u8; 10],
        #[case] value: u64,
        #[case] expected: [[u64; 2]; 3],
    ) {
        if !is_x86_feature_detected!("sse2") {
            return;
        }
        unsafe {
            let key = SSE2Key::<2, 18, 10>::new(&prefix, [value; 2], SpeckVersion::Speck96_144);
            let w: [[u64; 2]; 3] = key.sse2_u48x3_key().map(|r| std::mem::transmute(r));
            assert_eq!(w[0], expected[0]);
            assert_eq!(w[1], expected[1]);
            assert_eq!(w[2], expected[2]);
        }
    }

    #[rstest]
    #[case([0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07], 0x0f0e_0d0c_0b0a_0908u64, [[0x0f0e_0d0c_0b0a_0908u64; 2], [0x0706_0504_0302_0100u64; 2]])]
    fn sse2_key_conversion_128_128(
        #[case] prefix: [u8; 8],
        #[case] value: u64,
        #[case] expected: [[u64; 2]; 2],
    ) {
        if !is_x86_feature_detected!("sse2") {
            return;
        }
        unsafe {
            let key = SSE2Key::<2, 16, 8>::new(&prefix, [value; 2], SpeckVersion::Speck128_128);
            let w: [[u64; 2]; 2] = key.sse2_u64x2_key().map(|r| std::mem::transmute(r));
            assert_eq!(w[0], expected[0]);
            assert_eq!(w[1], expected[1]);
        }
    }

    #[rstest]
    #[case(
        [0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07],
        0x1716_1514_1312_1110u64,
        [[0x1716_1514_1312_1110u64; 2], [0x0f0e_0d0c_0b0a_0908u64; 2], [0x0706_0504_0302_0100u64; 2]],
    )]
    fn sse2_key_conversion_128_192(
        #[case] prefix: [u8; 16],
        #[case] value: u64,
        #[case] expected: [[u64; 2]; 3],
    ) {
        if !is_x86_feature_detected!("sse2") {
            return;
        }
        unsafe {
            let key = SSE2Key::<2, 24, 16>::new(&prefix, [value; 2], SpeckVersion::Speck128_192);
            let w: [[u64; 2]; 3] = key.sse2_u64x3_key().map(|r| std::mem::transmute(r));
            assert_eq!(w[0], expected[0]);
            assert_eq!(w[1], expected[1]);
            assert_eq!(w[2], expected[2]);
        }
    }

    #[rstest]
    #[case(
        [0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07],
        0x1f1e_1d1c_1b1a_1918u64,
        [[0x1f1e_1d1c_1b1a_1918u64; 2], [0x1716_1514_1312_1110u64; 2], [0x0f0e_0d0c_0b0a_0908u64; 2], [0x0706_0504_0302_0100u64; 2]],
    )]
    fn sse2_key_conversion_128_256(
        #[case] prefix: [u8; 24],
        #[case] value: u64,
        #[case] expected: [[u64; 2]; 4],
    ) {
        if !is_x86_feature_detected!("sse2") {
            return;
        }
        unsafe {
            let key = SSE2Key::<2, 32, 24>::new(&prefix, [value; 2], SpeckVersion::Speck128_256);
            let w: [[u64; 2]; 4] = key.sse2_u64x4_key().map(|r| std::mem::transmute(r));
            assert_eq!(w[0], expected[0]);
            assert_eq!(w[1], expected[1]);
            assert_eq!(w[2], expected[2]);
            assert_eq!(w[3], expected[3]);
        }
    }
}
