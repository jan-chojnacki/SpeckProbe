use crate::aarch64::neon::comparator::{
    neon_block_compare_u16, neon_block_compare_u32, neon_block_compare_u64,
};
use crate::domain::key::Key;
use crate::domain::key_iterator::KeyIterator;
use crate::domain::task::Task;
use paste::paste;
use speck::SpeckVersion;
use std::arch::aarch64::{uint16x8_t, uint32x4_t, uint64x2_t};

macro_rules! define_search {
    (
    $(#[$meta:meta])*
    version = $version:path,
    bytes = $bytes:literal,
    vector = $vector:ty,
    comparator = $comparator:path,
    key_conversion = $key_conversion:ident,
    new_key = $new_key:ident,
    name = $name:tt,
    simd = $simd:tt
    ) => {paste! {
        $(#[$meta])*
        pub fn [<$simd _search_encrypt_ $name>] <const PREFIX: usize>(
            task: Task<$vector, $bytes, PREFIX>,
            out: &mut Vec<Key<$bytes, PREFIX>>,
        ) {
            let mut iter =
                KeyIterator::<$bytes, PREFIX>::new(task.start, task.end, task.prefix, $version);
            let mut key = iter.$new_key();

            while iter.simd_next_into(&mut key).is_some() {
                let result = speck::[<$simd _encrypt_block_ $name>](task.data, key.$key_conversion());
                $comparator(&task.expected, &result, &key, out);
            }
        }

        $(#[$meta])*
        pub fn [<$simd _search_encrypt_inflight_ $name>]<const PREFIX: usize>(
            task: Task<$vector, $bytes, PREFIX>,
            out: &mut Vec<Key<$bytes, PREFIX>>,
        ) {
            let mut iter =
                KeyIterator::<$bytes, PREFIX>::new(task.start, task.end, task.prefix, $version);
            let mut key = iter.$new_key();

            while iter.simd_next_into(&mut key).is_some() {
                let result = speck::[<$simd _encrypt_block_inflight_ $name>](task.data, key.$key_conversion());
                $comparator(&task.expected, &result, &key, out);
            }
        }

        $(#[$meta])*
        pub fn [<$simd _search_decrypt_ $name>]<const PREFIX: usize>(
            task: Task<$vector, $bytes, PREFIX>,
            out: &mut Vec<Key<$bytes, PREFIX>>,
        ) {
            let mut iter =
                KeyIterator::<$bytes, PREFIX>::new(task.start, task.end, task.prefix, $version);
            let mut key = iter.$new_key();

            while iter.simd_next_into(&mut key).is_some() {
                let result = speck::[<$simd _decrypt_block_ $name>](task.data, key.$key_conversion());
                $comparator(&task.expected, &result, &key, out);
            }
        }
    }};
}

define_search!(
    #[target_feature(enable = "neon")]
    version = SpeckVersion::Speck32_64,
    bytes = 8,
    vector = uint16x8_t,
    comparator = neon_block_compare_u16,
    key_conversion = neon_u16x4_key,
    new_key = neon_new_key,
    name = 32_64,
    simd = neon
);
define_search!(
    #[target_feature(enable = "neon")]
    version = SpeckVersion::Speck48_72,
    bytes = 9,
    vector = uint32x4_t,
    comparator = neon_block_compare_u32,
    key_conversion = neon_u24x3_key,
    new_key = neon_new_key,
    name = 48_72,
    simd = neon
);
define_search!(
    #[target_feature(enable = "neon")]
    version = SpeckVersion::Speck48_96,
    bytes = 12,
    vector = uint32x4_t,
    comparator = neon_block_compare_u32,
    key_conversion = neon_u24x4_key,
    new_key = neon_new_key,
    name = 48_96,
    simd = neon
);
define_search!(
    #[target_feature(enable = "neon")]
    version = SpeckVersion::Speck64_96,
    bytes = 12,
    vector = uint32x4_t,
    comparator = neon_block_compare_u32,
    key_conversion = neon_u32x3_key,
    new_key = neon_new_key,
    name = 64_96,
    simd = neon
);
define_search!(
    #[target_feature(enable = "neon")]
    version = SpeckVersion::Speck64_128,
    bytes = 16,
    vector = uint32x4_t,
    comparator = neon_block_compare_u32,
    key_conversion = neon_u32x4_key,
    new_key = neon_new_key,
    name = 64_128,
    simd = neon
);
define_search!(
    #[target_feature(enable = "neon")]
    version = SpeckVersion::Speck96_96,
    bytes = 12,
    vector = uint64x2_t,
    comparator = neon_block_compare_u64,
    key_conversion = neon_u48x2_key,
    new_key = neon_new_key,
    name = 96_96,
    simd = neon
);
define_search!(
    #[target_feature(enable = "neon")]
    version = SpeckVersion::Speck96_144,
    bytes = 18,
    vector = uint64x2_t,
    comparator = neon_block_compare_u64,
    key_conversion = neon_u48x3_key,
    new_key = neon_new_key,
    name = 96_144,
    simd = neon
);
define_search!(
    #[target_feature(enable = "neon")]
    version = SpeckVersion::Speck128_128,
    bytes = 16,
    vector = uint64x2_t,
    comparator = neon_block_compare_u64,
    key_conversion = neon_u64x2_key,
    new_key = neon_new_key,
    name = 128_128,
    simd = neon
);
define_search!(
    #[target_feature(enable = "neon")]
    version = SpeckVersion::Speck128_192,
    bytes = 24,
    vector = uint64x2_t,
    comparator = neon_block_compare_u64,
    key_conversion = neon_u64x3_key,
    new_key = neon_new_key,
    name = 128_192,
    simd = neon
);
define_search!(
    #[target_feature(enable = "neon")]
    version = SpeckVersion::Speck128_256,
    bytes = 32,
    vector = uint64x2_t,
    comparator = neon_block_compare_u64,
    key_conversion = neon_u64x4_key,
    new_key = neon_new_key,
    name = 128_256,
    simd = neon
);
