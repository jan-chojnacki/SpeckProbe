use crate::domain::key::Key;
use crate::domain::simd_key::SimdKey;
use speck::SpeckVersion;

#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::uint8x16_t;
#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::{
    uint16x8_t, uint32x4_t, uint64x2_t, vdupq_n_u8, vld1q_u8, vreinterpretq_u16_u8,
    vreinterpretq_u32_u8, vreinterpretq_u64_u8,
};

#[derive(Debug, Copy, Clone)]
#[repr(C, align(16))]
pub struct NEONKey<const LANES: usize, const BYTES: usize, const PREFIX: usize> {
    bytes: [[u8; BYTES]; LANES],
    pa: uint8x16_t,
    pb: uint8x16_t,
    pc: uint8x16_t,
}

impl<const LANES: usize, const BYTES: usize, const PREFIX: usize> SimdKey<LANES>
    for NEONKey<LANES, BYTES, PREFIX>
{
    fn update(&mut self, v: [u64; LANES]) {
        self.update(v);
    }
}

impl<const LANES: usize, const BYTES: usize, const PREFIX: usize> NEONKey<LANES, BYTES, PREFIX> {
    const SUFFIX: usize = BYTES - PREFIX;

    #[cfg(target_arch = "aarch64")]
    #[target_feature(enable = "neon")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `neon` before calling this function."]
    pub fn new(prefix: &[u8; PREFIX], v: [u64; LANES], speck_version: SpeckVersion) -> Self {
        let mut bytes = [[0u8; BYTES]; LANES];

        for i in 0..LANES {
            bytes[i][Self::SUFFIX..].copy_from_slice(prefix);
            let suffix = v[i].to_le_bytes();
            bytes[i][..Self::SUFFIX].copy_from_slice(&suffix[..Self::SUFFIX]);
        }

        let mut pa = vdupq_n_u8(0);
        let mut pb = vdupq_n_u8(0);
        let mut pc = vdupq_n_u8(0);

        match speck_version {
            SpeckVersion::Speck48_96 => {
                let a: [[u8; 4]; LANES] = bytes.map(|b| [b[0], b[1], b[2], 0]);
                unsafe {
                    pa = vld1q_u8(a.as_ptr().cast());
                }
            }
            SpeckVersion::Speck64_96 => {
                let a: [[u8; 4]; LANES] = bytes.map(|b| [b[0], b[1], b[2], b[3]]);
                unsafe {
                    pa = vld1q_u8(a.as_ptr().cast());
                }
            }
            SpeckVersion::Speck64_128 => {
                let a: [[u8; 4]; LANES] = bytes.map(|b| [b[0], b[1], b[2], b[3]]);
                let b: [[u8; 4]; LANES] = bytes.map(|b| [b[4], b[5], b[6], b[7]]);
                unsafe {
                    pa = vld1q_u8(a.as_ptr().cast());
                    pb = vld1q_u8(b.as_ptr().cast());
                }
            }
            SpeckVersion::Speck96_144 => {
                let a: [[u8; 8]; LANES] = bytes.map(|b| [b[0], b[1], b[2], b[3], b[4], b[5], 0, 0]);
                unsafe {
                    pa = vld1q_u8(a.as_ptr().cast());
                }
            }
            SpeckVersion::Speck128_128 => {
                let a: [[u8; 8]; LANES] =
                    bytes.map(|b| [b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]);
                unsafe {
                    pa = vld1q_u8(a.as_ptr().cast());
                }
            }
            SpeckVersion::Speck128_192 => {
                let a: [[u8; 8]; LANES] =
                    bytes.map(|b| [b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]);
                let b: [[u8; 8]; LANES] =
                    bytes.map(|b| [b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15]]);
                unsafe {
                    pa = vld1q_u8(a.as_ptr().cast());
                    pb = vld1q_u8(b.as_ptr().cast());
                }
            }
            SpeckVersion::Speck128_256 => {
                let a: [[u8; 8]; LANES] =
                    bytes.map(|b| [b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]);
                let b: [[u8; 8]; LANES] =
                    bytes.map(|b| [b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15]]);
                let c: [[u8; 8]; LANES] =
                    bytes.map(|b| [b[16], b[17], b[18], b[19], b[20], b[21], b[22], b[23]]);
                unsafe {
                    pa = vld1q_u8(a.as_ptr().cast());
                    pb = vld1q_u8(b.as_ptr().cast());
                    pc = vld1q_u8(c.as_ptr().cast());
                }
            }
            _ => {}
        }

        Self { bytes, pa, pb, pc }
    }

    pub fn update(&mut self, v: [u64; LANES]) {
        for i in 0..LANES {
            let suffix = v[i].to_le_bytes();
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

impl<const PREFIX: usize> NEONKey<8, 8, PREFIX> {
    #[cfg(target_arch = "aarch64")]
    #[target_feature(enable = "neon")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `neon` before calling this function."]
    pub fn neon_u16x4_key(&self) -> [uint16x8_t; 4] {
        let a: [[u8; 2]; 8] = self.bytes.map(|b| [b[0], b[1]]);
        let b: [[u8; 2]; 8] = self.bytes.map(|b| [b[2], b[3]]);
        let c: [[u8; 2]; 8] = self.bytes.map(|b| [b[4], b[5]]);
        let d: [[u8; 2]; 8] = self.bytes.map(|b| [b[6], b[7]]);
        unsafe {
            [
                vreinterpretq_u16_u8(vld1q_u8(a.as_ptr().cast())),
                vreinterpretq_u16_u8(vld1q_u8(b.as_ptr().cast())),
                vreinterpretq_u16_u8(vld1q_u8(c.as_ptr().cast())),
                vreinterpretq_u16_u8(vld1q_u8(d.as_ptr().cast())),
            ]
        }
    }
}

impl<const PREFIX: usize> NEONKey<4, 9, PREFIX> {
    #[cfg(target_arch = "aarch64")]
    #[target_feature(enable = "neon")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `neon` before calling this function."]
    pub fn neon_u24x3_key(&self) -> [uint32x4_t; 3] {
        let a: [[u8; 4]; 4] = self.bytes.map(|b| [b[0], b[1], b[2], 0]);
        let b: [[u8; 4]; 4] = self.bytes.map(|b| [b[3], b[4], b[5], 0]);
        let c: [[u8; 4]; 4] = self.bytes.map(|b| [b[6], b[7], b[8], 0]);
        unsafe {
            [
                vreinterpretq_u32_u8(vld1q_u8(a.as_ptr().cast())),
                vreinterpretq_u32_u8(vld1q_u8(b.as_ptr().cast())),
                vreinterpretq_u32_u8(vld1q_u8(c.as_ptr().cast())),
            ]
        }
    }
}

impl<const PREFIX: usize> NEONKey<4, 12, PREFIX> {
    #[cfg(target_arch = "aarch64")]
    #[target_feature(enable = "neon")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `neon` before calling this function."]
    pub fn neon_u24x4_key(&self) -> [uint32x4_t; 4] {
        let a: [[u8; 4]; 4] = self.bytes.map(|b| [b[0], b[1], b[2], 0]);
        let b: [[u8; 4]; 4] = self.bytes.map(|b| [b[3], b[4], b[5], 0]);
        let c: [[u8; 4]; 4] = self.bytes.map(|b| [b[6], b[7], b[8], 0]);
        let d: [[u8; 4]; 4] = self.bytes.map(|b| [b[9], b[10], b[11], 0]);
        unsafe {
            [
                vreinterpretq_u32_u8(vld1q_u8(a.as_ptr().cast())),
                vreinterpretq_u32_u8(vld1q_u8(b.as_ptr().cast())),
                vreinterpretq_u32_u8(vld1q_u8(c.as_ptr().cast())),
                vreinterpretq_u32_u8(vld1q_u8(d.as_ptr().cast())),
            ]
        }
    }

    #[cfg(target_arch = "aarch64")]
    #[target_feature(enable = "neon")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `neon` before calling this function."]
    pub fn neon_u32x3_key(&self) -> [uint32x4_t; 3] {
        let a: [[u8; 4]; 4] = self.bytes.map(|b| [b[0], b[1], b[2], b[3]]);
        let b: [[u8; 4]; 4] = self.bytes.map(|b| [b[4], b[5], b[6], b[7]]);
        let c: [[u8; 4]; 4] = self.bytes.map(|b| [b[8], b[9], b[10], b[11]]);
        unsafe {
            [
                vreinterpretq_u32_u8(vld1q_u8(a.as_ptr().cast())),
                vreinterpretq_u32_u8(vld1q_u8(b.as_ptr().cast())),
                vreinterpretq_u32_u8(vld1q_u8(c.as_ptr().cast())),
            ]
        }
    }
}

impl<const PREFIX: usize> NEONKey<4, 16, PREFIX> {
    #[cfg(target_arch = "aarch64")]
    #[target_feature(enable = "neon")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `neon` before calling this function."]
    pub fn neon_u32x4_key(&self) -> [uint32x4_t; 4] {
        let a: [[u8; 4]; 4] = self.bytes.map(|b| [b[0], b[1], b[2], b[3]]);
        let b: [[u8; 4]; 4] = self.bytes.map(|b| [b[4], b[5], b[6], b[7]]);
        let c: [[u8; 4]; 4] = self.bytes.map(|b| [b[8], b[9], b[10], b[11]]);
        let d: [[u8; 4]; 4] = self.bytes.map(|b| [b[12], b[13], b[14], b[15]]);
        unsafe {
            [
                vreinterpretq_u32_u8(vld1q_u8(a.as_ptr().cast())),
                vreinterpretq_u32_u8(vld1q_u8(b.as_ptr().cast())),
                vreinterpretq_u32_u8(vld1q_u8(c.as_ptr().cast())),
                vreinterpretq_u32_u8(vld1q_u8(d.as_ptr().cast())),
            ]
        }
    }
}

impl<const PREFIX: usize> NEONKey<2, 12, PREFIX> {
    #[cfg(target_arch = "aarch64")]
    #[target_feature(enable = "neon")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `neon` before calling this function."]
    pub fn neon_u48x2_key(&self) -> [uint64x2_t; 2] {
        let a: [[u8; 8]; 2] = self
            .bytes
            .map(|b| [b[0], b[1], b[2], b[3], b[4], b[5], 0, 0]);
        let b: [[u8; 8]; 2] = self
            .bytes
            .map(|b| [b[6], b[7], b[8], b[9], b[10], b[11], 0, 0]);
        unsafe {
            [
                vreinterpretq_u64_u8(vld1q_u8(a.as_ptr().cast())),
                vreinterpretq_u64_u8(vld1q_u8(b.as_ptr().cast())),
            ]
        }
    }
}

impl<const PREFIX: usize> NEONKey<2, 18, PREFIX> {
    #[cfg(target_arch = "aarch64")]
    #[target_feature(enable = "neon")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `neon` before calling this function."]
    pub fn neon_u48x3_key(&self) -> [uint64x2_t; 3] {
        let a: [[u8; 8]; 2] = self
            .bytes
            .map(|b| [b[0], b[1], b[2], b[3], b[4], b[5], 0, 0]);
        let b: [[u8; 8]; 2] = self
            .bytes
            .map(|b| [b[6], b[7], b[8], b[9], b[10], b[11], 0, 0]);
        let c: [[u8; 8]; 2] = self
            .bytes
            .map(|b| [b[12], b[13], b[14], b[15], b[16], b[17], 0, 0]);
        unsafe {
            [
                vreinterpretq_u64_u8(vld1q_u8(a.as_ptr().cast())),
                vreinterpretq_u64_u8(vld1q_u8(b.as_ptr().cast())),
                vreinterpretq_u64_u8(vld1q_u8(c.as_ptr().cast())),
            ]
        }
    }
}

impl<const PREFIX: usize> NEONKey<2, 16, PREFIX> {
    #[cfg(target_arch = "aarch64")]
    #[target_feature(enable = "neon")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `neon` before calling this function."]
    pub fn neon_u64x2_key(&self) -> [uint64x2_t; 2] {
        let a: [[u8; 8]; 2] = self
            .bytes
            .map(|b| [b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]);
        let b: [[u8; 8]; 2] = self
            .bytes
            .map(|b| [b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15]]);
        unsafe {
            [
                vreinterpretq_u64_u8(vld1q_u8(a.as_ptr().cast())),
                vreinterpretq_u64_u8(vld1q_u8(b.as_ptr().cast())),
            ]
        }
    }
}

impl<const PREFIX: usize> NEONKey<2, 24, PREFIX> {
    #[cfg(target_arch = "aarch64")]
    #[target_feature(enable = "neon")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `neon` before calling this function."]
    pub fn neon_u64x3_key(&self) -> [uint64x2_t; 3] {
        let a: [[u8; 8]; 2] = self
            .bytes
            .map(|b| [b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]);
        let b: [[u8; 8]; 2] = self
            .bytes
            .map(|b| [b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15]]);
        let c: [[u8; 8]; 2] = self
            .bytes
            .map(|b| [b[16], b[17], b[18], b[19], b[20], b[21], b[22], b[23]]);
        unsafe {
            [
                vreinterpretq_u64_u8(vld1q_u8(a.as_ptr().cast())),
                vreinterpretq_u64_u8(vld1q_u8(b.as_ptr().cast())),
                vreinterpretq_u64_u8(vld1q_u8(c.as_ptr().cast())),
            ]
        }
    }
}

