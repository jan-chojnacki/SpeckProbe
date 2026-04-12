use crate::backend::macros::define_backend_dispatch;
use engine::aarch64::neon::converter::{
    neon_u16x2_block_to_vec, neon_u32x2_block_to_vec, neon_u64x2_block_to_vec,
};
use std::arch::aarch64::{uint16x8_t, uint32x4_t, uint64x2_t};

define_backend_dispatch! {
    attrs = [cfg(target_arch = "aarch64"), target_feature(enable = "neon")],
    simd = neon_,
    versions = [
        (32_64,   bytes=8,  ew=uint16x8_t, vw=u16, converter=neon_u16x2_block_to_vec),
        (48_72,   bytes=9,  ew=uint32x4_t, vw=u32, converter=neon_u32x2_block_to_vec),
        (48_96,   bytes=12, ew=uint32x4_t, vw=u32, converter=neon_u32x2_block_to_vec),
        (64_96,   bytes=12, ew=uint32x4_t, vw=u32, converter=neon_u32x2_block_to_vec),
        (64_128,  bytes=16, ew=uint32x4_t, vw=u32, converter=neon_u32x2_block_to_vec),
        (96_96,   bytes=12, ew=uint64x2_t, vw=u64, converter=neon_u64x2_block_to_vec),
        (96_144,  bytes=18, ew=uint64x2_t, vw=u64, converter=neon_u64x2_block_to_vec),
        (128_128, bytes=16, ew=uint64x2_t, vw=u64, converter=neon_u64x2_block_to_vec),
        (128_192, bytes=24, ew=uint64x2_t, vw=u64, converter=neon_u64x2_block_to_vec),
        (128_256, bytes=32, ew=uint64x2_t, vw=u64, converter=neon_u64x2_block_to_vec),
    ]
}
