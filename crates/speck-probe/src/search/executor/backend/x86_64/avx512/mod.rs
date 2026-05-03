pub(crate) mod comparator;
mod converter;
pub(crate) use converter::avx512_u16x2_block_to_vec;
pub(crate) use converter::avx512_u32x2_block_to_vec;
pub(crate) use converter::avx512_u64x2_block_to_vec;
mod key;
use crate::search::executor::backend::macros::define_backend_dispatch;
pub(crate) use key::AVX512Key;
use std::arch::x86_64::__m512i;

define_backend_dispatch! {
    attrs = [cfg(target_arch = "x86_64"), target_feature(enable = "avx512bw")],
    simd = avx512_,
    versions = [
        (32_64,   bytes=8,  ew=__m512i, vw=u16, converter=avx512_u16x2_block_to_vec),
        (48_72,   bytes=9,  ew=__m512i, vw=u32, converter=avx512_u32x2_block_to_vec),
        (48_96,   bytes=12, ew=__m512i, vw=u32, converter=avx512_u32x2_block_to_vec),
        (64_96,   bytes=12, ew=__m512i, vw=u32, converter=avx512_u32x2_block_to_vec),
        (64_128,  bytes=16, ew=__m512i, vw=u32, converter=avx512_u32x2_block_to_vec),
        (96_96,   bytes=12, ew=__m512i, vw=u64, converter=avx512_u64x2_block_to_vec),
        (96_144,  bytes=18, ew=__m512i, vw=u64, converter=avx512_u64x2_block_to_vec),
        (128_128, bytes=16, ew=__m512i, vw=u64, converter=avx512_u64x2_block_to_vec),
        (128_192, bytes=24, ew=__m512i, vw=u64, converter=avx512_u64x2_block_to_vec),
        (128_256, bytes=32, ew=__m512i, vw=u64, converter=avx512_u64x2_block_to_vec),
    ]
}
