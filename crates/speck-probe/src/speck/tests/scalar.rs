#[cfg(test)]
mod test {
    use super::super::common::define_speck_test;

    define_speck_test!(
        "encrypt_block_32_64",
        crate::speck::scalar_encrypt_block_32_64,
        key = [0x1918, 0x1110, 0x0908, 0x0100,],
        data = [0x6574, 0x694c],
        expected = [0xa868u16, 0x42f2u16]
    );

    define_speck_test!(
        "encrypt_block_48_72",
        crate::speck::scalar_encrypt_block_48_72,
        key = [0x121110u32, 0x0a0908u32, 0x020100u32,],
        data = [0x20796cu32, 0x6c6172u32],
        expected = [0xc049a5, 0x385adc]
    );

    define_speck_test!(
        "encrypt_block_48_96",
        crate::speck::scalar_encrypt_block_48_96,
        key = [0x1a1918u32, 0x121110u32, 0x0a0908u32, 0x020100u32,],
        data = [0x6d2073u32, 0x696874u32],
        expected = [0x735e10, 0xb6445d]
    );

    define_speck_test!(
        "encrypt_block_64_96",
        crate::speck::scalar_encrypt_block_64_96,
        key = [0x13121110, 0x0b0a0908, 0x03020100,],
        data = [0x74614620, 0x736e6165],
        expected = [0x9f7952ecu32, 0x4175946cu32]
    );

    define_speck_test!(
        "encrypt_block_64_128",
        crate::speck::scalar_encrypt_block_64_128,
        key = [0x1b1a1918, 0x13121110, 0x0b0a0908, 0x03020100,],
        data = [0x3b726574, 0x7475432d],
        expected = [0x8c6fa548u32, 0x454e028bu32]
    );

    define_speck_test!(
        "encrypt_block_96_96",
        crate::speck::scalar_encrypt_block_96_96,
        key = [0x0d0c0b0a0908u64, 0x050403020100u64,],
        data = [0x65776f68202cu64, 0x656761737520u64],
        expected = [0x9e4d09ab7178u64, 0x62bdde8f79aau64]
    );

    define_speck_test!(
        "encrypt_block_96_144",
        crate::speck::scalar_encrypt_block_96_144,
        key = [0x151413121110u64, 0x0d0c0b0a0908u64, 0x050403020100u64,],
        data = [0x656d6974206eu64, 0x69202c726576u64],
        expected = [0x2bf31072228au64, 0x7ae440252ee6u64]
    );

    define_speck_test!(
        "encrypt_block_128_128",
        crate::speck::scalar_encrypt_block_128_128,
        key = [0x0f0e0d0c0b0a0908, 0x0706050403020100,],
        data = [0x6c61766975716520, 0x7469206564616d20],
        expected = [0xa65d985179783265u64 as i64, 0x7860fedf5c570d18]
    );

    define_speck_test!(
        "encrypt_block_128_192",
        crate::speck::scalar_encrypt_block_128_192,
        key = [0x1716151413121110, 0x0f0e0d0c0b0a0908, 0x0706050403020100,],
        data = [0x7261482066656968, 0x43206f7420746e65],
        expected = [0x1be4cf3a13135566, 0xf9bc185de03c1886u64 as i64]
    );

    define_speck_test!(
        "encrypt_block_128_256",
        crate::speck::scalar_encrypt_block_128_256,
        key = [
            0x1f1e1d1c1b1a1918,
            0x1716151413121110,
            0x0f0e0d0c0b0a0908,
            0x0706050403020100,
        ],
        data = [0x65736f6874206e49, 0x202e72656e6f6f70],
        expected = [0x4109010405c0f53eu64, 0x4eeeb48d9c188f43u64]
    );

    define_speck_test!(
        "encrypt_block_inflight_32_64",
        crate::speck::scalar_encrypt_block_inflight_32_64,
        key = [0x1918, 0x1110, 0x0908, 0x0100,],
        data = [0x6574, 0x694c],
        expected = [0xa868u16, 0x42f2u16]
    );

    define_speck_test!(
        "encrypt_block_inflight_48_72",
        crate::speck::scalar_encrypt_block_inflight_48_72,
        key = [0x121110u32, 0x0a0908u32, 0x020100u32,],
        data = [0x20796cu32, 0x6c6172u32],
        expected = [0xc049a5, 0x385adc]
    );

    define_speck_test!(
        "encrypt_block_inflight_48_96",
        crate::speck::scalar_encrypt_block_inflight_48_96,
        key = [0x1a1918u32, 0x121110u32, 0x0a0908u32, 0x020100u32,],
        data = [0x6d2073u32, 0x696874u32],
        expected = [0x735e10, 0xb6445d]
    );

    define_speck_test!(
        "encrypt_block_inflight_64_96",
        crate::speck::scalar_encrypt_block_inflight_64_96,
        key = [0x13121110, 0x0b0a0908, 0x03020100,],
        data = [0x74614620, 0x736e6165],
        expected = [0x9f7952ecu32, 0x4175946cu32]
    );

    define_speck_test!(
        "encrypt_block_inflight_64_128",
        crate::speck::scalar_encrypt_block_inflight_64_128,
        key = [0x1b1a1918, 0x13121110, 0x0b0a0908, 0x03020100,],
        data = [0x3b726574, 0x7475432d],
        expected = [0x8c6fa548u32, 0x454e028bu32]
    );

