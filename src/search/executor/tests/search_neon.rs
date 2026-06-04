use super::common::search_tests;

search_tests! {
    #[cfg(all(test, target_arch = "aarch64", target_feature = "neon"))]
    mod neon_search_32_64,
    encrypt:   crate::search::executor::backend::neon_search_encrypt_32_64,
    inflight:  crate::search::executor::backend::neon_search_encrypt_inflight_32_64,
    decrypt:   crate::search::executor::backend::neon_search_decrypt_32_64,
    v:         0x0100_0908_1110_1918_u64,
    prefix:    [],
    key_bytes: &[0x18u8, 0x19, 0x10, 0x11, 0x08, 0x09, 0x00, 0x01],
    pt:        [0x6574_u16, 0x694c],
    ct:        [0xa868_u16, 0x42f2],
    converter: crate::search::executor::backend::aarch64::neon::neon_u16x2_block_to_vec,
    word:      u16,
}

search_tests! {
    #[cfg(all(test, target_arch = "aarch64", target_feature = "neon"))]
    mod neon_search_48_72,
    encrypt:   crate::search::executor::backend::neon_search_encrypt_48_72,
    inflight:  crate::search::executor::backend::neon_search_encrypt_inflight_48_72,
    decrypt:   crate::search::executor::backend::neon_search_decrypt_48_72,
    v:         0x0100_0a09_0812_1110_u64,
    prefix:    [0x02_u8],
    key_bytes: &[0x10u8, 0x11, 0x12, 0x08, 0x09, 0x0a, 0x00, 0x01, 0x02],
    pt:        [0x20796c_u32, 0x6c6172],
    ct:        [0xc049a5_u32, 0x385adc],
    converter: crate::search::executor::backend::aarch64::neon::neon_u32x2_block_to_vec,
    word:      u32,
}

search_tests! {
    #[cfg(all(test, target_arch = "aarch64", target_feature = "neon"))]
    mod neon_search_48_96,
    encrypt:   crate::search::executor::backend::neon_search_encrypt_48_96,
    inflight:  crate::search::executor::backend::neon_search_encrypt_inflight_48_96,
    decrypt:   crate::search::executor::backend::neon_search_decrypt_48_96,
    v:         0x0908_1211_101a_1918_u64,
    prefix:    [0x0a_u8, 0x00, 0x01, 0x02],
    key_bytes: &[0x18u8, 0x19, 0x1a, 0x10, 0x11, 0x12, 0x08, 0x09, 0x0a, 0x00, 0x01, 0x02],
    pt:        [0x6d2073_u32, 0x696874],
    ct:        [0x735e10_u32, 0xb6445d],
    converter: crate::search::executor::backend::aarch64::neon::neon_u32x2_block_to_vec,
    word:      u32,
}

search_tests! {
    #[cfg(all(test, target_arch = "aarch64", target_feature = "neon"))]
    mod neon_search_64_96,
    encrypt:   crate::search::executor::backend::neon_search_encrypt_64_96,
    inflight:  crate::search::executor::backend::neon_search_encrypt_inflight_64_96,
    decrypt:   crate::search::executor::backend::neon_search_decrypt_64_96,
    v:         0x0b0a_0908_1312_1110_u64,
    prefix:    [0x00_u8, 0x01, 0x02, 0x03],
    key_bytes: &[0x10u8, 0x11, 0x12, 0x13, 0x08, 0x09, 0x0a, 0x0b, 0x00, 0x01, 0x02, 0x03],
    pt:        [0x74614620_u32, 0x736e6165],
    ct:        [0x9f7952ec_u32, 0x4175946c],
    converter: crate::search::executor::backend::aarch64::neon::neon_u32x2_block_to_vec,
    word:      u32,
}

search_tests! {
    #[cfg(all(test, target_arch = "aarch64", target_feature = "neon"))]
    mod neon_search_64_128,
    encrypt:   crate::search::executor::backend::neon_search_encrypt_64_128,
    inflight:  crate::search::executor::backend::neon_search_encrypt_inflight_64_128,
    decrypt:   crate::search::executor::backend::neon_search_decrypt_64_128,
    v:         0x1312_1110_1b1a_1918_u64,
    prefix:    [0x08_u8, 0x09, 0x0a, 0x0b, 0x00, 0x01, 0x02, 0x03],
    key_bytes: &[0x18u8, 0x19, 0x1a, 0x1b, 0x10, 0x11, 0x12, 0x13, 0x08, 0x09, 0x0a, 0x0b, 0x00, 0x01, 0x02, 0x03],
    pt:        [0x3b726574_u32, 0x7475432d],
    ct:        [0x8c6fa548_u32, 0x454e028b],
    converter: crate::search::executor::backend::aarch64::neon::neon_u32x2_block_to_vec,
    word:      u32,
}

