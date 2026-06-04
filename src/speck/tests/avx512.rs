#[cfg(test)]
#[cfg(all(
    target_arch = "x86_64",
    target_feature = "avx512f",
    target_feature = "avx512bw"
))]
mod test {
    use super::super::common::define_speck_test_x86_64_simd;
    use crate::speck::{
        avx512_decrypt_block_32_64, avx512_decrypt_block_48_72, avx512_decrypt_block_48_96,
        avx512_decrypt_block_64_96, avx512_decrypt_block_64_128, avx512_decrypt_block_96_96,
        avx512_decrypt_block_96_144, avx512_decrypt_block_128_128, avx512_decrypt_block_128_192,
        avx512_decrypt_block_128_256, avx512_encrypt_block_32_64, avx512_encrypt_block_48_72,
        avx512_encrypt_block_48_96, avx512_encrypt_block_64_96, avx512_encrypt_block_64_128,
        avx512_encrypt_block_96_96, avx512_encrypt_block_96_144, avx512_encrypt_block_128_128,
        avx512_encrypt_block_128_192, avx512_encrypt_block_128_256,
        avx512_encrypt_block_inflight_32_64, avx512_encrypt_block_inflight_48_72,
        avx512_encrypt_block_inflight_48_96, avx512_encrypt_block_inflight_64_96,
        avx512_encrypt_block_inflight_64_128, avx512_encrypt_block_inflight_96_96,
        avx512_encrypt_block_inflight_96_144, avx512_encrypt_block_inflight_128_128,
        avx512_encrypt_block_inflight_128_192, avx512_encrypt_block_inflight_128_256,
    };
    use std::arch::x86_64::{_mm512_set1_epi16, _mm512_set1_epi32, _mm512_set1_epi64};

    define_speck_test_x86_64_simd!(
        "avx512_encrypt_block_32_64",
        avx512_encrypt_block_32_64,
        "x86_64",
        "avx512bw",
        key = [
            _mm512_set1_epi16(0x1918),
            _mm512_set1_epi16(0x1110),
            _mm512_set1_epi16(0x0908),
            _mm512_set1_epi16(0x0100),
        ],
        data = [_mm512_set1_epi16(0x6574), _mm512_set1_epi16(0x694c)],
        expected = [
            _mm512_set1_epi16(0xa868u16 as i16),
            _mm512_set1_epi16(0x42f2)
        ]
    );

    define_speck_test_x86_64_simd!(
        "avx512_encrypt_block_48_72",
        avx512_encrypt_block_48_72,
        "x86_64",
        "avx512f",
        key = [
            _mm512_set1_epi32(0x121110),
            _mm512_set1_epi32(0x0a0908),
            _mm512_set1_epi32(0x020100),
        ],
        data = [_mm512_set1_epi32(0x20796c), _mm512_set1_epi32(0x6c6172)],
        expected = [_mm512_set1_epi32(0xc049a5), _mm512_set1_epi32(0x385adc)]
    );

    define_speck_test_x86_64_simd!(
        "avx512_encrypt_block_48_96",
        avx512_encrypt_block_48_96,
        "x86_64",
        "avx512f",
        key = [
            _mm512_set1_epi32(0x1a1918),
            _mm512_set1_epi32(0x121110),
            _mm512_set1_epi32(0x0a0908),
            _mm512_set1_epi32(0x020100),
        ],
        data = [_mm512_set1_epi32(0x6d2073), _mm512_set1_epi32(0x696874)],
        expected = [_mm512_set1_epi32(0x735e10), _mm512_set1_epi32(0xb6445d)]
    );

    define_speck_test_x86_64_simd!(
        "avx512_encrypt_block_64_96",
        avx512_encrypt_block_64_96,
        "x86_64",
        "avx512f",
        key = [
            _mm512_set1_epi32(0x13121110),
            _mm512_set1_epi32(0x0b0a0908),
            _mm512_set1_epi32(0x03020100),
        ],
        data = [_mm512_set1_epi32(0x74614620), _mm512_set1_epi32(0x736e6165)],
        expected = [
            _mm512_set1_epi32(0x9f7952ecu32 as i32),
            _mm512_set1_epi32(0x4175946c)
        ]
    );

