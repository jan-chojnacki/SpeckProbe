use super::AVX2Key;
use crate::search::domain::key::Key;
use std::arch::x86_64::{__m256i, _mm256_cmpeq_epi16, _mm256_cmpeq_epi32, _mm256_movemask_epi8};

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[doc = "# Safety"]
#[doc = "Caller must ensure CPU support for `avx2` before calling this function."]
pub fn avx2_block_compare_u16<const BYTES: usize, const PREFIX: usize>(
    expected: &[__m256i; 2],
    value: &[__m256i; 2],
    key: &AVX2Key<16, BYTES, PREFIX>,
    out: &mut Vec<Key<BYTES, PREFIX>>,
) {
    let cmp_lo = _mm256_cmpeq_epi16(expected[0], value[0]);
    let cmp_hi = _mm256_cmpeq_epi16(expected[1], value[1]);

    let m_lo = _mm256_movemask_epi8(cmp_lo) as u32;
    let m_hi = _mm256_movemask_epi8(cmp_hi) as u32;

    let lanes_lo = m_lo & (m_lo >> 1);
    let lanes_hi = m_hi & (m_hi >> 1);

    let lane_bits_lo = (lanes_lo & 0x1)
        | ((lanes_lo >> 1) & 0x2)
        | ((lanes_lo >> 2) & 0x4)
        | ((lanes_lo >> 3) & 0x8)
        | ((lanes_lo >> 4) & 0x10)
        | ((lanes_lo >> 5) & 0x20)
        | ((lanes_lo >> 6) & 0x40)
        | ((lanes_lo >> 7) & 0x80)
        | ((lanes_lo >> 8) & 0x100)
        | ((lanes_lo >> 9) & 0x200)
        | ((lanes_lo >> 10) & 0x400)
        | ((lanes_lo >> 11) & 0x800)
        | ((lanes_lo >> 12) & 0x1000)
        | ((lanes_lo >> 13) & 0x2000)
        | ((lanes_lo >> 14) & 0x4000)
        | ((lanes_lo >> 15) & 0x8000);
    let lane_bits_hi = (lanes_hi & 0x1)
        | ((lanes_hi >> 1) & 0x2)
        | ((lanes_hi >> 2) & 0x4)
        | ((lanes_hi >> 3) & 0x8)
        | ((lanes_hi >> 4) & 0x10)
        | ((lanes_hi >> 5) & 0x20)
        | ((lanes_hi >> 6) & 0x40)
        | ((lanes_hi >> 7) & 0x80)
        | ((lanes_hi >> 8) & 0x100)
        | ((lanes_hi >> 9) & 0x200)
        | ((lanes_hi >> 10) & 0x400)
        | ((lanes_hi >> 11) & 0x800)
        | ((lanes_hi >> 12) & 0x1000)
        | ((lanes_hi >> 13) & 0x2000)
        | ((lanes_hi >> 14) & 0x4000)
        | ((lanes_hi >> 15) & 0x8000);

    let mut lanes = (lane_bits_lo & lane_bits_hi) & 0xFFFF;

    while lanes != 0 {
        let i = lanes.trailing_zeros() as usize;
        let k = key.get(i);
        out.push(k);
        lanes &= lanes - 1;
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[doc = "# Safety"]
#[doc = "Caller must ensure CPU support for `avx2` before calling this function."]
pub fn avx2_block_compare_u32<const BYTES: usize, const PREFIX: usize>(
    expected: &[__m256i; 2],
    value: &[__m256i; 2],
    key: &AVX2Key<8, BYTES, PREFIX>,
    out: &mut Vec<Key<BYTES, PREFIX>>,
) {
    let cmp_lo = _mm256_cmpeq_epi32(expected[0], value[0]);
    let cmp_hi = _mm256_cmpeq_epi32(expected[1], value[1]);

    let m_lo = _mm256_movemask_epi8(cmp_lo) as u32;
    let m_hi = _mm256_movemask_epi8(cmp_hi) as u32;

    let lanes_lo = m_lo & (m_lo >> 1) & (m_lo >> 2) & (m_lo >> 3);
    let lanes_hi = m_hi & (m_hi >> 1) & (m_hi >> 2) & (m_hi >> 3);

    let lane_bits_lo = (lanes_lo & 0x1)
        | ((lanes_lo >> 3) & 0x2)
        | ((lanes_lo >> 6) & 0x4)
        | ((lanes_lo >> 9) & 0x8)
        | ((lanes_lo >> 12) & 0x10)
        | ((lanes_lo >> 15) & 0x20)
        | ((lanes_lo >> 18) & 0x40)
        | ((lanes_lo >> 21) & 0x80);
    let lane_bits_hi = (lanes_hi & 0x1)
        | ((lanes_hi >> 3) & 0x2)
        | ((lanes_hi >> 6) & 0x4)
        | ((lanes_hi >> 9) & 0x8)
        | ((lanes_hi >> 12) & 0x10)
        | ((lanes_hi >> 15) & 0x20)
        | ((lanes_hi >> 18) & 0x40)
        | ((lanes_hi >> 21) & 0x80);

    let mut lanes = (lane_bits_lo & lane_bits_hi) & 0xFF;

    while lanes != 0 {
        let i = lanes.trailing_zeros() as usize;
        let k = key.get(i);
        out.push(k);
        lanes &= lanes - 1;
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[doc = "# Safety"]
#[doc = "Caller must ensure CPU support for `avx2` before calling this function."]
pub fn avx2_block_compare_u64<const BYTES: usize, const PREFIX: usize>(
    expected: &[__m256i; 2],
    value: &[__m256i; 2],
    key: &AVX2Key<4, BYTES, PREFIX>,
    out: &mut Vec<Key<BYTES, PREFIX>>,
) {
    let cmp_lo = _mm256_cmpeq_epi32(expected[0], value[0]);
    let cmp_hi = _mm256_cmpeq_epi32(expected[1], value[1]);

    let m_lo = _mm256_movemask_epi8(cmp_lo) as u32;
    let m_hi = _mm256_movemask_epi8(cmp_hi) as u32;

    let lanes_lo = m_lo
        & (m_lo >> 1)
        & (m_lo >> 2)
        & (m_lo >> 3)
        & (m_lo >> 4)
        & (m_lo >> 5)
        & (m_lo >> 6)
        & (m_lo >> 7);
    let lanes_hi = m_hi
        & (m_hi >> 1)
        & (m_hi >> 2)
        & (m_hi >> 3)
        & (m_hi >> 4)
        & (m_hi >> 5)
        & (m_hi >> 6)
        & (m_hi >> 7);

    let lane_bits_lo = (lanes_lo & 0x1)
        | ((lanes_lo >> 7) & 0x2)
        | ((lanes_lo >> 14) & 0x4)
        | ((lanes_lo >> 21) & 0x8);
    let lane_bits_hi = (lanes_hi & 0x1)
        | ((lanes_hi >> 7) & 0x2)
        | ((lanes_hi >> 14) & 0x4)
        | ((lanes_hi >> 21) & 0x8);

    let mut lanes = (lane_bits_lo & lane_bits_hi) & 0x0F;

    while lanes != 0 {
        let i = lanes.trailing_zeros() as usize;
        let k = key.get(i);
        out.push(k);
        lanes &= lanes - 1;
    }
}
