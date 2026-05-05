macro_rules! dispatch_tests {
    (
        mod $mod_name:ident,
        version: $version:expr,
        key_prefix: $key_prefix:expr,
        key_bytes: $kb:expr,
        pt: $pt:expr,
        ct: $ct:expr,
    ) => {
        #[cfg(all(test, target_arch = "x86_64", target_feature = "avx512bw"))]
        mod $mod_name {
            use crate::search::executor::dispatch::dispatch;
            use crate::search::executor::{
                BackendHint, CipherConfig, CipherFunction, CipherMode, DispatchOutput,
                InternalConfig, RuntimeConfig, RuntimeRequest, SearchSpace,
            };

            fn make_request(
                mode: CipherMode,
                func: CipherFunction,
                threads: usize,
                data: [u64; 2],
                expected: [u64; 2],
            ) -> RuntimeRequest {
                RuntimeRequest {
                    cipher_config: CipherConfig {
                        cipher_mode: mode,
                        speck_version: $version,
                        cipher_function: func,
                    },
                    runtime_config: RuntimeConfig {
                        suffix_bytes_size: 1,
                        num_threads: threads,
                        backend_hint: BackendHint::Avx512,
                    },
                    search_space: SearchSpace {
                        start: ($key_prefix as &[u8]).to_vec(),
                        end: ($key_prefix as &[u8]).to_vec(),
                        data: vec![data],
                        expected: vec![expected],
                        iv: if mode == CipherMode::Cbc {
                            Some([0u64, 0u64])
                        } else {
                            None
                        },
                    },
                    internal_config: InternalConfig { cli_tx: None },
                }
            }

            fn key_found(output: DispatchOutput) -> bool {
                let (candidates, found) = output;
                found.as_deref() == Some($kb as &[u8])
                    || candidates.iter().any(|k| k.as_slice() == ($kb as &[u8]))
            }

            #[rstest::rstest]
            #[case::t1(1)]
            #[case::t2(2)]
            #[case::t4(4)]
            #[case::oversubscribed(num_cpus::get() * 2 + 1)]
            fn encrypt_ecb(#[case] threads: usize) {
                let req = make_request(CipherMode::Ecb, CipherFunction::Encrypt, threads, $pt, $ct);
                assert!(key_found(dispatch(req).unwrap()));
            }

            #[rstest::rstest]
            #[case::t1(1)]
            #[case::t2(2)]
            #[case::t4(4)]
            #[case::oversubscribed(num_cpus::get() * 2 + 1)]
            fn encrypt_inflight_ecb(#[case] threads: usize) {
                let req = make_request(
                    CipherMode::Ecb,
                    CipherFunction::EncryptInflight,
                    threads,
                    $pt,
                    $ct,
                );
                assert!(key_found(dispatch(req).unwrap()));
            }

            #[rstest::rstest]
            #[case::t1(1)]
            #[case::t2(2)]
            #[case::t4(4)]
            #[case::oversubscribed(num_cpus::get() * 2 + 1)]
            fn decrypt_ecb(#[case] threads: usize) {
                let req = make_request(CipherMode::Ecb, CipherFunction::Decrypt, threads, $ct, $pt);
                assert!(key_found(dispatch(req).unwrap()));
            }

            #[rstest::rstest]
            #[case::t1(1)]
            #[case::t2(2)]
            #[case::t4(4)]
            #[case::oversubscribed(num_cpus::get() * 2 + 1)]
            fn encrypt_cbc(#[case] threads: usize) {
                let req = make_request(CipherMode::Cbc, CipherFunction::Encrypt, threads, $pt, $ct);
                assert!(key_found(dispatch(req).unwrap()));
            }

            #[rstest::rstest]
            #[case::t1(1)]
            #[case::t2(2)]
            #[case::t4(4)]
            #[case::oversubscribed(num_cpus::get() * 2 + 1)]
            fn encrypt_inflight_cbc(#[case] threads: usize) {
                let req = make_request(
                    CipherMode::Cbc,
                    CipherFunction::EncryptInflight,
                    threads,
                    $pt,
                    $ct,
                );
                assert!(key_found(dispatch(req).unwrap()));
            }

            #[rstest::rstest]
            #[case::t1(1)]
            #[case::t2(2)]
            #[case::t4(4)]
            #[case::oversubscribed(num_cpus::get() * 2 + 1)]
            fn decrypt_cbc(#[case] threads: usize) {
                let req = make_request(CipherMode::Cbc, CipherFunction::Decrypt, threads, $ct, $pt);
                assert!(key_found(dispatch(req).unwrap()));
            }
        }
    };
}

dispatch_tests! {
    mod avx512_dispatch_32_64,
    version:    crate::speck::SpeckVersion::Speck32_64,
    key_prefix: &[0x19u8, 0x10, 0x11, 0x08, 0x09, 0x00, 0x01],
    key_bytes:  &[0x18u8, 0x19, 0x10, 0x11, 0x08, 0x09, 0x00, 0x01],
    pt:         [0x6574_u64, 0x694c_u64],
    ct:         [0xa868_u64, 0x42f2_u64],
}

