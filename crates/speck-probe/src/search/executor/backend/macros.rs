macro_rules! define_backend_dispatch {
    (
        $(attrs = [$($attr:meta),* $(,)?],)?
        simd = $simd_prefix:ident,
        versions = [
            $( ($version:tt, bytes=$bytes:literal, ew=$ew:ty, vw=$vw:ty, converter=$conv:expr) ),+
            $(,)?
        ]
        $(,)?
    ) => {
        seq_macro::seq!(S in 1..=4 {
            paste::paste! {
                $($(#[$attr])*)?
                pub(crate) fn dispatch(
                    request: $crate::search::executor::RuntimeRequest,
                    version: speck::SpeckVersion,
                    suffix: usize,
                    function: $crate::search::executor::CipherFunction,
                ) -> $crate::search::executor::DispatchOutput {
                    use $crate::search::executor::backend::runner::run_orchestrator;
                    match (version, suffix, function) {
                        $(
                            #(
                                (speck::SpeckVersion::[<Speck $version>], S, $crate::search::executor::CipherFunction::Encrypt) =>
                                    run_orchestrator::<$ew, $vw, $bytes, {$bytes - S}, _, _>(
                                        request,
                                        |task, out| engine::[<$simd_prefix search_encrypt_ $version>](task, out),
                                        engine::[<ecb_validate_encrypt_ $version>],
                                        |__block| ($conv)(__block),
                                    ),
                                (speck::SpeckVersion::[<Speck $version>], S, $crate::search::executor::CipherFunction::Decrypt) =>
                                    run_orchestrator::<$ew, $vw, $bytes, {$bytes - S}, _, _>(
                                        request,
                                        |task, out| engine::[<$simd_prefix search_decrypt_ $version>](task, out),
                                        engine::[<ecb_validate_encrypt_ $version>],
                                        |__block| ($conv)(__block),
                                    ),
                                (speck::SpeckVersion::[<Speck $version>], S, $crate::search::executor::CipherFunction::EncryptInflight) =>
                                    run_orchestrator::<$ew, $vw, $bytes, {$bytes - S}, _, _>(
                                        request,
                                        |task, out| engine::[<$simd_prefix search_encrypt_inflight_ $version>](task, out),
                                        engine::[<ecb_validate_encrypt_ $version>],
                                        |__block| ($conv)(__block),
                                    ),
                            )*
                        )+
                        _ => unreachable!("validate suffix/version/function before dispatch"),
                    }
                }
            }
        });
    };
}

pub(crate) use define_backend_dispatch;
