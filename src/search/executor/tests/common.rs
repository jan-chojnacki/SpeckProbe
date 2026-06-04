macro_rules! search_tests {
    (
        $(#[$meta:meta])*
        mod $mod_name:ident,
        encrypt:   $enc:path,
        inflight:  $inf:path,
        decrypt:   $dec:path,
        v:         $v:expr,
        prefix:    $prefix:expr,
        key_bytes: $kb:expr,
        pt:        $pt:expr,
        ct:        $ct:expr,
        converter: $conv:path,
        word:      $word:ty,
    ) => {
        $(#[$meta])*
        mod $mod_name {
            const RANGE: u64 = 1000;

            #[allow(unused_unsafe)]
            #[rstest::rstest]
            #[case($pt, $ct, $v, $v)]
            #[case($pt, $ct, $v - RANGE, $v + RANGE)]
            #[case($pt, $ct, $v, $v + RANGE)]
            #[case($pt, $ct, $v - RANGE, $v)]
            fn encrypt_test(
                #[case] data: [$word; 2],
                #[case] expected: [$word; 2],
                #[case] start: u64,
                #[case] end: u64,
            ) {
                unsafe {
                    let mut out = Vec::new();
                    let data = $conv(data);
                    let expected = $conv(expected);
                    $enc(
                        crate::search::domain::task::Task {
                            prefix: $prefix,
                            start,
                            end,
                            data,
                            expected,
                        },
                        &mut out,
                    );
                    assert!(out.iter().any(|k| k.as_bytes() == ($kb as &[u8])));
                }
            }

            #[allow(unused_unsafe)]
            #[rstest::rstest]
            #[case($pt, $ct, $v, $v)]
            #[case($pt, $ct, $v - RANGE, $v + RANGE)]
            #[case($pt, $ct, $v, $v + RANGE)]
            #[case($pt, $ct, $v - RANGE, $v)]
            fn encrypt_inflight_test(
                #[case] data: [$word; 2],
                #[case] expected: [$word; 2],
                #[case] start: u64,
                #[case] end: u64,
            ) {
                unsafe {
                    let mut out = Vec::new();
                    let data = $conv(data);
                    let expected = $conv(expected);
                    $inf(
                        crate::search::domain::task::Task {
                            prefix: $prefix,
                            start,
                            end,
                            data,
                            expected,
                        },
                        &mut out,
                    );
                    assert!(out.iter().any(|k| k.as_bytes() == ($kb as &[u8])));
                }
            }

            #[allow(unused_unsafe)]
            #[rstest::rstest]
            #[case($ct, $pt, $v, $v)]
            #[case($ct, $pt, $v - RANGE, $v + RANGE)]
            #[case($ct, $pt, $v, $v + RANGE)]
            #[case($ct, $pt, $v - RANGE, $v)]
            fn decrypt_test(
                #[case] data: [$word; 2],
                #[case] expected: [$word; 2],
                #[case] start: u64,
                #[case] end: u64,
            ) {
                unsafe {
                    let mut out = Vec::new();
                    let data = $conv(data);
                    let expected = $conv(expected);
                    $dec(
                        crate::search::domain::task::Task {
                            prefix: $prefix,
                            start,
                            end,
                            data,
                            expected,
                        },
                        &mut out,
                    );
                    assert!(out.iter().any(|k| k.as_bytes() == ($kb as &[u8])));
                }
            }
        }
    };
}

pub(super) use search_tests;
