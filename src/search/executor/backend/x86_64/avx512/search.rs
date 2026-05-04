use crate::search::executor::backend::macros::define_search;
use crate::search::executor::backend::x86_64::avx512::comparator::{
    avx512_block_compare_u16, avx512_block_compare_u32, avx512_block_compare_u64,
};
use crate::speck::SpeckVersion;
use paste::paste;
use std::arch::x86_64::__m512i;

define_search!(
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx512bw")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `avx512bw` before calling this function."]
    version = SpeckVersion::Speck32_64,
    bytes = 8,
    vector = __m512i,
    comparator = avx512_block_compare_u16,
    key_conversion = u16x4_key,
    new_key = avx512_new_key,
    next_key = simd_next_into,
    name = 32_64,
    simd = avx512
);
define_search!(
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx512bw")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `avx512bw` before calling this function."]
    version = SpeckVersion::Speck48_72,
    bytes = 9,
    vector = __m512i,
    comparator = avx512_block_compare_u32,
    key_conversion = u24x3_key,
    new_key = avx512_new_key,
    next_key = simd_next_into,
    name = 48_72,
    simd = avx512
);
define_search!(
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx512bw")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `avx512bw` before calling this function."]
    version = SpeckVersion::Speck48_96,
    bytes = 12,
    vector = __m512i,
    comparator = avx512_block_compare_u32,
    key_conversion = u24x4_key,
    new_key = avx512_new_key,
    next_key = simd_next_into,
    name = 48_96,
    simd = avx512
);
define_search!(
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx512bw")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `avx512bw` before calling this function."]
    version = SpeckVersion::Speck64_96,
    bytes = 12,
    vector = __m512i,
    comparator = avx512_block_compare_u32,
    key_conversion = u32x3_key,
    new_key = avx512_new_key,
    next_key = simd_next_into,
    name = 64_96,
    simd = avx512
);
define_search!(
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx512bw")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `avx512bw` before calling this function."]
    version = SpeckVersion::Speck64_128,
    bytes = 16,
    vector = __m512i,
    comparator = avx512_block_compare_u32,
    key_conversion = u32x4_key,
    new_key = avx512_new_key,
    next_key = simd_next_into,
    name = 64_128,
    simd = avx512
);
define_search!(
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx512bw")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `avx512bw` before calling this function."]
    version = SpeckVersion::Speck96_96,
    bytes = 12,
    vector = __m512i,
    comparator = avx512_block_compare_u64,
    key_conversion = u48x2_key,
    new_key = avx512_new_key,
    next_key = simd_next_into,
    name = 96_96,
    simd = avx512
);
define_search!(
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx512bw")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `avx512bw` before calling this function."]
    version = SpeckVersion::Speck96_144,
    bytes = 18,
    vector = __m512i,
    comparator = avx512_block_compare_u64,
    key_conversion = u48x3_key,
    new_key = avx512_new_key,
    next_key = simd_next_into,
    name = 96_144,
    simd = avx512
);
define_search!(
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx512bw")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `avx512bw` before calling this function."]
    version = SpeckVersion::Speck128_128,
    bytes = 16,
    vector = __m512i,
    comparator = avx512_block_compare_u64,
    key_conversion = u64x2_key,
    new_key = avx512_new_key,
    next_key = simd_next_into,
    name = 128_128,
    simd = avx512
);
define_search!(
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx512bw")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `avx512bw` before calling this function."]
    version = SpeckVersion::Speck128_192,
    bytes = 24,
    vector = __m512i,
    comparator = avx512_block_compare_u64,
    key_conversion = u64x3_key,
    new_key = avx512_new_key,
    next_key = simd_next_into,
    name = 128_192,
    simd = avx512
);
define_search!(
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx512bw")]
    #[doc = "# Safety"]
    #[doc = "Caller must ensure CPU support for `avx512bw` before calling this function."]
    version = SpeckVersion::Speck128_256,
    bytes = 32,
    vector = __m512i,
    comparator = avx512_block_compare_u64,
    key_conversion = u64x4_key,
    new_key = avx512_new_key,
    next_key = simd_next_into,
    name = 128_256,
    simd = avx512
);
