use crate::domain::key::Key;
use paste::paste;

macro_rules! define_validate {
    (
    bytes = $bytes:literal,
    vector = $vector:ty,
    comparator = $comparator:path,
    key_conversion = $key_conversion:ident,
    name = $name:tt
    ) => {
        paste! {
            #[inline(always)]
            pub fn [<ecb_validate_encrypt_ $name>]<const PREFIX: usize>(
                pt: &[[$vector; 2]],
                expected: &[[$vector; 2]],
                key: &Key<$bytes, PREFIX>,
            ) -> bool {
                for (p, e) in pt.iter().zip(expected) {
                    let result = speck::[<encrypt_block_ $name>](*p, key.$key_conversion());
                    if !(result[0] == e[0] && result[1] == e[1]) {
                        return false;
                    }
                }

                true
            }

            #[inline(always)]
            pub fn [<ecb_validate_decrypt_ $name>]<const PREFIX: usize>(
                ct: &[[$vector; 2]],
                expected: &[[$vector; 2]],
                key: &Key<$bytes, PREFIX>,
            ) -> bool {
                for (c, e) in ct.iter().zip(expected) {
                    let result = speck::[<decrypt_block_ $name>](*c, key.$key_conversion());
                    if !(result[0] == e[0] && result[1] == e[1]) {
                        return false;
                    }
                }

                true
            }
        }
    };
}

define_validate!(
    bytes = 8,
    vector = u16,
    comparator = block_compare,
    key_conversion = as_u16x4_le,
    name = 32_64
);
define_validate!(
    bytes = 9,
    vector = u32,
    comparator = block_compare,
    key_conversion = as_u24x3_le,
    name = 48_72
);
define_validate!(
    bytes = 12,
    vector = u32,
    comparator = block_compare,
    key_conversion = as_u24x4_le,
    name = 48_96
);
define_validate!(
    bytes = 12,
    vector = u32,
    comparator = block_compare,
    key_conversion = as_u32x3_le,
    name = 64_96
);
define_validate!(
    bytes = 16,
    vector = u32,
    comparator = block_compare,
    key_conversion = as_u32x4_le,
    name = 64_128
);
define_validate!(
    bytes = 12,
    vector = u64,
    comparator = block_compare,
    key_conversion = as_u48x2_le,
    name = 96_96
);
define_validate!(
    bytes = 18,
    vector = u64,
    comparator = block_compare,
    key_conversion = as_u48x3_le,
    name = 96_144
);
define_validate!(
    bytes = 16,
    vector = u64,
    comparator = block_compare,
    key_conversion = as_u64x2_le,
    name = 128_128
);
define_validate!(
    bytes = 24,
    vector = u64,
    comparator = block_compare,
    key_conversion = as_u64x3_le,
    name = 128_192
);
define_validate!(
    bytes = 32,
    vector = u64,
    comparator = block_compare,
    key_conversion = as_u64x4_le,
    name = 128_256
);
