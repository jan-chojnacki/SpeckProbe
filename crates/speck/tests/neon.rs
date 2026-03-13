mod common;

#[cfg(test)]
#[cfg(target_arch = "aarch64")]
mod test {
    use crate::common::define_speck_test_simd;
    use speck::{
        neon_decrypt_block_128_128, neon_decrypt_block_128_192, neon_decrypt_block_128_256,
        neon_decrypt_block_32_64, neon_decrypt_block_48_72, neon_decrypt_block_48_96,
        neon_decrypt_block_64_128, neon_decrypt_block_64_96, neon_decrypt_block_96_144,
        neon_decrypt_block_96_96, neon_encrypt_block_128_128, neon_encrypt_block_128_192,
        neon_encrypt_block_128_256, neon_encrypt_block_32_64, neon_encrypt_block_48_72,
        neon_encrypt_block_48_96, neon_encrypt_block_64_128, neon_encrypt_block_64_96,
        neon_encrypt_block_96_144, neon_encrypt_block_96_96,
    };
    use std::arch::aarch64::{vdupq_n_u16, vdupq_n_u32, vdupq_n_u64};

    define_speck_test_simd!(
        "neon_encrypt_block_32_64",
        neon_encrypt_block_32_64,
        "aarch64",
        "neon",
        key = [
            vdupq_n_u16(0x1918),
            vdupq_n_u16(0x1110),
            vdupq_n_u16(0x0908),
            vdupq_n_u16(0x0100),
        ],
        data = [vdupq_n_u16(0x6574), vdupq_n_u16(0x694c)],
        expected = [vdupq_n_u16(0xa868u16), vdupq_n_u16(0x42f2)]
    );

    define_speck_test_simd!(
        "neon_encrypt_block_48_72",
        neon_encrypt_block_48_72,
        "aarch64",
        "neon",
        key = [
            vdupq_n_u32(0x121110),
            vdupq_n_u32(0x0a0908),
            vdupq_n_u32(0x020100),
        ],
        data = [vdupq_n_u32(0x20796c), vdupq_n_u32(0x6c6172)],
        expected = [vdupq_n_u32(0xc049a5), vdupq_n_u32(0x385adc)]
    );

    define_speck_test_simd!(
        "neon_encrypt_block_48_96",
        neon_encrypt_block_48_96,
        "aarch64",
        "neon",
        key = [
            vdupq_n_u32(0x1a1918),
            vdupq_n_u32(0x121110),
            vdupq_n_u32(0x0a0908),
            vdupq_n_u32(0x020100),
        ],
        data = [vdupq_n_u32(0x6d2073), vdupq_n_u32(0x696874)],
        expected = [vdupq_n_u32(0x735e10), vdupq_n_u32(0xb6445d)]
    );

    define_speck_test_simd!(
        "neon_encrypt_block_64_96",
        neon_encrypt_block_64_96,
        "aarch64",
        "neon",
        key = [
            vdupq_n_u32(0x13121110),
            vdupq_n_u32(0x0b0a0908),
            vdupq_n_u32(0x03020100),
        ],
        data = [vdupq_n_u32(0x74614620), vdupq_n_u32(0x736e6165)],
        expected = [vdupq_n_u32(0x9f7952ec), vdupq_n_u32(0x4175946c)]
    );

    define_speck_test_simd!(
        "neon_encrypt_block_64_128",
        neon_encrypt_block_64_128,
        "aarch64",
        "neon",
        key = [
            vdupq_n_u32(0x1b1a1918),
            vdupq_n_u32(0x13121110),
            vdupq_n_u32(0x0b0a0908),
            vdupq_n_u32(0x03020100),
        ],
        data = [vdupq_n_u32(0x3b726574), vdupq_n_u32(0x7475432d)],
        expected = [vdupq_n_u32(0x8c6fa548), vdupq_n_u32(0x454e028b)]
    );

    define_speck_test_simd!(
        "neon_encrypt_block_96_96",
        neon_encrypt_block_96_96,
        "aarch64",
        "neon",
        key = [vdupq_n_u64(0x0d0c0b0a0908), vdupq_n_u64(0x050403020100),],
        data = [vdupq_n_u64(0x65776f68202c), vdupq_n_u64(0x656761737520)],
        expected = [vdupq_n_u64(0x9e4d09ab7178), vdupq_n_u64(0x62bdde8f79aa)]
    );