impl<const PREFIX: usize> NEONKey<2, 32, PREFIX> {
    #[cfg(target_arch = "aarch64")]
    #[target_feature(enable = "neon")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `neon` before calling this function."]
    pub fn neon_u64x4_key(&self) -> [uint64x2_t; 4] {
        let a: [[u8; 8]; 2] = self
            .bytes
            .map(|b| [b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]);
        let b: [[u8; 8]; 2] = self
            .bytes
            .map(|b| [b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15]]);
        let c: [[u8; 8]; 2] = self
            .bytes
            .map(|b| [b[16], b[17], b[18], b[19], b[20], b[21], b[22], b[23]]);
        let d: [[u8; 8]; 2] = self
            .bytes
            .map(|b| [b[24], b[25], b[26], b[27], b[28], b[29], b[30], b[31]]);
        unsafe {
            [
                vreinterpretq_u64_u8(vld1q_u8(a.as_ptr().cast())),
                vreinterpretq_u64_u8(vld1q_u8(b.as_ptr().cast())),
                vreinterpretq_u64_u8(vld1q_u8(c.as_ptr().cast())),
                vreinterpretq_u64_u8(vld1q_u8(d.as_ptr().cast())),
            ]
        }
    }
}

#[cfg(all(test, target_arch = "aarch64"))]
mod tests {
    use super::*;
    use rstest::rstest;
    use speck::SpeckVersion;

