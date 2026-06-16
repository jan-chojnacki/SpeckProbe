use super::NEONKey;
use crate::search::domain::Key;
#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::{
    uint16x8_t, uint32x4_t, uint64x2_t, vandq_u16, vandq_u32, vandq_u64, vceqq_u16, vceqq_u32,
    vceqq_u64, vst1q_u16, vst1q_u32, vst1q_u64,
};

#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
#[doc = "# Safety"]
#[doc = "Caller must ensure CPU support for `neon` before calling this function."]
pub fn neon_block_compare_u16<const BYTES: usize, const PREFIX: usize>(
    e: &[uint16x8_t; 2],
    v: &[uint16x8_t; 2],
    key: &NEONKey<8, BYTES, PREFIX>,
    out: &mut Vec<Key<BYTES, PREFIX>>,
) {
    let cmp_lo = vceqq_u16(e[0], v[0]);
    let cmp_hi = vceqq_u16(e[1], v[1]);
    let cmp = vandq_u16(cmp_lo, cmp_hi);

    let mut lanes = [0u16; 8];
    unsafe {
        vst1q_u16(lanes.as_mut_ptr(), cmp);
    }
    for (i, m) in lanes.into_iter().enumerate() {
        if m == u16::MAX {
            out.push(key.get(i));
        }
    }
}

#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
#[doc = "# Safety"]
#[doc = "Caller must ensure CPU support for `neon` before calling this function."]
pub fn neon_block_compare_u32<const BYTES: usize, const PREFIX: usize>(
    e: &[uint32x4_t; 2],
    v: &[uint32x4_t; 2],
    key: &NEONKey<4, BYTES, PREFIX>,
    out: &mut Vec<Key<BYTES, PREFIX>>,
) {
    let cmp_lo = vceqq_u32(e[0], v[0]);
    let cmp_hi = vceqq_u32(e[1], v[1]);
    let cmp = vandq_u32(cmp_lo, cmp_hi);

    let mut lanes = [0u32; 4];
    unsafe {
        vst1q_u32(lanes.as_mut_ptr(), cmp);
    }
    for (i, m) in lanes.into_iter().enumerate() {
        if m == u32::MAX {
            out.push(key.get(i));
        }
    }
}

#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
#[doc = "# Safety"]
#[doc = "Caller must ensure CPU support for `neon` before calling this function."]
pub fn neon_block_compare_u64<const BYTES: usize, const PREFIX: usize>(
    e: &[uint64x2_t; 2],
    v: &[uint64x2_t; 2],
    key: &NEONKey<2, BYTES, PREFIX>,
    out: &mut Vec<Key<BYTES, PREFIX>>,
) {
    let cmp_lo = vceqq_u64(e[0], v[0]);
    let cmp_hi = vceqq_u64(e[1], v[1]);
    let cmp = vandq_u64(cmp_lo, cmp_hi);

    let mut lanes = [0u64; 2];
    unsafe {
        vst1q_u64(lanes.as_mut_ptr(), cmp);
    }
    for (i, m) in lanes.into_iter().enumerate() {
        if m == u64::MAX {
            out.push(key.get(i));
        }
    }
}
