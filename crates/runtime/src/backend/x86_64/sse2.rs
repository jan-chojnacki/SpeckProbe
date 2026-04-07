use crate::backend::macors::define_runtime_variants_default;
use engine::x86_64::sse2::converter::{
    sse2_u16x2_block_to_vec, sse2_u32x2_block_to_vec, sse2_u64x2_block_to_vec,
};
use std::arch::x86_64::__m128i;

define_runtime_variants_default!(
    attrs = [
        #[cfg(target_arch = "x86_64")]
        #[target_feature(enable = "sse2")]
    ],
    sse2_32_64,
    32_64,
    __m128i,
    u16,
    converter = sse2_u16x2_block_to_vec,
    bytes = 8,
    simd = sse2_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(target_arch = "x86_64")]
        #[target_feature(enable = "sse2")]
    ],
    sse2_48_72,
    48_72,
    __m128i,
    u32,
    converter = sse2_u32x2_block_to_vec,
    bytes = 9,
    simd = sse2_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(target_arch = "x86_64")]
        #[target_feature(enable = "sse2")]
    ],
    sse2_48_96,
    48_96,
    __m128i,
    u32,
    converter = sse2_u32x2_block_to_vec,
    bytes = 12,
    simd = sse2_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(target_arch = "x86_64")]
        #[target_feature(enable = "sse2")]
    ],
    sse2_64_96,
    64_96,
    __m128i,
    u32,
    converter = sse2_u32x2_block_to_vec,
    bytes = 12,
    simd = sse2_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(target_arch = "x86_64")]
        #[target_feature(enable = "sse2")]
    ],
    sse2_64_128,
    64_128,
    __m128i,
    u32,
    converter = sse2_u32x2_block_to_vec,
    bytes = 16,
    simd = sse2_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(target_arch = "x86_64")]
        #[target_feature(enable = "sse2")]
    ],
    sse2_96_96,
    96_96,
    __m128i,
    u64,
    converter = sse2_u64x2_block_to_vec,
    bytes = 12,
    simd = sse2_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(target_arch = "x86_64")]
        #[target_feature(enable = "sse2")]
    ],
    sse2_96_144,
    96_144,
    __m128i,
    u64,
    converter = sse2_u64x2_block_to_vec,
    bytes = 18,
    simd = sse2_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(target_arch = "x86_64")]
        #[target_feature(enable = "sse2")]
    ],
    sse2_128_128,
    128_128,
    __m128i,
    u64,
    converter = sse2_u64x2_block_to_vec,
    bytes = 16,
    simd = sse2_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(target_arch = "x86_64")]
        #[target_feature(enable = "sse2")]
    ],
    sse2_128_192,
    128_192,
    __m128i,
    u64,
    converter = sse2_u64x2_block_to_vec,
    bytes = 24,
    simd = sse2_,
);
define_runtime_variants_default!(
    attrs = [
        #[cfg(target_arch = "x86_64")]
        #[target_feature(enable = "sse2")]
    ],
    sse2_128_256,
    128_256,
    __m128i,
    u64,
    converter = sse2_u64x2_block_to_vec,
    bytes = 32,
    simd = sse2_,
);