    #[rstest]
    #[case([], 0x0100_0908_1110_1918u64, [[0x1918u16; 8], [0x1110u16; 8], [0x0908u16; 8], [0x0100u16; 8]])]
    fn neon_key_conversion_32_64(
        #[case] prefix: [u8; 0],
        #[case] value: u64,
        #[case] expected: [[u16; 8]; 4],
    ) {
        if !std::arch::is_aarch64_feature_detected!("neon") {
            return;
        }
        unsafe {
            let key = NEONKey::<8, 8, 0>::new(&prefix, [value; 8], SpeckVersion::Speck32_64);
            let w: [[u16; 8]; 4] = key.neon_u16x4_key().map(|r| std::mem::transmute(r));
            assert_eq!(w[0], expected[0]);
            assert_eq!(w[1], expected[1]);
            assert_eq!(w[2], expected[2]);
            assert_eq!(w[3], expected[3]);
        }
    }

    #[rstest]
    #[case([0x02], 0x0100_0a09_0812_1110u64, [[0x0012_1110u32; 4], [0x000a_0908u32; 4], [0x0002_0100u32; 4]])]
    fn neon_key_conversion_48_72(
        #[case] prefix: [u8; 1],
        #[case] value: u64,
        #[case] expected: [[u32; 4]; 3],
    ) {
        if !std::arch::is_aarch64_feature_detected!("neon") {
            return;
        }
        unsafe {
            let key = NEONKey::<4, 9, 1>::new(&prefix, [value; 4], SpeckVersion::Speck48_72);
            let w: [[u32; 4]; 3] = key.neon_u24x3_key().map(|r| std::mem::transmute(r));
            assert_eq!(w[0], expected[0]);
            assert_eq!(w[1], expected[1]);
            assert_eq!(w[2], expected[2]);
        }
    }

