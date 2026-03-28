use crate::backend::x86_64::avx2::comparator::{
    avx2_block_compare_u16, avx2_block_compare_u32, avx2_block_compare_u64,
};
use crate::backend::x86_64::avx512::comparator::{
    avx512_block_compare_u16, avx512_block_compare_u32, avx512_block_compare_u64,
};
use crate::backend::x86_64::sse2::comparator::{
    sse2_block_compare_u16, sse2_block_compare_u32, sse2_block_compare_u64,
};
use crate::domain::key::Key;
use crate::domain::key_iterator::KeyIterator;
use crate::domain::task::Task;
use speck::SpeckVersion;
use std::arch::x86_64::{__m128i, __m256i, __m512i};

macro_rules! define_search {
    ($(#[$meta:meta])* $name:ident, version = $version:path, bytes = $bytes:literal, vector = $vector:ty, function = $function:path, comparator = $comparator:path, key_conversion = $key_conversion:ident, new_key = $new_key:ident) => {
        $(#[$meta])*
        pub fn $name<const PREFIX: usize>(
            task: Task<$vector, $bytes, PREFIX>,
            out: &mut Vec<Key<$bytes, PREFIX>>,
        ) {
            let mut iter = KeyIterator::<$bytes, PREFIX>::new(task.start, task.end, task.prefix, $version);
            let mut key = iter.$new_key();

            while iter.simd_next_into(&mut key).is_some() {
                let result = $function(task.data, key.$key_conversion());
                $comparator(&task.expected, &result, &key, out);
            }
        }
    };
}

define_search!(
    #[target_feature(enable = "sse2")]
    sse2_search_encrypt_32_64,
    version = SpeckVersion::Speck32_64,
    bytes = 8,
    vector = __m128i,
    function = speck::sse2_encrypt_block_32_64,
    comparator = sse2_block_compare_u16,
    key_conversion = sse2_u16x4_key,
    new_key = sse2_new_key
);
define_search!(
    #[target_feature(enable = "sse2")]
    sse2_search_decrypt_32_64,
    version = SpeckVersion::Speck32_64,
    bytes = 8,
    vector = __m128i,
    function = speck::sse2_decrypt_block_32_64,
    comparator = sse2_block_compare_u16,
    key_conversion = sse2_u16x4_key,
    new_key = sse2_new_key
);
define_search!(
    #[target_feature(enable = "sse2")]
    sse2_search_decrypt_48_72,
    version = SpeckVersion::Speck48_72,
    bytes = 9,
    vector = __m128i,
    function = speck::sse2_decrypt_block_48_72,
    comparator = sse2_block_compare_u32,
    key_conversion = sse2_u24x3_key,
    new_key = sse2_new_key
);
define_search!(
    #[target_feature(enable = "sse2")]
    sse2_search_encrypt_48_96,
    version = SpeckVersion::Speck48_96,
    bytes = 12,
    vector = __m128i,
    function = speck::sse2_encrypt_block_48_96,
    comparator = sse2_block_compare_u32,
    key_conversion = sse2_u24x4_key,
    new_key = sse2_new_key
);
define_search!(
    #[target_feature(enable = "sse2")]
    sse2_search_decrypt_48_96,
    version = SpeckVersion::Speck48_96,
    bytes = 12,
    vector = __m128i,
    function = speck::sse2_decrypt_block_48_96,
    comparator = sse2_block_compare_u32,
    key_conversion = sse2_u24x4_key,
    new_key = sse2_new_key
);
define_search!(
    #[target_feature(enable = "sse2")]
    sse2_search_encrypt_64_96,
    version = SpeckVersion::Speck64_96,
    bytes = 12,
    vector = __m128i,
    function = speck::sse2_encrypt_block_64_96,
    comparator = sse2_block_compare_u32,
    key_conversion = sse2_u32x3_key,
    new_key = sse2_new_key
);
define_search!(
    #[target_feature(enable = "sse2")]
    sse2_search_decrypt_64_96,
    version = SpeckVersion::Speck64_96,
    bytes = 12,
    vector = __m128i,
    function = speck::sse2_decrypt_block_64_96,
    comparator = sse2_block_compare_u32,
    key_conversion = sse2_u32x3_key,
    new_key = sse2_new_key
);
define_search!(
    #[target_feature(enable = "sse2")]
    sse2_search_encrypt_64_128,
    version = SpeckVersion::Speck64_128,
    bytes = 16,
    vector = __m128i,
    function = speck::sse2_encrypt_block_64_128,
    comparator = sse2_block_compare_u32,
    key_conversion = sse2_u32x4_key,
    new_key = sse2_new_key
);
define_search!(
    #[target_feature(enable = "sse2")]
    sse2_search_decrypt_64_128,
    version = SpeckVersion::Speck64_128,
    bytes = 16,
    vector = __m128i,
    function = speck::sse2_decrypt_block_64_128,
    comparator = sse2_block_compare_u32,
    key_conversion = sse2_u32x4_key,
    new_key = sse2_new_key
);
define_search!(
    #[target_feature(enable = "sse2")]
    sse2_search_encrypt_96_96,
    version = SpeckVersion::Speck96_96,
    bytes = 12,
    vector = __m128i,
    function = speck::sse2_encrypt_block_96_96,
    comparator = sse2_block_compare_u64,
    key_conversion = sse2_u48x2_key,
    new_key = sse2_new_key
);
define_search!(
    #[target_feature(enable = "sse2")]
    sse2_search_decrypt_96_96,
    version = SpeckVersion::Speck96_96,
    bytes = 12,
    vector = __m128i,
    function = speck::sse2_decrypt_block_96_96,
    comparator = sse2_block_compare_u64,
    key_conversion = sse2_u48x2_key,
    new_key = sse2_new_key
);
define_search!(
    #[target_feature(enable = "sse2")]
    sse2_search_encrypt_96_144,
    version = SpeckVersion::Speck96_144,
    bytes = 18,
    vector = __m128i,
    function = speck::sse2_encrypt_block_96_144,
    comparator = sse2_block_compare_u64,
    key_conversion = sse2_u48x3_key,
    new_key = sse2_new_key
);
define_search!(
    #[target_feature(enable = "sse2")]
    sse2_search_decrypt_96_144,
    version = SpeckVersion::Speck96_144,
    bytes = 18,
    vector = __m128i,
    function = speck::sse2_decrypt_block_96_144,
    comparator = sse2_block_compare_u64,
    key_conversion = sse2_u48x3_key,
    new_key = sse2_new_key
);
define_search!(
    #[target_feature(enable = "sse2")]
    sse2_search_encrypt_128_128,
    version = SpeckVersion::Speck128_128,
    bytes = 16,
    vector = __m128i,
    function = speck::sse2_encrypt_block_128_128,
    comparator = sse2_block_compare_u64,
    key_conversion = sse2_u64x2_key,
    new_key = sse2_new_key
);
define_search!(
    #[target_feature(enable = "sse2")]
    sse2_search_decrypt_128_128,
    version = SpeckVersion::Speck128_128,
    bytes = 16,
    vector = __m128i,
    function = speck::sse2_decrypt_block_128_128,
    comparator = sse2_block_compare_u64,
    key_conversion = sse2_u64x2_key,
    new_key = sse2_new_key
);
define_search!(
    #[target_feature(enable = "sse2")]
    sse2_search_encrypt_128_192,
    version = SpeckVersion::Speck128_192,
    bytes = 24,
    vector = __m128i,
    function = speck::sse2_encrypt_block_128_192,
    comparator = sse2_block_compare_u64,
    key_conversion = sse2_u64x3_key,
    new_key = sse2_new_key
);
define_search!(
    #[target_feature(enable = "sse2")]
    sse2_search_decrypt_128_192,
    version = SpeckVersion::Speck128_192,
    bytes = 24,
    vector = __m128i,
    function = speck::sse2_decrypt_block_128_192,
    comparator = sse2_block_compare_u64,
    key_conversion = sse2_u64x3_key,
    new_key = sse2_new_key
);
define_search!(
    #[target_feature(enable = "sse2")]
    sse2_search_encrypt_128_256,
    version = SpeckVersion::Speck128_256,
    bytes = 32,
    vector = __m128i,
    function = speck::sse2_encrypt_block_128_256,
    comparator = sse2_block_compare_u64,
    key_conversion = sse2_u64x4_key,
    new_key = sse2_new_key
);
define_search!(
    #[target_feature(enable = "sse2")]
    sse2_search_decrypt_128_256,
    version = SpeckVersion::Speck128_256,
    bytes = 32,
    vector = __m128i,
    function = speck::sse2_decrypt_block_128_256,
    comparator = sse2_block_compare_u64,
    key_conversion = sse2_u64x4_key,
    new_key = sse2_new_key
);

define_search!(
    #[target_feature(enable = "avx2")]
    avx2_search_encrypt_32_64,
    version = SpeckVersion::Speck32_64,
    bytes = 8,
    vector = __m256i,
    function = speck::avx2_encrypt_block_32_64,
    comparator = avx2_block_compare_u16,
    key_conversion = avx2_u16x4_key,
    new_key = avx2_new_key
);
define_search!(
    #[target_feature(enable = "avx2")]
    avx2_search_decrypt_32_64,
    version = SpeckVersion::Speck32_64,
    bytes = 8,
    vector = __m256i,
    function = speck::avx2_decrypt_block_32_64,
    comparator = avx2_block_compare_u16,
    key_conversion = avx2_u16x4_key,
    new_key = avx2_new_key
);
define_search!(
    #[target_feature(enable = "avx2")]
    avx2_search_encrypt_48_72,
    version = SpeckVersion::Speck48_72,
    bytes = 9,
    vector = __m256i,
    function = speck::avx2_encrypt_block_48_72,
    comparator = avx2_block_compare_u32,
    key_conversion = avx2_u24x3_key,
    new_key = avx2_new_key
);
define_search!(
    #[target_feature(enable = "avx2")]
    avx2_search_decrypt_48_72,
    version = SpeckVersion::Speck48_72,
    bytes = 9,
    vector = __m256i,
    function = speck::avx2_decrypt_block_48_72,
    comparator = avx2_block_compare_u32,
    key_conversion = avx2_u24x3_key,
    new_key = avx2_new_key
);
define_search!(
    #[target_feature(enable = "avx2")]
    avx2_search_encrypt_48_96,
    version = SpeckVersion::Speck48_96,
    bytes = 12,
    vector = __m256i,
    function = speck::avx2_encrypt_block_48_96,
    comparator = avx2_block_compare_u32,
    key_conversion = avx2_u24x4_key,
    new_key = avx2_new_key
);
define_search!(
    #[target_feature(enable = "avx2")]
    avx2_search_decrypt_48_96,
    version = SpeckVersion::Speck48_96,
    bytes = 12,
    vector = __m256i,
    function = speck::avx2_decrypt_block_48_96,
    comparator = avx2_block_compare_u32,
    key_conversion = avx2_u24x4_key,
    new_key = avx2_new_key
);
define_search!(
    #[target_feature(enable = "avx2")]
    avx2_search_encrypt_64_96,
    version = SpeckVersion::Speck64_96,
    bytes = 12,
    vector = __m256i,
    function = speck::avx2_encrypt_block_64_96,
    comparator = avx2_block_compare_u32,
    key_conversion = avx2_u32x3_key,
    new_key = avx2_new_key
);
define_search!(
    #[target_feature(enable = "avx2")]
    avx2_search_decrypt_64_96,
    version = SpeckVersion::Speck64_96,
    bytes = 12,
    vector = __m256i,
    function = speck::avx2_decrypt_block_64_96,
    comparator = avx2_block_compare_u32,
    key_conversion = avx2_u32x3_key,
    new_key = avx2_new_key
);
define_search!(
    #[target_feature(enable = "avx2")]
    avx2_search_encrypt_64_128,
    version = SpeckVersion::Speck64_128,
    bytes = 16,
    vector = __m256i,
    function = speck::avx2_encrypt_block_64_128,
    comparator = avx2_block_compare_u32,
    key_conversion = avx2_u32x4_key,
    new_key = avx2_new_key
);
define_search!(
    #[target_feature(enable = "avx2")]
    avx2_search_decrypt_64_128,
    version = SpeckVersion::Speck64_128,
    bytes = 16,
    vector = __m256i,
    function = speck::avx2_decrypt_block_64_128,
    comparator = avx2_block_compare_u32,
    key_conversion = avx2_u32x4_key,
    new_key = avx2_new_key
);
define_search!(
    #[target_feature(enable = "avx2")]
    avx2_search_encrypt_96_96,
    version = SpeckVersion::Speck96_96,
    bytes = 12,
    vector = __m256i,
    function = speck::avx2_encrypt_block_96_96,
    comparator = avx2_block_compare_u64,
    key_conversion = avx2_u48x2_key,
    new_key = avx2_new_key
);
define_search!(
    #[target_feature(enable = "avx2")]
    avx2_search_decrypt_96_96,
    version = SpeckVersion::Speck96_96,
    bytes = 12,
    vector = __m256i,
    function = speck::avx2_decrypt_block_96_96,
    comparator = avx2_block_compare_u64,
    key_conversion = avx2_u48x2_key,
    new_key = avx2_new_key
);
define_search!(
    #[target_feature(enable = "avx2")]
    avx2_search_encrypt_96_144,
    version = SpeckVersion::Speck96_144,
    bytes = 18,
    vector = __m256i,
    function = speck::avx2_encrypt_block_96_144,
    comparator = avx2_block_compare_u64,
    key_conversion = avx2_u48x3_key,
    new_key = avx2_new_key
);
define_search!(
    #[target_feature(enable = "avx2")]
    avx2_search_decrypt_96_144,
    version = SpeckVersion::Speck96_144,
    bytes = 18,
    vector = __m256i,
    function = speck::avx2_decrypt_block_96_144,
    comparator = avx2_block_compare_u64,
    key_conversion = avx2_u48x3_key,
    new_key = avx2_new_key
);
define_search!(
    #[target_feature(enable = "avx2")]
    avx2_search_encrypt_128_128,
    version = SpeckVersion::Speck128_128,
    bytes = 16,
    vector = __m256i,
    function = speck::avx2_encrypt_block_128_128,
    comparator = avx2_block_compare_u64,
    key_conversion = avx2_u64x2_key,
    new_key = avx2_new_key
);
define_search!(
    #[target_feature(enable = "avx2")]
    avx2_search_decrypt_128_128,
    version = SpeckVersion::Speck128_128,
    bytes = 16,
    vector = __m256i,
    function = speck::avx2_decrypt_block_128_128,
    comparator = avx2_block_compare_u64,
    key_conversion = avx2_u64x2_key,
    new_key = avx2_new_key
);
define_search!(
    #[target_feature(enable = "avx2")]
    avx2_search_encrypt_128_192,
    version = SpeckVersion::Speck128_192,
    bytes = 24,
    vector = __m256i,
    function = speck::avx2_encrypt_block_128_192,
    comparator = avx2_block_compare_u64,
    key_conversion = avx2_u64x3_key,
    new_key = avx2_new_key
);
define_search!(
    #[target_feature(enable = "avx2")]
    avx2_search_decrypt_128_192,
    version = SpeckVersion::Speck128_192,
    bytes = 24,
    vector = __m256i,
    function = speck::avx2_decrypt_block_128_192,
    comparator = avx2_block_compare_u64,
    key_conversion = avx2_u64x3_key,
    new_key = avx2_new_key
);
define_search!(
    #[target_feature(enable = "avx2")]
    avx2_search_encrypt_128_256,
    version = SpeckVersion::Speck128_256,
    bytes = 32,
    vector = __m256i,
    function = speck::avx2_encrypt_block_128_256,
    comparator = avx2_block_compare_u64,
    key_conversion = avx2_u64x4_key,
    new_key = avx2_new_key
);
define_search!(
    #[target_feature(enable = "avx2")]
    avx2_search_decrypt_128_256,
    version = SpeckVersion::Speck128_256,
    bytes = 32,
    vector = __m256i,
    function = speck::avx2_decrypt_block_128_256,
    comparator = avx2_block_compare_u64,
    key_conversion = avx2_u64x4_key,
    new_key = avx2_new_key
);

define_search!(
    #[target_feature(enable = "avx512f,avx512bw")]
    avx512_search_encrypt_32_64,
    version = SpeckVersion::Speck32_64,
    bytes = 8,
    vector = __m512i,
    function = speck::avx512_encrypt_block_32_64,
    comparator = avx512_block_compare_u16,
    key_conversion = avx512_u16x4_key,
    new_key = avx512_new_key
);
define_search!(
    #[target_feature(enable = "avx512f,avx512bw")]
    avx512_search_decrypt_32_64,
    version = SpeckVersion::Speck32_64,
    bytes = 8,
    vector = __m512i,
    function = speck::avx512_decrypt_block_32_64,
    comparator = avx512_block_compare_u16,
    key_conversion = avx512_u16x4_key,
    new_key = avx512_new_key
);
define_search!(
    #[target_feature(enable = "avx512f")]
    avx512_search_encrypt_48_72,
    version = SpeckVersion::Speck48_72,
    bytes = 9,
    vector = __m512i,
    function = speck::avx512_encrypt_block_48_72,
    comparator = avx512_block_compare_u32,
    key_conversion = avx512_u24x3_key,
    new_key = avx512_new_key
);
define_search!(
    #[target_feature(enable = "avx512f")]
    avx512_search_decrypt_48_72,
    version = SpeckVersion::Speck48_72,
    bytes = 9,
    vector = __m512i,
    function = speck::avx512_decrypt_block_48_72,
    comparator = avx512_block_compare_u32,
    key_conversion = avx512_u24x3_key,
    new_key = avx512_new_key
);
define_search!(
    #[target_feature(enable = "avx512f")]
    avx512_search_encrypt_48_96,
    version = SpeckVersion::Speck48_96,
    bytes = 12,
    vector = __m512i,
    function = speck::avx512_encrypt_block_48_96,
    comparator = avx512_block_compare_u32,
    key_conversion = avx512_u24x4_key,
    new_key = avx512_new_key
);
define_search!(
    #[target_feature(enable = "avx512f")]
    avx512_search_decrypt_48_96,
    version = SpeckVersion::Speck48_96,
    bytes = 12,
    vector = __m512i,
    function = speck::avx512_decrypt_block_48_96,
    comparator = avx512_block_compare_u32,
    key_conversion = avx512_u24x4_key,
    new_key = avx512_new_key
);
define_search!(
    #[target_feature(enable = "avx512f")]
    avx512_search_encrypt_64_96,
    version = SpeckVersion::Speck64_96,
    bytes = 12,
    vector = __m512i,
    function = speck::avx512_encrypt_block_64_96,
    comparator = avx512_block_compare_u32,
    key_conversion = avx512_u32x3_key,
    new_key = avx512_new_key
);
define_search!(
    #[target_feature(enable = "avx512f")]
    avx512_search_decrypt_64_96,
    version = SpeckVersion::Speck64_96,
    bytes = 12,
    vector = __m512i,
    function = speck::avx512_decrypt_block_64_96,
    comparator = avx512_block_compare_u32,
    key_conversion = avx512_u32x3_key,
    new_key = avx512_new_key
);
define_search!(
    #[target_feature(enable = "avx512f")]
    avx512_search_encrypt_64_128,
    version = SpeckVersion::Speck64_128,
    bytes = 16,
    vector = __m512i,
    function = speck::avx512_encrypt_block_64_128,
    comparator = avx512_block_compare_u32,
    key_conversion = avx512_u32x4_key,
    new_key = avx512_new_key
);
define_search!(
    #[target_feature(enable = "avx512f")]
    avx512_search_decrypt_64_128,
    version = SpeckVersion::Speck64_128,
    bytes = 16,
    vector = __m512i,
    function = speck::avx512_decrypt_block_64_128,
    comparator = avx512_block_compare_u32,
    key_conversion = avx512_u32x4_key,
    new_key = avx512_new_key
);
define_search!(
    #[target_feature(enable = "avx512f")]
    avx512_search_encrypt_96_96,
    version = SpeckVersion::Speck96_96,
    bytes = 12,
    vector = __m512i,
    function = speck::avx512_encrypt_block_96_96,
    comparator = avx512_block_compare_u64,
    key_conversion = avx512_u48x2_key,
    new_key = avx512_new_key
);
define_search!(
    #[target_feature(enable = "avx512f")]
    avx512_search_decrypt_96_96,
    version = SpeckVersion::Speck96_96,
    bytes = 12,
    vector = __m512i,
    function = speck::avx512_decrypt_block_96_96,
    comparator = avx512_block_compare_u64,
    key_conversion = avx512_u48x2_key,
    new_key = avx512_new_key
);
define_search!(
    #[target_feature(enable = "avx512f")]
    avx512_search_encrypt_96_144,
    version = SpeckVersion::Speck96_144,
    bytes = 18,
    vector = __m512i,
    function = speck::avx512_encrypt_block_96_144,
    comparator = avx512_block_compare_u64,
    key_conversion = avx512_u48x3_key,
    new_key = avx512_new_key
);
define_search!(
    #[target_feature(enable = "avx512f")]
    avx512_search_decrypt_96_144,
    version = SpeckVersion::Speck96_144,
    bytes = 18,
    vector = __m512i,
    function = speck::avx512_decrypt_block_96_144,
    comparator = avx512_block_compare_u64,
    key_conversion = avx512_u48x3_key,
    new_key = avx512_new_key
);
define_search!(
    #[target_feature(enable = "avx512f")]
    avx512_search_encrypt_128_128,
    version = SpeckVersion::Speck128_128,
    bytes = 16,
    vector = __m512i,
    function = speck::avx512_encrypt_block_128_128,
    comparator = avx512_block_compare_u64,
    key_conversion = avx512_u64x2_key,
    new_key = avx512_new_key
);
define_search!(
    #[target_feature(enable = "avx512f")]
    avx512_search_decrypt_128_128,
    version = SpeckVersion::Speck128_128,
    bytes = 16,
    vector = __m512i,
    function = speck::avx512_decrypt_block_128_128,
    comparator = avx512_block_compare_u64,
    key_conversion = avx512_u64x2_key,
    new_key = avx512_new_key
);
define_search!(
    #[target_feature(enable = "avx512f")]
    avx512_search_encrypt_128_192,
    version = SpeckVersion::Speck128_192,
    bytes = 24,
    vector = __m512i,
    function = speck::avx512_encrypt_block_128_192,
    comparator = avx512_block_compare_u64,
    key_conversion = avx512_u64x3_key,
    new_key = avx512_new_key
);
define_search!(
    #[target_feature(enable = "avx512f")]
    avx512_search_decrypt_128_192,
    version = SpeckVersion::Speck128_192,
    bytes = 24,
    vector = __m512i,
    function = speck::avx512_decrypt_block_128_192,
    comparator = avx512_block_compare_u64,
    key_conversion = avx512_u64x3_key,
    new_key = avx512_new_key
);
define_search!(
    #[target_feature(enable = "avx512f")]
    avx512_search_encrypt_128_256,
    version = SpeckVersion::Speck128_256,
    bytes = 32,
    vector = __m512i,
    function = speck::avx512_encrypt_block_128_256,
    comparator = avx512_block_compare_u64,
    key_conversion = avx512_u64x4_key,
    new_key = avx512_new_key
);
define_search!(
    #[target_feature(enable = "avx512f")]
    avx512_search_decrypt_128_256,
    version = SpeckVersion::Speck128_256,
    bytes = 32,
    vector = __m512i,
    function = speck::avx512_decrypt_block_128_256,
    comparator = avx512_block_compare_u64,
    key_conversion = avx512_u64x4_key,
    new_key = avx512_new_key
);