    define_speck_test_x86_64_simd!(
        "avx512_encrypt_block_64_128",
        avx512_encrypt_block_64_128,
        "x86_64",
        "avx512f",
        key = [
            _mm512_set1_epi32(0x1b1a1918),
            _mm512_set1_epi32(0x13121110),
            _mm512_set1_epi32(0x0b0a0908),
            _mm512_set1_epi32(0x03020100),
        ],
        data = [_mm512_set1_epi32(0x3b726574), _mm512_set1_epi32(0x7475432d)],
        expected = [
            _mm512_set1_epi32(0x8c6fa548u32 as i32),
            _mm512_set1_epi32(0x454e028b)
        ]
    );

    define_speck_test_x86_64_simd!(
        "avx512_encrypt_block_96_96",
        avx512_encrypt_block_96_96,
        "x86_64",
        "avx512f",
        key = [
            _mm512_set1_epi64(0x0d0c0b0a0908),
            _mm512_set1_epi64(0x050403020100),
        ],
        data = [
            _mm512_set1_epi64(0x65776f68202c),
            _mm512_set1_epi64(0x656761737520)
        ],
        expected = [
            _mm512_set1_epi64(0x9e4d09ab7178),
            _mm512_set1_epi64(0x62bdde8f79aa)
        ]
    );

    define_speck_test_x86_64_simd!(
        "avx512_encrypt_block_96_144",
        avx512_encrypt_block_96_144,
        "x86_64",
        "avx512f",
        key = [
            _mm512_set1_epi64(0x151413121110),
            _mm512_set1_epi64(0x0d0c0b0a0908),
            _mm512_set1_epi64(0x050403020100),
        ],
        data = [
            _mm512_set1_epi64(0x656d6974206e),
            _mm512_set1_epi64(0x69202c726576)
        ],
        expected = [
            _mm512_set1_epi64(0x2bf31072228a),
            _mm512_set1_epi64(0x7ae440252ee6)
        ]
    );

    define_speck_test_x86_64_simd!(
        "avx512_encrypt_block_128_128",
        avx512_encrypt_block_128_128,
        "x86_64",
        "avx512f",
        key = [
            _mm512_set1_epi64(0x0f0e0d0c0b0a0908),
            _mm512_set1_epi64(0x0706050403020100),
        ],
        data = [
            _mm512_set1_epi64(0x6c61766975716520),
            _mm512_set1_epi64(0x7469206564616d20)
        ],
        expected = [
            _mm512_set1_epi64(0xa65d985179783265u64 as i64),
            _mm512_set1_epi64(0x7860fedf5c570d18)
        ]
    );

    define_speck_test_x86_64_simd!(
        "avx512_encrypt_block_128_192",
        avx512_encrypt_block_128_192,
        "x86_64",
        "avx512f",
        key = [
            _mm512_set1_epi64(0x1716151413121110),
            _mm512_set1_epi64(0x0f0e0d0c0b0a0908),
            _mm512_set1_epi64(0x0706050403020100),
        ],
        data = [
            _mm512_set1_epi64(0x7261482066656968),
            _mm512_set1_epi64(0x43206f7420746e65)
        ],
        expected = [
            _mm512_set1_epi64(0x1be4cf3a13135566),
            _mm512_set1_epi64(0xf9bc185de03c1886u64 as i64)
        ]
    );

    define_speck_test_x86_64_simd!(
        "avx512_encrypt_block_128_256",
        avx512_encrypt_block_128_256,
        "x86_64",
        "avx512f",
        key = [
            _mm512_set1_epi64(0x1f1e1d1c1b1a1918),
            _mm512_set1_epi64(0x1716151413121110),
            _mm512_set1_epi64(0x0f0e0d0c0b0a0908),
            _mm512_set1_epi64(0x0706050403020100),
        ],
        data = [
            _mm512_set1_epi64(0x65736f6874206e49),
            _mm512_set1_epi64(0x202e72656e6f6f70)
        ],
        expected = [
            _mm512_set1_epi64(0x4109010405c0f53e),
            _mm512_set1_epi64(0x4eeeb48d9c188f43)
        ]
    );

    define_speck_test_x86_64_simd!(
        "avx512_encrypt_block_inflight_32_64",
        avx512_encrypt_block_inflight_32_64,
        "x86_64",
        "avx512bw",
        key = [
            _mm512_set1_epi16(0x1918),
            _mm512_set1_epi16(0x1110),
            _mm512_set1_epi16(0x0908),
            _mm512_set1_epi16(0x0100),
        ],
        data = [_mm512_set1_epi16(0x6574), _mm512_set1_epi16(0x694c)],
        expected = [
            _mm512_set1_epi16(0xa868u16 as i16),
            _mm512_set1_epi16(0x42f2)
        ]
    );