search_tests! {
    #[cfg(all(test, target_arch = "aarch64", target_feature = "neon"))]
    mod neon_search_96_96,
    encrypt:   crate::search::executor::backend::neon_search_encrypt_96_96,
    inflight:  crate::search::executor::backend::neon_search_encrypt_inflight_96_96,
    decrypt:   crate::search::executor::backend::neon_search_decrypt_96_96,
    v:         0x0100_0d0c_0b0a_0908_u64,
    prefix:    [0x02_u8, 0x03, 0x04, 0x05],
    key_bytes: &[0x08u8, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05],
    pt:        [0x65776f68202c_u64, 0x656761737520],
    ct:        [0x9e4d09ab7178_u64, 0x62bdde8f79aa],
    converter: crate::search::executor::backend::aarch64::neon::neon_u64x2_block_to_vec,
    word:      u64,
}

search_tests! {
    #[cfg(all(test, target_arch = "aarch64", target_feature = "neon"))]
    mod neon_search_96_144,
    encrypt:   crate::search::executor::backend::neon_search_encrypt_96_144,
    inflight:  crate::search::executor::backend::neon_search_encrypt_inflight_96_144,
    decrypt:   crate::search::executor::backend::neon_search_decrypt_96_144,
    v:         0x0908_1514_1312_1110_u64,
    prefix:    [0x0a_u8, 0x0b, 0x0c, 0x0d, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05],
    key_bytes: &[0x10u8, 0x11, 0x12, 0x13, 0x14, 0x15, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05],
    pt:        [0x656d6974206e_u64, 0x69202c726576],
    ct:        [0x2bf31072228a_u64, 0x7ae440252ee6],
    converter: crate::search::executor::backend::aarch64::neon::neon_u64x2_block_to_vec,
    word:      u64,
}

search_tests! {
    #[cfg(all(test, target_arch = "aarch64", target_feature = "neon"))]
    mod neon_search_128_128,
    encrypt:   crate::search::executor::backend::neon_search_encrypt_128_128,
    inflight:  crate::search::executor::backend::neon_search_encrypt_inflight_128_128,
    decrypt:   crate::search::executor::backend::neon_search_decrypt_128_128,
    v:         0x0f0e_0d0c_0b0a_0908_u64,
    prefix:    [0x00_u8, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07],
    key_bytes: &[0x08u8, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07],
    pt:        [0x6c61766975716520_u64, 0x7469206564616d20],
    ct:        [0xa65d985179783265_u64, 0x7860fedf5c570d18],
    converter: crate::search::executor::backend::aarch64::neon::neon_u64x2_block_to_vec,
    word:      u64,
}

search_tests! {
    #[cfg(all(test, target_arch = "aarch64", target_feature = "neon"))]
    mod neon_search_128_192,
    encrypt:   crate::search::executor::backend::neon_search_encrypt_128_192,
    inflight:  crate::search::executor::backend::neon_search_encrypt_inflight_128_192,
    decrypt:   crate::search::executor::backend::neon_search_decrypt_128_192,
    v:         0x1716_1514_1312_1110_u64,
    prefix:    [0x08_u8, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07],
    key_bytes: &[0x10u8, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07],
    pt:        [0x7261482066656968_u64, 0x43206f7420746e65],
    ct:        [0x1be4cf3a13135566_u64, 0xf9bc185de03c1886],
    converter: crate::search::executor::backend::aarch64::neon::neon_u64x2_block_to_vec,
    word:      u64,
}

search_tests! {
    #[cfg(all(test, target_arch = "aarch64", target_feature = "neon"))]
    mod neon_search_128_256,
    encrypt:   crate::search::executor::backend::neon_search_encrypt_128_256,
    inflight:  crate::search::executor::backend::neon_search_encrypt_inflight_128_256,
    decrypt:   crate::search::executor::backend::neon_search_decrypt_128_256,
    v:         0x1f1e_1d1c_1b1a_1918_u64,
    prefix:    [0x10_u8, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07],
    key_bytes: &[0x18u8, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07],
    pt:        [0x65736f6874206e49_u64, 0x202e72656e6f6f70],
    ct:        [0x4109010405c0f53e_u64, 0x4eeeb48d9c188f43],
    converter: crate::search::executor::backend::aarch64::neon::neon_u64x2_block_to_vec,
    word:      u64,
}