dispatch_tests! {
    mod avx512_dispatch_48_72,
    version:    crate::speck::SpeckVersion::Speck48_72,
    key_prefix: &[0x11u8, 0x12, 0x08, 0x09, 0x0a, 0x00, 0x01, 0x02],
    key_bytes:  &[0x10u8, 0x11, 0x12, 0x08, 0x09, 0x0a, 0x00, 0x01, 0x02],
    pt:         [0x20796c_u64, 0x6c6172_u64],
    ct:         [0xc049a5_u64, 0x385adc_u64],
}

dispatch_tests! {
    mod avx512_dispatch_48_96,
    version:    crate::speck::SpeckVersion::Speck48_96,
    key_prefix: &[0x19u8, 0x1a, 0x10, 0x11, 0x12, 0x08, 0x09, 0x0a, 0x00, 0x01, 0x02],
    key_bytes:  &[0x18u8, 0x19, 0x1a, 0x10, 0x11, 0x12, 0x08, 0x09, 0x0a, 0x00, 0x01, 0x02],
    pt:         [0x6d2073_u64, 0x696874_u64],
    ct:         [0x735e10_u64, 0xb6445d_u64],
}

dispatch_tests! {
    mod avx512_dispatch_64_96,
    version:    crate::speck::SpeckVersion::Speck64_96,
    key_prefix: &[0x11u8, 0x12, 0x13, 0x08, 0x09, 0x0a, 0x0b, 0x00, 0x01, 0x02, 0x03],
    key_bytes:  &[0x10u8, 0x11, 0x12, 0x13, 0x08, 0x09, 0x0a, 0x0b, 0x00, 0x01, 0x02, 0x03],
    pt:         [0x74614620_u64, 0x736e6165_u64],
    ct:         [0x9f7952ec_u64, 0x4175946c_u64],
}

dispatch_tests! {
    mod avx512_dispatch_64_128,
    version:    crate::speck::SpeckVersion::Speck64_128,
    key_prefix: &[0x19u8, 0x1a, 0x1b, 0x10, 0x11, 0x12, 0x13, 0x08, 0x09, 0x0a, 0x0b, 0x00, 0x01, 0x02, 0x03],
    key_bytes:  &[0x18u8, 0x19, 0x1a, 0x1b, 0x10, 0x11, 0x12, 0x13, 0x08, 0x09, 0x0a, 0x0b, 0x00, 0x01, 0x02, 0x03],
    pt:         [0x3b726574_u64, 0x7475432d_u64],
    ct:         [0x8c6fa548_u64, 0x454e028b_u64],
}

dispatch_tests! {
    mod avx512_dispatch_96_96,
    version:    crate::speck::SpeckVersion::Speck96_96,
    key_prefix: &[0x09u8, 0x0a, 0x0b, 0x0c, 0x0d, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05],
    key_bytes:  &[0x08u8, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05],
    pt:         [0x65776f68202c_u64, 0x656761737520_u64],
    ct:         [0x9e4d09ab7178_u64, 0x62bdde8f79aa_u64],
}

dispatch_tests! {
    mod avx512_dispatch_96_144,
    version:    crate::speck::SpeckVersion::Speck96_144,
    key_prefix: &[0x11u8, 0x12, 0x13, 0x14, 0x15, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05],
    key_bytes:  &[0x10u8, 0x11, 0x12, 0x13, 0x14, 0x15, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05],
    pt:         [0x656d6974206e_u64, 0x69202c726576_u64],
    ct:         [0x2bf31072228a_u64, 0x7ae440252ee6_u64],
}

dispatch_tests! {
    mod avx512_dispatch_128_128,
    version:    crate::speck::SpeckVersion::Speck128_128,
    key_prefix: &[0x09u8, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07],
    key_bytes:  &[0x08u8, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07],
    pt:         [0x6c61766975716520_u64, 0x7469206564616d20_u64],
    ct:         [0xa65d985179783265_u64, 0x7860fedf5c570d18_u64],
}

dispatch_tests! {
    mod avx512_dispatch_128_192,
    version:    crate::speck::SpeckVersion::Speck128_192,
    key_prefix: &[0x11u8, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07],
    key_bytes:  &[0x10u8, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07],
    pt:         [0x7261482066656968_u64, 0x43206f7420746e65_u64],
    ct:         [0x1be4cf3a13135566_u64, 0xf9bc185de03c1886_u64],
}

dispatch_tests! {
    mod avx512_dispatch_128_256,
    version:    crate::speck::SpeckVersion::Speck128_256,
    key_prefix: &[0x19u8, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07],
    key_bytes:  &[0x18u8, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07],
    pt:         [0x65736f6874206e49_u64, 0x202e72656e6f6f70_u64],
    ct:         [0x4109010405c0f53e_u64, 0x4eeeb48d9c188f43_u64],
}
