use crate::domain::key::Key;
use crate::domain::simd_key::SimdKey;
use speck::SpeckVersion;
use std::arch::x86_64::{__m256i, _mm256_loadu_si256, _mm256_setzero_si256};

#[derive(Debug, Copy, Clone)]
pub struct AVX2Key<const LANES: usize, const BYTES: usize, const PREFIX: usize> {
    bytes: [[u8; BYTES]; LANES],
    pa: __m256i,
    pb: __m256i,
    pc: __m256i,
}

impl<const LANES: usize, const BYTES: usize, const PREFIX: usize> SimdKey<LANES>
    for AVX2Key<LANES, BYTES, PREFIX>
{
    fn update(&mut self, v: [u64; LANES]) {
        self.update(v);
    }
}

impl<const LANES: usize, const BYTES: usize, const PREFIX: usize> AVX2Key<LANES, BYTES, PREFIX> {
    const SUFFIX: usize = BYTES - PREFIX;

    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `avx2` before calling this function."]
    pub fn new(prefix: &[u8; PREFIX], v: [u64; LANES], speck_version: SpeckVersion) -> Self {
        let mut bytes = [[0u8; BYTES]; LANES];

        for i in 0..LANES {
            bytes[i][Self::SUFFIX..].copy_from_slice(prefix);
            let suffix = v[i].to_le_bytes();
            bytes[i][..Self::SUFFIX].copy_from_slice(&suffix[..Self::SUFFIX]);
        }

        let mut pa = _mm256_setzero_si256();
        let mut pb = _mm256_setzero_si256();
        let mut pc = _mm256_setzero_si256();

        match speck_version {
            SpeckVersion::Speck48_96 => {
                let a: [[u8; 4]; LANES] = bytes.map(|b| [b[0], b[1], b[2], 0]);
                unsafe {
                    pa = _mm256_loadu_si256(a.as_ptr().cast());
                }
            }
            SpeckVersion::Speck64_96 => {
                let a: [[u8; 4]; LANES] = bytes.map(|b| [b[0], b[1], b[2], b[3]]);
                unsafe {
                    pa = _mm256_loadu_si256(a.as_ptr().cast());
                }
            }
            SpeckVersion::Speck64_128 => {
                let a: [[u8; 4]; LANES] = bytes.map(|b| [b[0], b[1], b[2], b[3]]);
                let b: [[u8; 4]; LANES] = bytes.map(|b| [b[4], b[5], b[6], b[7]]);
                unsafe {
                    pa = _mm256_loadu_si256(a.as_ptr().cast());
                    pb = _mm256_loadu_si256(b.as_ptr().cast());
                }
            }
            SpeckVersion::Speck96_144 => {
                let a: [[u8; 8]; LANES] = bytes.map(|b| [b[0], b[1], b[2], b[3], b[4], b[5], 0, 0]);
                unsafe {
                    pa = _mm256_loadu_si256(a.as_ptr().cast());
                }
            }
            SpeckVersion::Speck128_128 => {
                let a: [[u8; 8]; LANES] =
                    bytes.map(|b| [b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]);
                unsafe {
                    pa = _mm256_loadu_si256(a.as_ptr().cast());
                }
            }
            SpeckVersion::Speck128_192 => {
                let a: [[u8; 8]; LANES] =
                    bytes.map(|b| [b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]);
                let b: [[u8; 8]; LANES] =
                    bytes.map(|b| [b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15]]);
                unsafe {
                    pa = _mm256_loadu_si256(a.as_ptr().cast());
                    pb = _mm256_loadu_si256(b.as_ptr().cast());
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
                    pa = _mm256_loadu_si256(a.as_ptr().cast());
                    pb = _mm256_loadu_si256(b.as_ptr().cast());
                    pc = _mm256_loadu_si256(c.as_ptr().cast());
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

impl<const PREFIX: usize> AVX2Key<16, 8, PREFIX> {
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `avx2` before calling this function."]
    pub fn avx2_u16x4_key(&self) -> [__m256i; 4] {
        let a: [[u8; 2]; 16] = self.bytes.map(|b| [b[0], b[1]]);
        let b: [[u8; 2]; 16] = self.bytes.map(|b| [b[2], b[3]]);
        let c: [[u8; 2]; 16] = self.bytes.map(|b| [b[4], b[5]]);
        let d: [[u8; 2]; 16] = self.bytes.map(|b| [b[6], b[7]]);
        unsafe {
            [
                _mm256_loadu_si256(a.as_ptr().cast()),
                _mm256_loadu_si256(b.as_ptr().cast()),
                _mm256_loadu_si256(c.as_ptr().cast()),
                _mm256_loadu_si256(d.as_ptr().cast()),
            ]
        }
    }
}

impl<const PREFIX: usize> AVX2Key<8, 9, PREFIX> {
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `avx2` before calling this function."]
    pub fn avx2_u24x3_key(&self) -> [__m256i; 3] {
        let a: [[u8; 4]; 8] = self.bytes.map(|b| [b[0], b[1], b[2], 0]);
        let b: [[u8; 4]; 8] = self.bytes.map(|b| [b[3], b[4], b[5], 0]);
        let c: [[u8; 4]; 8] = self.bytes.map(|b| [b[6], b[7], b[8], 0]);
        unsafe {
            [
                _mm256_loadu_si256(a.as_ptr().cast()),
                _mm256_loadu_si256(b.as_ptr().cast()),
                _mm256_loadu_si256(c.as_ptr().cast()),
            ]
        }
    }
}

impl<const PREFIX: usize> AVX2Key<8, 12, PREFIX> {
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `avx2` before calling this function."]
    pub fn avx2_u24x4_key(&self) -> [__m256i; 4] {
        let b: [[u8; 4]; 8] = self.bytes.map(|b| [b[3], b[4], b[5], 0]);
        let c: [[u8; 4]; 8] = self.bytes.map(|b| [b[6], b[7], b[8], 0]);
        let d: [[u8; 4]; 8] = self.bytes.map(|b| [b[9], b[10], b[11], 0]);
        unsafe {
            [
                self.pa,
                _mm256_loadu_si256(b.as_ptr().cast()),
                _mm256_loadu_si256(c.as_ptr().cast()),
                _mm256_loadu_si256(d.as_ptr().cast()),
            ]
        }
    }

    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `avx2` before calling this function."]
    pub fn avx2_u32x3_key(&self) -> [__m256i; 3] {
        let b: [[u8; 4]; 8] = self.bytes.map(|b| [b[4], b[5], b[6], b[7]]);
        let c: [[u8; 4]; 8] = self.bytes.map(|b| [b[8], b[9], b[10], b[11]]);
        unsafe {
            [
                self.pa,
                _mm256_loadu_si256(b.as_ptr().cast()),
                _mm256_loadu_si256(c.as_ptr().cast()),
            ]
        }
    }
}

impl<const PREFIX: usize> AVX2Key<8, 16, PREFIX> {
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `avx2` before calling this function."]
    pub fn avx2_u32x4_key(&self) -> [__m256i; 4] {
        let c: [[u8; 4]; 8] = self.bytes.map(|b| [b[8], b[9], b[10], b[11]]);
        let d: [[u8; 4]; 8] = self.bytes.map(|b| [b[12], b[13], b[14], b[15]]);
        unsafe {
            [
                self.pa,
                self.pb,
                _mm256_loadu_si256(c.as_ptr().cast()),
                _mm256_loadu_si256(d.as_ptr().cast()),
            ]
        }
    }
}

impl<const PREFIX: usize> AVX2Key<4, 12, PREFIX> {
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `avx2` before calling this function."]
    pub fn avx2_u48x2_key(&self) -> [__m256i; 2] {
        let a: [[u8; 8]; 4] = self
            .bytes
            .map(|b| [b[0], b[1], b[2], b[3], b[4], b[5], 0, 0]);
        let b: [[u8; 8]; 4] = self
            .bytes
            .map(|b| [b[6], b[7], b[8], b[9], b[10], b[11], 0, 0]);
        unsafe {
            [
                _mm256_loadu_si256(a.as_ptr().cast()),
                _mm256_loadu_si256(b.as_ptr().cast()),
            ]
        }
    }
}

impl<const PREFIX: usize> AVX2Key<4, 18, PREFIX> {
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `avx2` before calling this function."]
    pub fn avx2_u48x3_key(&self) -> [__m256i; 3] {
        let b: [[u8; 8]; 4] = self
            .bytes
            .map(|b| [b[6], b[7], b[8], b[9], b[10], b[11], 0, 0]);
        let c: [[u8; 8]; 4] = self
            .bytes
            .map(|b| [b[12], b[13], b[14], b[15], b[16], b[17], 0, 0]);
        unsafe {
            [
                self.pa,
                _mm256_loadu_si256(b.as_ptr().cast()),
                _mm256_loadu_si256(c.as_ptr().cast()),
            ]
        }
    }
}

impl<const PREFIX: usize> AVX2Key<4, 16, PREFIX> {
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `avx2` before calling this function."]
    pub fn avx2_u64x2_key(&self) -> [__m256i; 2] {
        let b: [[u8; 8]; 4] = self
            .bytes
            .map(|b| [b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15]]);
        unsafe { [self.pa, _mm256_loadu_si256(b.as_ptr().cast())] }
    }
}

impl<const PREFIX: usize> AVX2Key<4, 24, PREFIX> {
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `avx2` before calling this function."]
    pub fn avx2_u64x3_key(&self) -> [__m256i; 3] {
        let c: [[u8; 8]; 4] = self
            .bytes
            .map(|b| [b[16], b[17], b[18], b[19], b[20], b[21], b[22], b[23]]);
        unsafe { [self.pa, self.pb, _mm256_loadu_si256(c.as_ptr().cast())] }
    }
}

impl<const PREFIX: usize> AVX2Key<4, 32, PREFIX> {
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `avx2` before calling this function."]
    pub fn avx2_u64x4_key(&self) -> [__m256i; 4] {
        let d: [[u8; 8]; 4] = self
            .bytes
            .map(|b| [b[24], b[25], b[26], b[27], b[28], b[29], b[30], b[31]]);
        unsafe {
            [
                self.pa,
                self.pb,
                self.pc,
                _mm256_loadu_si256(d.as_ptr().cast()),
            ]
        }
    }
}
