use crate::backend::macors::define_runtime_variants_default;
use engine::aarch64::neon::converter::{
    neon_u16x2_block_to_vec, neon_u32x2_block_to_vec, neon_u64x2_block_to_vec,
};
use std::arch::aarch64::{uint16x8_t, uint32x4_t, uint64x2_t};

define_runtime_variants_default!(
    attrs = [
        #[cfg(target_arch = "aarch64")]
        #[target_feature(enable = "neon")]
    ],
    neon_32_64,
    32_64,
    uint16x8_t,
    u16,
    converter = neon_u16x2_block_to_vec,
    bytes = 8,
    simd = neon_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(target_arch = "aarch64")]
        #[target_feature(enable = "neon")]
    ],
    neon_48_72,
    48_72,
    uint32x4_t,
    u32,
    converter = neon_u32x2_block_to_vec,
    bytes = 9,
    simd = neon_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(target_arch = "aarch64")]
        #[target_feature(enable = "neon")]
    ],
    neon_48_96,
    48_96,
    uint32x4_t,
    u32,
    converter = neon_u32x2_block_to_vec,
    bytes = 12,
    simd = neon_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(target_arch = "aarch64")]
        #[target_feature(enable = "neon")]
    ],
    neon_64_96,
    64_96,
    uint32x4_t,
    u32,
    converter = neon_u32x2_block_to_vec,
    bytes = 12,
    simd = neon_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(target_arch = "aarch64")]
        #[target_feature(enable = "neon")]
    ],
    neon_64_128,
    64_128,
    uint32x4_t,
    u32,
    converter = neon_u32x2_block_to_vec,
    bytes = 16,
    simd = neon_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(target_arch = "aarch64")]
        #[target_feature(enable = "neon")]
    ],
    neon_96_96,
    96_96,
    uint64x2_t,
    u64,
    converter = neon_u64x2_block_to_vec,
    bytes = 12,
    simd = neon_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(target_arch = "aarch64")]
        #[target_feature(enable = "neon")]
    ],
    neon_96_144,
    96_144,
    uint64x2_t,
    u64,
    converter = neon_u64x2_block_to_vec,
    bytes = 18,
    simd = neon_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(target_arch = "aarch64")]
        #[target_feature(enable = "neon")]
    ],
    neon_128_128,
    128_128,
    uint64x2_t,
    u64,
    converter = neon_u64x2_block_to_vec,
    bytes = 16,
    simd = neon_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(target_arch = "aarch64")]
        #[target_feature(enable = "neon")]
    ],
    neon_128_192,
    128_192,
    uint64x2_t,
    u64,
    converter = neon_u64x2_block_to_vec,
    bytes = 24,
    simd = neon_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(target_arch = "aarch64")]
        #[target_feature(enable = "neon")]
    ],
    neon_128_256,
    128_256,
    uint64x2_t,
    u64,
    converter = neon_u64x2_block_to_vec,
    bytes = 32,
    simd = neon_,
);
