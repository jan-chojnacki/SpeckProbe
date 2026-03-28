use crate::backend::x86_64::avx512::key::AVX512Key;
use crate::domain::key::Key;
use std::arch::x86_64::{
    __m512i, _mm512_cmpeq_epi16_mask, _mm512_cmpeq_epi32_mask, _mm512_cmpeq_epi64_mask,
};

#[cfg(all(
    target_arch = "x86_64",
    target_feature = "avx512f",
    target_feature = "avx512bw"
))]
#[target_feature(enable = "avx512f,avx512bw")]
pub fn avx512_block_compare_u16<const BYTES: usize, const PREFIX: usize>(
    e: &[__m512i; 2],
    v: &[__m512i; 2],
    key: &AVX512Key<32, BYTES, PREFIX>,
    out: &mut Vec<Key<BYTES, PREFIX>>,
) {
    let cmp_lo = _mm512_cmpeq_epi16_mask(e[0], v[0]);
    let cmp_hi = _mm512_cmpeq_epi16_mask(e[1], v[1]);
    let mut lanes = cmp_lo & cmp_hi;

    while lanes != 0 {
        let i = lanes.trailing_zeros() as usize;
        let k = key.get(i);
        out.push(k);
        lanes &= lanes - 1;
    }
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
#[target_feature(enable = "avx512f")]
pub fn avx512_block_compare_u32<const BYTES: usize, const PREFIX: usize>(
    e: &[__m512i; 2],
    v: &[__m512i; 2],
    key: &AVX512Key<16, BYTES, PREFIX>,
    out: &mut Vec<Key<BYTES, PREFIX>>,
) {
    let cmp_lo = _mm512_cmpeq_epi32_mask(e[0], v[0]);
    let cmp_hi = _mm512_cmpeq_epi32_mask(e[1], v[1]);
    let mut lanes = cmp_lo & cmp_hi;

    while lanes != 0 {
        let i = lanes.trailing_zeros() as usize;
        let k = key.get(i);
        out.push(k);
        lanes &= lanes - 1;
    }
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
#[target_feature(enable = "avx512f")]
pub fn avx512_block_compare_u64<const BYTES: usize, const PREFIX: usize>(
    e: &[__m512i; 2],
    v: &[__m512i; 2],
    key: &AVX512Key<8, BYTES, PREFIX>,
    out: &mut Vec<Key<BYTES, PREFIX>>,
) {
    let cmp_lo = _mm512_cmpeq_epi64_mask(e[0], v[0]);
    let cmp_hi = _mm512_cmpeq_epi64_mask(e[1], v[1]);
    let mut lanes = cmp_lo & cmp_hi;

    while lanes != 0 {
        let i = lanes.trailing_zeros() as usize;
        let k = key.get(i);
        out.push(k);
        lanes &= lanes - 1;
    }
}