    #[rstest]
    #[case([0x0a, 0x00, 0x01, 0x02], 0x0908_1211_101a_1918u64, [[0x001a_1918u32; 4], [0x0012_1110u32; 4], [0x000a_0908u32; 4], [0x0002_0100u32; 4]])]
    fn neon_key_conversion_48_96(
        #[case] prefix: [u8; 4],
        #[case] value: u64,
        #[case] expected: [[u32; 4]; 4],
    ) {
        if !std::arch::is_aarch64_feature_detected!("neon") {
            return;
        }
        unsafe {
            let key = NEONKey::<4, 12, 4>::new(&prefix, [value; 4], SpeckVersion::Speck48_96);
            let w: [[u32; 4]; 4] = key.neon_u24x4_key().map(|r| std::mem::transmute(r));
            assert_eq!(w[0], expected[0]);
            assert_eq!(w[1], expected[1]);
            assert_eq!(w[2], expected[2]);
            assert_eq!(w[3], expected[3]);
        }
    }

    #[rstest]
    #[case([0x00, 0x01, 0x02, 0x03], 0x0b0a_0908_1312_1110u64, [[0x1312_1110u32; 4], [0x0b0a_0908u32; 4], [0x0302_0100u32; 4]])]
    fn neon_key_conversion_64_96(
        #[case] prefix: [u8; 4],
        #[case] value: u64,
        #[case] expected: [[u32; 4]; 3],
    ) {
        if !std::arch::is_aarch64_feature_detected!("neon") {
            return;
        }
        unsafe {
            let key = NEONKey::<4, 12, 4>::new(&prefix, [value; 4], SpeckVersion::Speck64_96);
            let w: [[u32; 4]; 3] = key.neon_u32x3_key().map(|r| std::mem::transmute(r));
            assert_eq!(w[0], expected[0]);
            assert_eq!(w[1], expected[1]);
            assert_eq!(w[2], expected[2]);
        }
    }

