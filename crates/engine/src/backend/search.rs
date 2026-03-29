use crate::backend::scalar::comparator::block_compare;
use crate::domain::key::Key;
use crate::domain::key_iterator::KeyIterator;
use crate::domain::task::Task;
use speck::SpeckVersion;

macro_rules! define_search {
    (name = $name:ident, version = $version:path, bytes = $bytes:literal, vector = $vector:ty, function = $function:path, comparator = $comparator:path, key_conversion = $key_conversion:ident) => {
        #[inline(always)]
        pub fn $name<const PREFIX: usize>(
            task: Task<$vector, $bytes, PREFIX>,
            out: &mut Vec<Key<$bytes, PREFIX>>,
        ) {
            let mut iter =
                KeyIterator::<$bytes, PREFIX>::new(task.start, task.end, task.prefix, $version);
            let mut key = iter.new_key();

            while iter.next_into(&mut key).is_some() {
                let result = $function(task.data, key.$key_conversion());
                $comparator(&task.expected, &result, &key, out);
            }
        }
    };
}

define_search!(
    name = search_encrypt_32_64,
    version = SpeckVersion::Speck32_64,
    bytes = 8,
    vector = u16,
    function = speck::encrypt_block_32_64,
    comparator = block_compare,
    key_conversion = as_u16x4_le
);
define_search!(
    name = search_decrypt_32_64,
    version = SpeckVersion::Speck32_64,
    bytes = 8,
    vector = u16,
    function = speck::decrypt_block_32_64,
    comparator = block_compare,
    key_conversion = as_u16x4_le
);
define_search!(
    name = search_encrypt_48_72,
    version = SpeckVersion::Speck48_72,
    bytes = 9,
    vector = u32,
    function = speck::encrypt_block_48_72,
    comparator = block_compare,
    key_conversion = as_u24x3_le
);
define_search!(
    name = search_decrypt_48_72,
    version = SpeckVersion::Speck48_72,
    bytes = 9,
    vector = u32,
    function = speck::decrypt_block_48_72,
    comparator = block_compare,
    key_conversion = as_u24x3_le
);
define_search!(
    name = search_encrypt_48_96,
    version = SpeckVersion::Speck48_96,
    bytes = 12,
    vector = u32,
    function = speck::encrypt_block_48_96,
    comparator = block_compare,
    key_conversion = as_u24x4_le
);
define_search!(
    name = search_decrypt_48_96,
    version = SpeckVersion::Speck48_96,
    bytes = 12,
    vector = u32,
    function = speck::decrypt_block_48_96,
    comparator = block_compare,
    key_conversion = as_u24x4_le
);
define_search!(
    name = search_encrypt_64_96,
    version = SpeckVersion::Speck64_96,
    bytes = 12,
    vector = u32,
    function = speck::encrypt_block_64_96,
    comparator = block_compare,
    key_conversion = as_u32x3_le
);
define_search!(
    name = search_decrypt_64_96,
    version = SpeckVersion::Speck64_96,
    bytes = 12,
    vector = u32,
    function = speck::decrypt_block_64_96,
    comparator = block_compare,
    key_conversion = as_u32x3_le
);
define_search!(
    name = search_encrypt_64_128,
    version = SpeckVersion::Speck64_128,
    bytes = 16,
    vector = u32,
    function = speck::encrypt_block_64_128,
    comparator = block_compare,
    key_conversion = as_u32x4_le
);
define_search!(
    name = search_decrypt_64_128,
    version = SpeckVersion::Speck64_128,
    bytes = 16,
    vector = u32,
    function = speck::decrypt_block_64_128,
    comparator = block_compare,
    key_conversion = as_u32x4_le
);
define_search!(
    name = search_encrypt_96_96,
    version = SpeckVersion::Speck96_96,
    bytes = 12,
    vector = u64,
    function = speck::encrypt_block_96_96,
    comparator = block_compare,
    key_conversion = as_u48x2_le
);
define_search!(
    name = search_decrypt_96_96,
    version = SpeckVersion::Speck96_96,
    bytes = 12,
    vector = u64,
    function = speck::decrypt_block_96_96,
    comparator = block_compare,
    key_conversion = as_u48x2_le
);
define_search!(
    name = search_encrypt_96_144,
    version = SpeckVersion::Speck96_144,
    bytes = 18,
    vector = u64,
    function = speck::encrypt_block_96_144,
    comparator = block_compare,
    key_conversion = as_u48x3_le
);
define_search!(
    name = search_decrypt_96_144,
    version = SpeckVersion::Speck96_144,
    bytes = 18,
    vector = u64,
    function = speck::decrypt_block_96_144,
    comparator = block_compare,
    key_conversion = as_u48x3_le
);
define_search!(
    name = search_encrypt_128_128,
    version = SpeckVersion::Speck128_128,
    bytes = 16,
    vector = u64,
    function = speck::encrypt_block_128_128,
    comparator = block_compare,
    key_conversion = as_u64x2_le
);
define_search!(
    name = search_decrypt_128_128,
    version = SpeckVersion::Speck128_128,
    bytes = 16,
    vector = u64,
    function = speck::decrypt_block_128_128,
    comparator = block_compare,
    key_conversion = as_u64x2_le
);
define_search!(
    name = search_encrypt_128_192,
    version = SpeckVersion::Speck128_192,
    bytes = 24,
    vector = u64,
    function = speck::encrypt_block_128_192,
    comparator = block_compare,
    key_conversion = as_u64x3_le
);
define_search!(
    name = search_decrypt_128_192,
    version = SpeckVersion::Speck128_192,
    bytes = 24,
    vector = u64,
    function = speck::decrypt_block_128_192,
    comparator = block_compare,
    key_conversion = as_u64x3_le
);
define_search!(
    name = search_encrypt_128_256,
    version = SpeckVersion::Speck128_256,
    bytes = 32,
    vector = u64,
    function = speck::encrypt_block_128_256,
    comparator = block_compare,
    key_conversion = as_u64x4_le
);
define_search!(
    name = search_decrypt_128_256,
    version = SpeckVersion::Speck128_256,
    bytes = 32,
    vector = u64,
    function = speck::decrypt_block_128_256,
    comparator = block_compare,
    key_conversion = as_u64x4_le
);
