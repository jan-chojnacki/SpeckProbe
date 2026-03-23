use crate::domain::key::Key;
use crate::domain::simd_key::SimdKey;
use speck::SpeckVersion;

#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::uint8x16_t;
#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
use std::arch::aarch64::{
    uint16x8_t, uint32x4_t, uint64x2_t, vdupq_n_u8, vld1q_u8, vreinterpretq_u16_u8,
    vreinterpretq_u32_u8, vreinterpretq_u64_u8,
};

#[derive(Debug, Copy, Clone)]
pub struct NEONKey<const T: usize> {
    bytes: [[u8; 32]; T],
    len: usize,
    prefix_len: usize,
    pa: uint8x16_t,
    pb: uint8x16_t,
    pc: uint8x16_t,
}

impl<const T: usize> SimdKey<T> for NEONKey<T> {
    fn update(&mut self, v: [u64; T]) {
        self.update(v);
    }
}

impl<const T: usize> NEONKey<T> {
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    #[target_feature(enable = "neon")]
    pub fn new(prefix: &[u8], v: [u64; T], speck_version: SpeckVersion) -> Self {
        let p = prefix.len();

        let mut bytes = [[0u8; 32]; T];

        let len = p + 8;

        for i in 0..T {
            bytes[i][..p].copy_from_slice(prefix);
            bytes[i][p..len].copy_from_slice(&v[i].to_le_bytes());
        }

        let prefix_len = p;

        let mut pa = vdupq_n_u8(0);
        let mut pb = vdupq_n_u8(0);
        let mut pc = vdupq_n_u8(0);

        match speck_version {
            SpeckVersion::Speck48_96 => {
                let a: [[u8; 4]; T] = bytes.map(|b| [b[0], b[1], b[2], 0]);
                unsafe {
                    pa = vld1q_u8(a.as_ptr().cast());
                }
            }
            SpeckVersion::Speck64_96 => {
                let a: [[u8; 4]; T] = bytes.map(|b| [b[0], b[1], b[2], b[3]]);
                unsafe {
                    pa = vld1q_u8(a.as_ptr().cast());
                }
            }
            SpeckVersion::Speck64_128 => {
                let a: [[u8; 4]; T] = bytes.map(|b| [b[0], b[1], b[2], b[3]]);
                let b: [[u8; 4]; T] = bytes.map(|b| [b[4], b[5], b[6], b[7]]);
                unsafe {
                    pa = vld1q_u8(a.as_ptr().cast());
                    pb = vld1q_u8(b.as_ptr().cast());
                }
            }
            SpeckVersion::Speck96_144 => {
                let a: [[u8; 8]; T] = bytes.map(|b| [b[0], b[1], b[2], b[3], b[4], b[5], 0, 0]);
                unsafe {
                    pa = vld1q_u8(a.as_ptr().cast());
                }
            }
            SpeckVersion::Speck128_128 => {
                let a: [[u8; 8]; T] =
                    bytes.map(|b| [b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]);
                unsafe {
                    pa = vld1q_u8(a.as_ptr().cast());
                }
            }
            SpeckVersion::Speck128_192 => {
                let a: [[u8; 8]; T] =
                    bytes.map(|b| [b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]);
                let b: [[u8; 8]; T] =
                    bytes.map(|b| [b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15]]);
                unsafe {
                    pa = vld1q_u8(a.as_ptr().cast());
                    pb = vld1q_u8(b.as_ptr().cast());
                }
            }
            SpeckVersion::Speck128_256 => {
                let a: [[u8; 8]; T] =
                    bytes.map(|b| [b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]);
                let b: [[u8; 8]; T] =
                    bytes.map(|b| [b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15]]);
                let c: [[u8; 8]; T] =
                    bytes.map(|b| [b[16], b[17], b[18], b[19], b[20], b[21], b[22], b[23]]);
                unsafe {
                    pa = vld1q_u8(a.as_ptr().cast());
                    pb = vld1q_u8(b.as_ptr().cast());
                    pc = vld1q_u8(c.as_ptr().cast());
                }
            }
            _ => {}
        }

        Self {
            bytes,
            len,
            prefix_len,
            pa,
            pb,
            pc,
        }
    }

    pub fn update(&mut self, v: [u64; T]) {
        let p = self.prefix_len;
        let len = self.len;

        for i in 0..T {
            self.bytes[i][p..len].copy_from_slice(&v[i].to_le_bytes());
        }
    }

    pub fn get(&self, i: usize) -> Key {
        let p = self.prefix_len;
        let row = &self.bytes[i];

        let value = u64::from_le_bytes(
            row[p..p + 8]
                .try_into()
                .expect("SimdKey invariant broken: value part must be 8 bytes"),
        );

        Key::new(&row[..p], value)
    }

    pub fn as_bytes(&self) -> [&[u8]; T] {
        self.bytes.each_ref().map(|b| &b[..self.len])
    }

    pub fn to_vec(&self) -> [Vec<u8>; T] {
        self.as_bytes().map(|b| b.to_vec())
    }
}