    define_speck_test_x86_64_simd!(
        "avx512_encrypt_block_inflight_48_72",
        avx512_encrypt_block_inflight_48_72,
        "x86_64",
        "avx512f",
        key = [
            _mm512_set1_epi32(0x121110),
            _mm512_set1_epi32(0x0a0908),
            _mm512_set1_epi32(0x020100),
        ],
        data = [_mm512_set1_epi32(0x20796c), _mm512_set1_epi32(0x6c6172)],
        expected = [_mm512_set1_epi32(0xc049a5), _mm512_set1_epi32(0x385adc)]
    );

    define_speck_test_x86_64_simd!(
        "avx512_encrypt_block_inflight_48_96",
        avx512_encrypt_block_inflight_48_96,
        "x86_64",
        "avx512f",
        key = [
            _mm512_set1_epi32(0x1a1918),
            _mm512_set1_epi32(0x121110),
            _mm512_set1_epi32(0x0a0908),
            _mm512_set1_epi32(0x020100),
        ],
        data = [_mm512_set1_epi32(0x6d2073), _mm512_set1_epi32(0x696874)],
        expected = [_mm512_set1_epi32(0x735e10), _mm512_set1_epi32(0xb6445d)]
    );

    define_speck_test_x86_64_simd!(
        "avx512_encrypt_block_inflight_64_96",
        avx512_encrypt_block_inflight_64_96,
        "x86_64",
        "avx512f",
        key = [
            _mm512_set1_epi32(0x13121110),
            _mm512_set1_epi32(0x0b0a0908),
            _mm512_set1_epi32(0x03020100),
        ],
        data = [_mm512_set1_epi32(0x74614620), _mm512_set1_epi32(0x736e6165)],
        expected = [
            _mm512_set1_epi32(0x9f7952ecu32 as i32),
            _mm512_set1_epi32(0x4175946c)
        ]
    );

    define_speck_test_x86_64_simd!(
        "avx512_encrypt_block_inflight_64_128",
        avx512_encrypt_block_inflight_64_128,
        "x86_64",
        "avx512f",
        key = [
            _mm512_set1_epi32(0x1b1a1918),
            _mm512_set1_epi32(0x13121110),
            _mm512_set1_epi32(0x0b0a0908),
            _mm512_set1_epi32(0x03020100),
        ],
        data = [_mm512_set1_epi32(0x3b726574), _mm512_set1_epi32(0x7475432d)],
        expected = [
            _mm512_set1_epi32(0x8c6fa548u32 as i32),
            _mm512_set1_epi32(0x454e028b)
        ]
    );

    define_speck_test_x86_64_simd!(
        "avx512_encrypt_block_inflight_96_96",
        avx512_encrypt_block_inflight_96_96,
        "x86_64",
        "avx512f",
        key = [
            _mm512_set1_epi64(0x0d0c0b0a0908),
            _mm512_set1_epi64(0x050403020100),
        ],
        data = [
            _mm512_set1_epi64(0x65776f68202c),
            _mm512_set1_epi64(0x656761737520)
        ],
        expected = [
            _mm512_set1_epi64(0x9e4d09ab7178),
            _mm512_set1_epi64(0x62bdde8f79aa)
        ]
    );

    define_speck_test_x86_64_simd!(
        "avx512_encrypt_block_inflight_96_144",
        avx512_encrypt_block_inflight_96_144,
        "x86_64",
        "avx512f",
        key = [
            _mm512_set1_epi64(0x151413121110),
            _mm512_set1_epi64(0x0d0c0b0a0908),
            _mm512_set1_epi64(0x050403020100),
        ],
        data = [
            _mm512_set1_epi64(0x656d6974206e),
            _mm512_set1_epi64(0x69202c726576)
        ],
        expected = [
            _mm512_set1_epi64(0x2bf31072228a),
            _mm512_set1_epi64(0x7ae440252ee6)
        ]
    );

