#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::{
    uint16x8_t, uint32x4_t, uint64x2_t, vdupq_n_u16, vdupq_n_u32, vdupq_n_u64,
};

#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
#[doc = "# Safety"]
#[doc = "Caller must ensure CPU support for `neon` before calling this function."]
pub fn neon_u16x2_block_to_vec(v: [u16; 2]) -> [uint16x8_t; 2] {
    v.map(|l| vdupq_n_u16(l))
}

#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
#[doc = "# Safety"]
#[doc = "Caller must ensure CPU support for `neon` before calling this function."]
pub fn neon_u32x2_block_to_vec(v: [u32; 2]) -> [uint32x4_t; 2] {
    v.map(|l| vdupq_n_u32(l))
}

#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
#[doc = "# Safety"]
#[doc = "Caller must ensure CPU support for `neon` before calling this function."]
pub fn neon_u64x2_block_to_vec(v: [u64; 2]) -> [uint64x2_t; 2] {
    v.map(|l| vdupq_n_u64(l))
}