    define_speck_test_simd!(
        "neon_encrypt_block_96_144",
        neon_encrypt_block_96_144,
        "aarch64",
        "neon",
        key = [
            vdupq_n_u64(0x151413121110),
            vdupq_n_u64(0x0d0c0b0a0908),
            vdupq_n_u64(0x050403020100),
        ],
        data = [vdupq_n_u64(0x656d6974206e), vdupq_n_u64(0x69202c726576)],
        expected = [vdupq_n_u64(0x2bf31072228a), vdupq_n_u64(0x7ae440252ee6)]
    );

    define_speck_test_simd!(
        "neon_encrypt_block_128_128",
        neon_encrypt_block_128_128,
        "aarch64",
        "neon",
        key = [
            vdupq_n_u64(0x0f0e0d0c0b0a0908),
            vdupq_n_u64(0x0706050403020100),
        ],
        data = [
            vdupq_n_u64(0x6c61766975716520),
            vdupq_n_u64(0x7469206564616d20)
        ],
        expected = [
            vdupq_n_u64(0xa65d985179783265),
            vdupq_n_u64(0x7860fedf5c570d18)
        ]
    );

    define_speck_test_simd!(
        "neon_encrypt_block_128_192",
        neon_encrypt_block_128_192,
        "aarch64",
        "neon",
        key = [
            vdupq_n_u64(0x1716151413121110),
            vdupq_n_u64(0x0f0e0d0c0b0a0908),
            vdupq_n_u64(0x0706050403020100),
        ],
        data = [
            vdupq_n_u64(0x7261482066656968),
            vdupq_n_u64(0x43206f7420746e65)
        ],
        expected = [
            vdupq_n_u64(0x1be4cf3a13135566),
            vdupq_n_u64(0xf9bc185de03c1886)
        ]
    );

    define_speck_test_simd!(
        "neon_encrypt_block_128_256",
        neon_encrypt_block_128_256,
        "aarch64",
        "neon",
        key = [
            vdupq_n_u64(0x1f1e1d1c1b1a1918),
            vdupq_n_u64(0x1716151413121110),
            vdupq_n_u64(0x0f0e0d0c0b0a0908),
            vdupq_n_u64(0x0706050403020100),
        ],
        data = [
            vdupq_n_u64(0x65736f6874206e49),
            vdupq_n_u64(0x202e72656e6f6f70)
        ],
        expected = [
            vdupq_n_u64(0x4109010405c0f53e),
            vdupq_n_u64(0x4eeeb48d9c188f43)
        ]
    );

    define_speck_test_simd!(
        "neon_decrypt_block_32_64",
        neon_decrypt_block_32_64,
        "aarch64",
        "neon",
        key = [
            vdupq_n_u16(0x1918),
            vdupq_n_u16(0x1110),
            vdupq_n_u16(0x0908),
            vdupq_n_u16(0x0100),
        ],
        data = [vdupq_n_u16(0xa868), vdupq_n_u16(0x42f2)],
        expected = [vdupq_n_u16(0x6574), vdupq_n_u16(0x694c)]
    );

    define_speck_test_simd!(
        "neon_decrypt_block_48_72",
        neon_decrypt_block_48_72,
        "aarch64",
        "neon",
        key = [
            vdupq_n_u32(0x121110),
            vdupq_n_u32(0x0a0908),
            vdupq_n_u32(0x020100),
        ],
        data = [vdupq_n_u32(0xc049a5), vdupq_n_u32(0x385adc)],
        expected = [vdupq_n_u32(0x20796c), vdupq_n_u32(0x6c6172)]
    );

    define_speck_test_simd!(
        "neon_decrypt_block_48_96",
        neon_decrypt_block_48_96,
        "aarch64",
        "neon",
        key = [
            vdupq_n_u32(0x1a1918),
            vdupq_n_u32(0x121110),
            vdupq_n_u32(0x0a0908),
            vdupq_n_u32(0x020100),
        ],
        data = [vdupq_n_u32(0x735e10), vdupq_n_u32(0xb6445d)],
        expected = [vdupq_n_u32(0x6d2073), vdupq_n_u32(0x696874)]
    );