    define_speck_test_x86_64_simd!(
        "avx512_encrypt_block_inflight_128_128",
        avx512_encrypt_block_inflight_128_128,
        "x86_64",
        "avx512f",
        key = [
            _mm512_set1_epi64(0x0f0e0d0c0b0a0908),
            _mm512_set1_epi64(0x0706050403020100),
        ],
        data = [
            _mm512_set1_epi64(0x6c61766975716520),
            _mm512_set1_epi64(0x7469206564616d20)
        ],
        expected = [
            _mm512_set1_epi64(0xa65d985179783265u64 as i64),
            _mm512_set1_epi64(0x7860fedf5c570d18)
        ]
    );

    define_speck_test_x86_64_simd!(
        "avx512_encrypt_block_inflight_128_192",
        avx512_encrypt_block_inflight_128_192,
        "x86_64",
        "avx512f",
        key = [
            _mm512_set1_epi64(0x1716151413121110),
            _mm512_set1_epi64(0x0f0e0d0c0b0a0908),
            _mm512_set1_epi64(0x0706050403020100),
        ],
        data = [
            _mm512_set1_epi64(0x7261482066656968),
            _mm512_set1_epi64(0x43206f7420746e65)
        ],
        expected = [
            _mm512_set1_epi64(0x1be4cf3a13135566),
            _mm512_set1_epi64(0xf9bc185de03c1886u64 as i64)
        ]
    );

    define_speck_test_x86_64_simd!(
        "avx512_encrypt_block_inflight_128_256",
        avx512_encrypt_block_inflight_128_256,
        "x86_64",
        "avx512f",
        key = [
            _mm512_set1_epi64(0x1f1e1d1c1b1a1918),
            _mm512_set1_epi64(0x1716151413121110),
            _mm512_set1_epi64(0x0f0e0d0c0b0a0908),
            _mm512_set1_epi64(0x0706050403020100),
        ],
        data = [
            _mm512_set1_epi64(0x65736f6874206e49),
            _mm512_set1_epi64(0x202e72656e6f6f70)
        ],
        expected = [
            _mm512_set1_epi64(0x4109010405c0f53e),
            _mm512_set1_epi64(0x4eeeb48d9c188f43)
        ]
    );

    define_speck_test_x86_64_simd!(
        "avx512_decrypt_block_32_64",
        avx512_decrypt_block_32_64,
        "x86_64",
        "avx512bw",
        key = [
            _mm512_set1_epi16(0x1918),
            _mm512_set1_epi16(0x1110),
            _mm512_set1_epi16(0x0908),
            _mm512_set1_epi16(0x0100),
        ],
        data = [
            _mm512_set1_epi16(0xa868u16 as i16),
            _mm512_set1_epi16(0x42f2)
        ],
        expected = [_mm512_set1_epi16(0x6574), _mm512_set1_epi16(0x694c)]
    );

    define_speck_test_x86_64_simd!(
        "avx512_decrypt_block_48_72",
        avx512_decrypt_block_48_72,
        "x86_64",
        "avx512f",
        key = [
            _mm512_set1_epi32(0x121110),
            _mm512_set1_epi32(0x0a0908),
            _mm512_set1_epi32(0x020100),
        ],
        data = [_mm512_set1_epi32(0xc049a5), _mm512_set1_epi32(0x385adc)],
        expected = [_mm512_set1_epi32(0x20796c), _mm512_set1_epi32(0x6c6172)]
    );

    define_speck_test_x86_64_simd!(
        "avx512_decrypt_block_48_96",
        avx512_decrypt_block_48_96,
        "x86_64",
        "avx512f",
        key = [
            _mm512_set1_epi32(0x1a1918),
            _mm512_set1_epi32(0x121110),
            _mm512_set1_epi32(0x0a0908),
            _mm512_set1_epi32(0x020100),
        ],
        data = [_mm512_set1_epi32(0x735e10), _mm512_set1_epi32(0xb6445d)],
        expected = [_mm512_set1_epi32(0x6d2073), _mm512_set1_epi32(0x696874)]
    );

    define_speck_test_x86_64_simd!(
        "avx512_decrypt_block_64_96",
        avx512_decrypt_block_64_96,
        "x86_64",
        "avx512f",
        key = [
            _mm512_set1_epi32(0x13121110),
            _mm512_set1_epi32(0x0b0a0908),
            _mm512_set1_epi32(0x03020100),
        ],
        data = [
            _mm512_set1_epi32(0x9f7952ecu32 as i32),
            _mm512_set1_epi32(0x4175946c)
        ],
        expected = [_mm512_set1_epi32(0x74614620), _mm512_set1_epi32(0x736e6165)]
    );

