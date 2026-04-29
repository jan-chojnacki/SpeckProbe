use crate::runtime::backend::macros::define_backend_dispatch;
use engine::x86_64::sse2::converter::{
    sse2_u16x2_block_to_vec, sse2_u32x2_block_to_vec, sse2_u64x2_block_to_vec,
};
use std::arch::x86_64::__m128i;

define_backend_dispatch! {
    attrs = [cfg(target_arch = "x86_64"), target_feature(enable = "sse2")],
    simd = sse2_,
    versions = [
        (32_64,   bytes=8,  ew=__m128i, vw=u16, converter=sse2_u16x2_block_to_vec),
        (48_72,   bytes=9,  ew=__m128i, vw=u32, converter=sse2_u32x2_block_to_vec),
        (48_96,   bytes=12, ew=__m128i, vw=u32, converter=sse2_u32x2_block_to_vec),
        (64_96,   bytes=12, ew=__m128i, vw=u32, converter=sse2_u32x2_block_to_vec),
        (64_128,  bytes=16, ew=__m128i, vw=u32, converter=sse2_u32x2_block_to_vec),
        (96_96,   bytes=12, ew=__m128i, vw=u64, converter=sse2_u64x2_block_to_vec),
        (96_144,  bytes=18, ew=__m128i, vw=u64, converter=sse2_u64x2_block_to_vec),
        (128_128, bytes=16, ew=__m128i, vw=u64, converter=sse2_u64x2_block_to_vec),
        (128_192, bytes=24, ew=__m128i, vw=u64, converter=sse2_u64x2_block_to_vec),
        (128_256, bytes=32, ew=__m128i, vw=u64, converter=sse2_u64x2_block_to_vec),
    ]
}