    #[rstest]
    #[case([0x08, 0x09, 0x0a, 0x0b, 0x00, 0x01, 0x02, 0x03], 0x1312_1110_1b1a_1918u64, [[0x1b1a_1918u32; 4], [0x1312_1110u32; 4], [0x0b0a_0908u32; 4], [0x0302_0100u32; 4]])]
    fn neon_key_conversion_64_128(
        #[case] prefix: [u8; 8],
        #[case] value: u64,
        #[case] expected: [[u32; 4]; 4],
    ) {
        if !std::arch::is_aarch64_feature_detected!("neon") {
            return;
        }
        unsafe {
            let key = NEONKey::<4, 16, 8>::new(&prefix, [value; 4], SpeckVersion::Speck64_128);
            let w: [[u32; 4]; 4] = key.neon_u32x4_key().map(|r| std::mem::transmute(r));
            assert_eq!(w[0], expected[0]);
            assert_eq!(w[1], expected[1]);
            assert_eq!(w[2], expected[2]);
            assert_eq!(w[3], expected[3]);
        }
    }

    #[rstest]
    #[case([0x02, 0x03, 0x04, 0x05], 0x0100_0d0c_0b0a_0908u64, [[0x0000_0d0c_0b0a_0908u64; 2], [0x0000_0504_0302_0100u64; 2]])]
    fn neon_key_conversion_96_96(
        #[case] prefix: [u8; 4],
        #[case] value: u64,
        #[case] expected: [[u64; 2]; 2],
    ) {
        if !std::arch::is_aarch64_feature_detected!("neon") {
            return;
        }
        unsafe {
            let key = NEONKey::<2, 12, 4>::new(&prefix, [value; 2], SpeckVersion::Speck96_96);
            let w: [[u64; 2]; 2] = key.neon_u48x2_key().map(|r| std::mem::transmute(r));
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
    fn neon_key_conversion_96_144(
        #[case] prefix: [u8; 10],
        #[case] value: u64,
        #[case] expected: [[u64; 2]; 3],
    ) {
        if !std::arch::is_aarch64_feature_detected!("neon") {
            return;
        }
        unsafe {
            let key = NEONKey::<2, 18, 10>::new(&prefix, [value; 2], SpeckVersion::Speck96_144);
            let w: [[u64; 2]; 3] = key.neon_u48x3_key().map(|r| std::mem::transmute(r));
            assert_eq!(w[0], expected[0]);
            assert_eq!(w[1], expected[1]);
            assert_eq!(w[2], expected[2]);
        }
    }

    #[rstest]
    #[case([0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07], 0x0f0e_0d0c_0b0a_0908u64, [[0x0f0e_0d0c_0b0a_0908u64; 2], [0x0706_0504_0302_0100u64; 2]])]
    fn neon_key_conversion_128_128(
        #[case] prefix: [u8; 8],
        #[case] value: u64,
        #[case] expected: [[u64; 2]; 2],
    ) {
        if !std::arch::is_aarch64_feature_detected!("neon") {
            return;
        }
        unsafe {
            let key = NEONKey::<2, 16, 8>::new(&prefix, [value; 2], SpeckVersion::Speck128_128);
            let w: [[u64; 2]; 2] = key.neon_u64x2_key().map(|r| std::mem::transmute(r));
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
    fn neon_key_conversion_128_192(
        #[case] prefix: [u8; 16],
        #[case] value: u64,
        #[case] expected: [[u64; 2]; 3],
    ) {
        if !std::arch::is_aarch64_feature_detected!("neon") {
            return;
        }
        unsafe {
            let key = NEONKey::<2, 24, 16>::new(&prefix, [value; 2], SpeckVersion::Speck128_192);
            let w: [[u64; 2]; 3] = key.neon_u64x3_key().map(|r| std::mem::transmute(r));
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
    fn neon_key_conversion_128_256(
        #[case] prefix: [u8; 24],
        #[case] value: u64,
        #[case] expected: [[u64; 2]; 4],
    ) {
        if !std::arch::is_aarch64_feature_detected!("neon") {
            return;
        }
        unsafe {
            let key = NEONKey::<2, 32, 24>::new(&prefix, [value; 2], SpeckVersion::Speck128_256);
            let w: [[u64; 2]; 4] = key.neon_u64x4_key().map(|r| std::mem::transmute(r));
            assert_eq!(w[0], expected[0]);
            assert_eq!(w[1], expected[1]);
            assert_eq!(w[2], expected[2]);
            assert_eq!(w[3], expected[3]);
        }
    }
}