    define_speck_test!(
        "encrypt_block_inflight_96_96",
        crate::speck::scalar_encrypt_block_inflight_96_96,
        key = [0x0d0c0b0a0908u64, 0x050403020100u64,],
        data = [0x65776f68202cu64, 0x656761737520u64],
        expected = [0x9e4d09ab7178u64, 0x62bdde8f79aau64]
    );

    define_speck_test!(
        "encrypt_block_inflight_96_144",
        crate::speck::scalar_encrypt_block_inflight_96_144,
        key = [0x151413121110u64, 0x0d0c0b0a0908u64, 0x050403020100u64,],
        data = [0x656d6974206eu64, 0x69202c726576u64],
        expected = [0x2bf31072228au64, 0x7ae440252ee6u64]
    );

    define_speck_test!(
        "encrypt_block_inflight_128_128",
        crate::speck::scalar_encrypt_block_inflight_128_128,
        key = [0x0f0e0d0c0b0a0908, 0x0706050403020100,],
        data = [0x6c61766975716520, 0x7469206564616d20],
        expected = [0xa65d985179783265u64 as i64, 0x7860fedf5c570d18]
    );

    define_speck_test!(
        "encrypt_block_inflight_128_192",
        crate::speck::scalar_encrypt_block_inflight_128_192,
        key = [0x1716151413121110, 0x0f0e0d0c0b0a0908, 0x0706050403020100,],
        data = [0x7261482066656968, 0x43206f7420746e65],
        expected = [0x1be4cf3a13135566, 0xf9bc185de03c1886u64 as i64]
    );

    define_speck_test!(
        "encrypt_block_inflight_128_256",
        crate::speck::scalar_encrypt_block_inflight_128_256,
        key = [
            0x1f1e1d1c1b1a1918,
            0x1716151413121110,
            0x0f0e0d0c0b0a0908,
            0x0706050403020100,
        ],
        data = [0x65736f6874206e49, 0x202e72656e6f6f70],
        expected = [0x4109010405c0f53eu64, 0x4eeeb48d9c188f43u64]
    );

    define_speck_test!(
        "decrypt_block_32_64",
        crate::speck::scalar_decrypt_block_32_64,
        key = [0x1918, 0x1110, 0x0908, 0x0100,],
        data = [0xa868, 0x42f2],
        expected = [0x6574u16, 0x694cu16]
    );

    define_speck_test!(
        "decrypt_block_48_72",
        crate::speck::scalar_decrypt_block_48_72,
        key = [0x121110u32, 0x0a0908u32, 0x020100u32,],
        data = [0xc049a5u32, 0x385adcu32],
        expected = [0x20796c, 0x6c6172]
    );

    define_speck_test!(
        "decrypt_block_48_96",
        crate::speck::scalar_decrypt_block_48_96,
        key = [0x1a1918u32, 0x121110u32, 0x0a0908u32, 0x020100u32,],
        data = [0x735e10u32, 0xb6445du32],
        expected = [0x6d2073, 0x696874]
    );

    define_speck_test!(
        "decrypt_block_64_96",
        crate::speck::scalar_decrypt_block_64_96,
        key = [0x13121110, 0x0b0a0908, 0x03020100,],
        data = [0x9f7952ec, 0x4175946c],
        expected = [0x74614620, 0x736e6165]
    );

    define_speck_test!(
        "decrypt_block_64_128",
        crate::speck::scalar_decrypt_block_64_128,
        key = [0x1b1a1918, 0x13121110, 0x0b0a0908, 0x03020100,],
        data = [0x8c6fa548, 0x454e028b],
        expected = [0x3b726574, 0x7475432d]
    );

    define_speck_test!(
        "decrypt_block_96_96",
        crate::speck::scalar_decrypt_block_96_96,
        key = [0x0d0c0b0a0908u64, 0x050403020100u64,],
        data = [0x9e4d09ab7178u64, 0x62bdde8f79aau64],
        expected = [0x65776f68202cu64, 0x656761737520u64]
    );

    define_speck_test!(
        "decrypt_block_96_144",
        crate::speck::scalar_decrypt_block_96_144,
        key = [0x151413121110u64, 0x0d0c0b0a0908u64, 0x050403020100u64,],
        data = [0x2bf31072228au64, 0x7ae440252ee6u64],
        expected = [0x656d6974206eu64, 0x69202c726576u64]
    );

    define_speck_test!(
        "decrypt_block_128_128",
        crate::speck::scalar_decrypt_block_128_128,
        key = [0x0f0e0d0c0b0a0908, 0x0706050403020100,],
        data = [0xa65d985179783265, 0x7860fedf5c570d18],
        expected = [0x6c61766975716520u64, 0x7469206564616d20u64]
    );

    define_speck_test!(
        "decrypt_block_128_192",
        crate::speck::scalar_decrypt_block_128_192,
        key = [0x1716151413121110, 0x0f0e0d0c0b0a0908, 0x0706050403020100,],
        data = [0x1be4cf3a13135566, 0xf9bc185de03c1886],
        expected = [0x7261482066656968u64, 0x43206f7420746e65u64]
    );

    define_speck_test!(
        "decrypt_block_128_256",
        crate::speck::scalar_decrypt_block_128_256,
        key = [
            0x1f1e1d1c1b1a1918,
            0x1716151413121110,
            0x0f0e0d0c0b0a0908,
            0x0706050403020100,
        ],
        data = [0x4109010405c0f53e, 0x4eeeb48d9c188f43],
        expected = [0x65736f6874206e49u64, 0x202e72656e6f6f70u64]
    );
}