    define_speck_test_simd!(
        "neon_decrypt_block_64_96",
        neon_decrypt_block_64_96,
        "aarch64",
        "neon",
        key = [
            vdupq_n_u32(0x13121110),
            vdupq_n_u32(0x0b0a0908),
            vdupq_n_u32(0x03020100),
        ],
        data = [vdupq_n_u32(0x9f7952ec), vdupq_n_u32(0x4175946c)],
        expected = [vdupq_n_u32(0x74614620), vdupq_n_u32(0x736e6165)]
    );

    define_speck_test_simd!(
        "neon_decrypt_block_64_128",
        neon_decrypt_block_64_128,
        "aarch64",
        "neon",
        key = [
            vdupq_n_u32(0x1b1a1918),
            vdupq_n_u32(0x13121110),
            vdupq_n_u32(0x0b0a0908),
            vdupq_n_u32(0x03020100),
        ],
        data = [vdupq_n_u32(0x8c6fa548), vdupq_n_u32(0x454e028b)],
        expected = [vdupq_n_u32(0x3b726574), vdupq_n_u32(0x7475432d)]
    );

    define_speck_test_simd!(
        "neon_decrypt_block_96_96",
        neon_decrypt_block_96_96,
        "aarch64",
        "neon",
        key = [vdupq_n_u64(0x0d0c0b0a0908), vdupq_n_u64(0x050403020100),],
        data = [vdupq_n_u64(0x9e4d09ab7178), vdupq_n_u64(0x62bdde8f79aa)],
        expected = [vdupq_n_u64(0x65776f68202c), vdupq_n_u64(0x656761737520)]
    );

    define_speck_test_simd!(
        "neon_decrypt_block_96_144",
        neon_decrypt_block_96_144,
        "aarch64",
        "neon",
        key = [
            vdupq_n_u64(0x151413121110),
            vdupq_n_u64(0x0d0c0b0a0908),
            vdupq_n_u64(0x050403020100),
        ],
        data = [vdupq_n_u64(0x2bf31072228a), vdupq_n_u64(0x7ae440252ee6)],
        expected = [vdupq_n_u64(0x656d6974206e), vdupq_n_u64(0x69202c726576)]
    );

    define_speck_test_simd!(
        "neon_decrypt_block_128_128",
        neon_decrypt_block_128_128,
        "aarch64",
        "neon",
        key = [
            vdupq_n_u64(0x0f0e0d0c0b0a0908),
            vdupq_n_u64(0x0706050403020100),
        ],
        data = [
            vdupq_n_u64(0xa65d985179783265),
            vdupq_n_u64(0x7860fedf5c570d18)
        ],
        expected = [
            vdupq_n_u64(0x6c61766975716520),
            vdupq_n_u64(0x7469206564616d20)
        ]
    );

    define_speck_test_simd!(
        "neon_decrypt_block_128_192",
        neon_decrypt_block_128_192,
        "aarch64",
        "neon",
        key = [
            vdupq_n_u64(0x1716151413121110),
            vdupq_n_u64(0x0f0e0d0c0b0a0908),
            vdupq_n_u64(0x0706050403020100),
        ],
        data = [
            vdupq_n_u64(0x1be4cf3a13135566),
            vdupq_n_u64(0xf9bc185de03c1886)
        ],
        expected = [
            vdupq_n_u64(0x7261482066656968),
            vdupq_n_u64(0x43206f7420746e65)
        ]
    );

    define_speck_test_simd!(
        "neon_decrypt_block_128_256",
        neon_decrypt_block_128_256,
        "aarch64",
        "neon",
        key = [
            vdupq_n_u64(0x1f1e1d1c1b1a1918),
            vdupq_n_u64(0x1716151413121110),
            vdupq_n_u64(0x0f0e0d0c0b0a0908),
            vdupq_n_u64(0x0706050403020100),
        ],
        data = [
            vdupq_n_u64(0x4109010405c0f53e),
            vdupq_n_u64(0x4eeeb48d9c188f43)
        ],
        expected = [
            vdupq_n_u64(0x65736f6874206e49),
            vdupq_n_u64(0x202e72656e6f6f70)
        ]
    );
}