impl NEONKey<8> {
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    #[target_feature(enable = "neon")]
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

impl NEONKey<4> {
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    #[target_feature(enable = "neon")]
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

    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    #[target_feature(enable = "neon")]
    pub fn neon_u24x4_key(&self) -> [uint32x4_t; 4] {
        let b: [[u8; 4]; 4] = self.bytes.map(|b| [b[3], b[4], b[5], 0]);
        let c: [[u8; 4]; 4] = self.bytes.map(|b| [b[6], b[7], b[8], 0]);
        let d: [[u8; 4]; 4] = self.bytes.map(|b| [b[9], b[10], b[11], 0]);
        unsafe {
            [
                vreinterpretq_u32_u8(self.pa),
                vreinterpretq_u32_u8(vld1q_u8(b.as_ptr().cast())),
                vreinterpretq_u32_u8(vld1q_u8(c.as_ptr().cast())),
                vreinterpretq_u32_u8(vld1q_u8(d.as_ptr().cast())),
            ]
        }
    }

    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    #[target_feature(enable = "neon")]
    pub fn neon_u32x3_key(&self) -> [uint32x4_t; 3] {
        let b: [[u8; 4]; 4] = self.bytes.map(|b| [b[4], b[5], b[6], b[7]]);
        let c: [[u8; 4]; 4] = self.bytes.map(|b| [b[8], b[9], b[10], b[11]]);
        unsafe {
            [
                vreinterpretq_u32_u8(self.pa),
                vreinterpretq_u32_u8(vld1q_u8(b.as_ptr().cast())),
                vreinterpretq_u32_u8(vld1q_u8(c.as_ptr().cast())),
            ]
        }
    }

    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    #[target_feature(enable = "neon")]
    pub fn neon_u32x4_key(&self) -> [uint32x4_t; 4] {
        let c: [[u8; 4]; 4] = self.bytes.map(|b| [b[8], b[9], b[10], b[11]]);
        let d: [[u8; 4]; 4] = self.bytes.map(|b| [b[12], b[13], b[14], b[15]]);
        unsafe {
            [
                vreinterpretq_u32_u8(self.pa),
                vreinterpretq_u32_u8(self.pb),
                vreinterpretq_u32_u8(vld1q_u8(c.as_ptr().cast())),
                vreinterpretq_u32_u8(vld1q_u8(d.as_ptr().cast())),
            ]
        }
    }
}

impl NEONKey<2> {
    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    #[target_feature(enable = "neon")]
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

    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    #[target_feature(enable = "neon")]
    pub fn neon_u48x3_key(&self) -> [uint64x2_t; 3] {
        let b: [[u8; 8]; 2] = self
            .bytes
            .map(|b| [b[6], b[7], b[8], b[9], b[10], b[11], 0, 0]);
        let c: [[u8; 8]; 2] = self
            .bytes
            .map(|b| [b[12], b[13], b[14], b[15], b[16], b[17], 0, 0]);
        unsafe {
            [
                vreinterpretq_u64_u8(self.pa),
                vreinterpretq_u64_u8(vld1q_u8(b.as_ptr().cast())),
                vreinterpretq_u64_u8(vld1q_u8(c.as_ptr().cast())),
            ]
        }
    }

    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    #[target_feature(enable = "neon")]
    pub fn neon_u64x2_key(&self) -> [uint64x2_t; 2] {
        let b: [[u8; 8]; 2] = self
            .bytes
            .map(|b| [b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15]]);
        unsafe {
            [
                vreinterpretq_u64_u8(self.pa),
                vreinterpretq_u64_u8(vld1q_u8(b.as_ptr().cast())),
            ]
        }
    }

    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    #[target_feature(enable = "neon")]
    pub fn neon_u64x3_key(&self) -> [uint64x2_t; 3] {
        let c: [[u8; 8]; 2] = self
            .bytes
            .map(|b| [b[16], b[17], b[18], b[19], b[20], b[21], b[22], b[23]]);
        unsafe {
            [
                vreinterpretq_u64_u8(self.pa),
                vreinterpretq_u64_u8(self.pb),
                vreinterpretq_u64_u8(vld1q_u8(c.as_ptr().cast())),
            ]
        }
    }

    #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
    #[target_feature(enable = "neon")]
    pub fn neon_u64x4_key(&self) -> [uint64x2_t; 4] {
        let d: [[u8; 8]; 2] = self
            .bytes
            .map(|b| [b[24], b[25], b[26], b[27], b[28], b[29], b[30], b[31]]);
        unsafe {
            [
                vreinterpretq_u64_u8(self.pa),
                vreinterpretq_u64_u8(self.pb),
                vreinterpretq_u64_u8(self.pc),
                vreinterpretq_u64_u8(vld1q_u8(d.as_ptr().cast())),
            ]
        }
    }
}
