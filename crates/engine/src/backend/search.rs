use crate::backend::scalar::comparator::block_compare;
use crate::domain::key::Key;
use crate::domain::key_iterator::KeyIterator;
use crate::domain::task::Task;
use paste::paste;
use speck::SpeckVersion;

macro_rules! define_search {
    (
    version = $version:path,
    bytes = $bytes:literal,
    vector = $vector:ty,
    comparator = $comparator:path,
    key_conversion = $key_conversion:ident,
    name = $name:tt
    ) => {paste! {
        #[inline(always)]
        pub fn [<search_encrypt_ $name>] <const PREFIX: usize>(
            task: Task<$vector, $bytes, PREFIX>,
            out: &mut Vec<Key<$bytes, PREFIX>>,
        ) {
            let mut iter =
                KeyIterator::<$bytes, PREFIX>::new(task.start, task.end, task.prefix, $version);
            let mut key = iter.new_key();

            while iter.next_into(&mut key).is_some() {
                let result = speck::[<encrypt_block_ $name>](task.data, key.$key_conversion());
                $comparator(&task.expected, &result, &key, out);
            }
        }

        #[inline(always)]
        pub fn [<search_encrypt_inflight_ $name>]<const PREFIX: usize>(
            task: Task<$vector, $bytes, PREFIX>,
            out: &mut Vec<Key<$bytes, PREFIX>>,
        ) {
            let mut iter =
                KeyIterator::<$bytes, PREFIX>::new(task.start, task.end, task.prefix, $version);
            let mut key = iter.new_key();

            while iter.next_into(&mut key).is_some() {
                let result = speck::[<encrypt_block_inflight_ $name>](task.data, key.$key_conversion());
                $comparator(&task.expected, &result, &key, out);
            }
        }

        #[inline(always)]
        pub fn [<search_decrypt_ $name>]<const PREFIX: usize>(
            task: Task<$vector, $bytes, PREFIX>,
            out: &mut Vec<Key<$bytes, PREFIX>>,
        ) {
            let mut iter =
                KeyIterator::<$bytes, PREFIX>::new(task.start, task.end, task.prefix, $version);
            let mut key = iter.new_key();

            while iter.next_into(&mut key).is_some() {
                let result = speck::[<decrypt_block_ $name>](task.data, key.$key_conversion());
                $comparator(&task.expected, &result, &key, out);
            }
        }
    }};
}

define_search!(
    version = SpeckVersion::Speck32_64,
    bytes = 8,
    vector = u16,
    comparator = block_compare,
    key_conversion = as_u16x4_le,
    name = 32_64
);
define_search!(
    version = SpeckVersion::Speck48_72,
    bytes = 9,
    vector = u32,
    comparator = block_compare,
    key_conversion = as_u24x3_le,
    name = 48_72
);
define_search!(
    version = SpeckVersion::Speck48_96,
    bytes = 12,
    vector = u32,
    comparator = block_compare,
    key_conversion = as_u24x4_le,
    name = 48_96
);
define_search!(
    version = SpeckVersion::Speck64_96,
    bytes = 12,
    vector = u32,
    comparator = block_compare,
    key_conversion = as_u32x3_le,
    name = 64_96
);
define_search!(
    version = SpeckVersion::Speck64_128,
    bytes = 16,
    vector = u32,
    comparator = block_compare,
    key_conversion = as_u32x4_le,
    name = 64_128
);
define_search!(
    version = SpeckVersion::Speck96_96,
    bytes = 12,
    vector = u64,
    comparator = block_compare,
    key_conversion = as_u48x2_le,
    name = 96_96
);
define_search!(
    version = SpeckVersion::Speck96_144,
    bytes = 18,
    vector = u64,
    comparator = block_compare,
    key_conversion = as_u48x3_le,
    name = 96_144
);
define_search!(
    version = SpeckVersion::Speck128_128,
    bytes = 16,
    vector = u64,
    comparator = block_compare,
    key_conversion = as_u64x2_le,
    name = 128_128
);
define_search!(
    version = SpeckVersion::Speck128_192,
    bytes = 24,
    vector = u64,
    comparator = block_compare,
    key_conversion = as_u64x3_le,
    name = 128_192
);
define_search!(
    version = SpeckVersion::Speck128_256,
    bytes = 32,
    vector = u64,
    comparator = block_compare,
    key_conversion = as_u64x4_le,
    name = 128_256
);