    define_speck_test_x86_64_simd!(
        "avx512_decrypt_block_64_128",
        avx512_decrypt_block_64_128,
        "x86_64",
        "avx512f",
        key = [
            _mm512_set1_epi32(0x1b1a1918),
            _mm512_set1_epi32(0x13121110),
            _mm512_set1_epi32(0x0b0a0908),
            _mm512_set1_epi32(0x03020100),
        ],
        data = [
            _mm512_set1_epi32(0x8c6fa548u32 as i32),
            _mm512_set1_epi32(0x454e028b)
        ],
        expected = [_mm512_set1_epi32(0x3b726574), _mm512_set1_epi32(0x7475432d)]
    );

    define_speck_test_x86_64_simd!(
        "avx512_decrypt_block_96_96",
        avx512_decrypt_block_96_96,
        "x86_64",
        "avx512f",
        key = [
            _mm512_set1_epi64(0x0d0c0b0a0908),
            _mm512_set1_epi64(0x050403020100),
        ],
        data = [
            _mm512_set1_epi64(0x9e4d09ab7178),
            _mm512_set1_epi64(0x62bdde8f79aa)
        ],
        expected = [
            _mm512_set1_epi64(0x65776f68202c),
            _mm512_set1_epi64(0x656761737520)
        ]
    );

    define_speck_test_x86_64_simd!(
        "avx512_decrypt_block_96_144",
        avx512_decrypt_block_96_144,
        "x86_64",
        "avx512f",
        key = [
            _mm512_set1_epi64(0x151413121110),
            _mm512_set1_epi64(0x0d0c0b0a0908),
            _mm512_set1_epi64(0x050403020100),
        ],
        data = [
            _mm512_set1_epi64(0x2bf31072228a),
            _mm512_set1_epi64(0x7ae440252ee6)
        ],
        expected = [
            _mm512_set1_epi64(0x656d6974206e),
            _mm512_set1_epi64(0x69202c726576)
        ]
    );

    define_speck_test_x86_64_simd!(
        "avx512_decrypt_block_128_128",
        avx512_decrypt_block_128_128,
        "x86_64",
        "avx512f",
        key = [
            _mm512_set1_epi64(0x0f0e0d0c0b0a0908),
            _mm512_set1_epi64(0x0706050403020100),
        ],
        data = [
            _mm512_set1_epi64(0xa65d985179783265u64 as i64),
            _mm512_set1_epi64(0x7860fedf5c570d18)
        ],
        expected = [
            _mm512_set1_epi64(0x6c61766975716520),
            _mm512_set1_epi64(0x7469206564616d20)
        ]
    );

    define_speck_test_x86_64_simd!(
        "avx512_decrypt_block_128_192",
        avx512_decrypt_block_128_192,
        "x86_64",
        "avx512f",
        key = [
            _mm512_set1_epi64(0x1716151413121110),
            _mm512_set1_epi64(0x0f0e0d0c0b0a0908),
            _mm512_set1_epi64(0x0706050403020100),
        ],
        data = [
            _mm512_set1_epi64(0x1be4cf3a13135566),
            _mm512_set1_epi64(0xf9bc185de03c1886u64 as i64)
        ],
        expected = [
            _mm512_set1_epi64(0x7261482066656968),
            _mm512_set1_epi64(0x43206f7420746e65)
        ]
    );

    define_speck_test_x86_64_simd!(
        "avx512_decrypt_block_128_256",
        avx512_decrypt_block_128_256,
        "x86_64",
        "avx512f",
        key = [
            _mm512_set1_epi64(0x1f1e1d1c1b1a1918),
            _mm512_set1_epi64(0x1716151413121110),
            _mm512_set1_epi64(0x0f0e0d0c0b0a0908),
            _mm512_set1_epi64(0x0706050403020100),
        ],
        data = [
            _mm512_set1_epi64(0x4109010405c0f53e),
            _mm512_set1_epi64(0x4eeeb48d9c188f43)
        ],
        expected = [
            _mm512_set1_epi64(0x65736f6874206e49),
            _mm512_set1_epi64(0x202e72656e6f6f70)
        ]
    );
}
