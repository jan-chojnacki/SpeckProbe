macro_rules! define_runtime {
    (
        $(#[$meta:meta])*
        $fn_name:ident,
        $bytes:literal,
        $suffix:literal,
        $engine_word:ty,
        $validator_word:ty,
        $converter:expr,
        $version:tt,
        $mode:tt,
        $cipher_fn:tt
        $(, $simd:tt)?
        $(,)?
    ) => { paste::paste! {
        $(#[$meta])*
        $(
            #[doc = "# Safety"]
            #[doc = "Caller must ensure CPU support for `" $simd "` before calling this function."]
        )?
        pub fn $fn_name(
            runtime_request: $crate::api::RuntimeRequest,
        ) -> (Vec<Vec<u8>>, Option<Vec<u8>>) {
            let start: [u8; { $bytes - $suffix }] = runtime_request.search_space.start
                .try_into()
                .expect("start length mismatch");
            let end: [u8; { $bytes - $suffix }] = runtime_request.search_space.end
                .try_into()
                .expect("end length mismatch");

            let data: Vec<[$validator_word; 2]> = runtime_request.search_space.data
                .iter()
                .map(|[a, b]| [*a as $validator_word, *b as $validator_word])
                .collect();
            let expected: Vec<[$validator_word; 2]> = runtime_request.search_space.expected
                .iter()
                .map(|[a, b]| [*a as $validator_word, *b as $validator_word])
                .collect();

            let mut runtime = $crate::orchestrator::Orchestrator::<_, _, $engine_word, $validator_word, $bytes, { $bytes - $suffix }>::new(
                start,
                end,
                &data,
                &expected,
                runtime_request.runtime_config.num_threads,
                runtime_request.runtime_config.cap,
                runtime_request.internal_config.cli_tx,
                |task, out| engine::[<$($simd)? search_ $cipher_fn _ $version>](task, out),
                engine::[<$mode _validate_encrypt_ $version>],
                |block| ($converter)(block),
            );

            let (keys, found) = runtime.run();
            (
                keys.into_iter().map(|k| k.to_vec()).collect(),
                found.map(|k| k.to_vec()),
            )
        }
    }};
}

pub(crate) use define_runtime;

macro_rules! define_runtime_with_attrs {
    ([$($attrs:tt)*], $($rest:tt)*) => {
        $crate::backend::macors::define_runtime!($($attrs)* $($rest)*);
    };
}

pub(crate) use define_runtime_with_attrs;

macro_rules! define_runtime_for_mode {
    (
     $base:ident, $version:tt, $engine_word:ty, $converter:expr,
     $prefix:literal, $mode:tt, $cipher_fn:tt, [$($suffix:literal),+ $(,)?]) => {
        $( paste::paste! {
            $crate::backend::macors::define_runtime_with_attrs!(
                [],
                [< $base _s $suffix _ $mode _ $cipher_fn _runtime >],
                $prefix, $suffix, $engine_word, $engine_word, $converter, $version, $mode, $cipher_fn
            );
        } )+
    };

    (
     $attrs:tt,
     $base:ident, $version:tt, $engine_word:ty, $validator_word:ty, $converter:expr,
     $prefix:literal, $mode:tt, $cipher_fn:tt, [$($suffix:literal),+ $(,)?], $simd:tt) => {
        $( paste::paste! {
            $crate::backend::macors::define_runtime_with_attrs!(
                $attrs,
                [< $base _s $suffix _ $mode _ $cipher_fn _runtime >],
                $prefix, $suffix, $engine_word, $validator_word, $converter, $version, $mode, $cipher_fn, $simd
            );
        } )+
    };
}

pub(crate) use define_runtime_for_mode;

macro_rules! define_runtime_variants {
    (
        $base:ident,
        $version:tt,
        $engine_word:ty,
        $converter:expr,
        bytes    = $bytes:literal,
        suffixes = $suffixes:tt,
        modes    = [$($mode:tt),+ $(,)?],
        cipher_fn = $cipher_fn:tt
        $(,)?
    ) => {
        $( $crate::backend::macors::define_runtime_for_mode!($base, $version, $engine_word, $converter, $bytes, $mode, $cipher_fn, $suffixes); )+
    };

    (
        attrs = $attrs:tt,
        $base:ident,
        $version:tt,
        $engine_word:ty,
        $validator_word:ty,
        $converter:expr,
        bytes    = $bytes:literal,
        suffixes = $suffixes:tt,
        modes    = [$($mode:tt),+ $(,)?],
        cipher_fn = $cipher_fn:tt,
        simd     = $simd:tt
        $(,)?
    ) => {
        $( $crate::backend::macors::define_runtime_for_mode!($attrs, $base, $version, $engine_word, $validator_word, $converter, $bytes, $mode, $cipher_fn, $suffixes, $simd); )+
    };
}

pub(crate) use define_runtime_variants;

macro_rules! define_runtime_variants_default {
    (
        attrs = $attrs:tt,
        $base:ident,
        $version:tt,
        $engine_word:ty,
        $validator_word:ty,
        converter = $converter:expr,
        bytes = $bytes:literal
        $(, simd = $simd:tt)?
        $(,)?
    ) => {
        $crate::backend::macors::define_runtime_variants! {
            attrs = $attrs,
            $base,
            $version,
            $engine_word,
            $validator_word,
            $converter,
            bytes = $bytes,
            suffixes = [1, 2, 3, 4],
            modes = [ecb],
            cipher_fn = encrypt,
            $(simd = $simd,)?
        }
        $crate::backend::macors::define_runtime_variants! {
            attrs = $attrs,
            $base,
            $version,
            $engine_word,
            $validator_word,
            $converter,
            bytes = $bytes,
            suffixes = [1, 2, 3, 4],
            modes = [ecb],
            cipher_fn = decrypt,
            $(simd = $simd,)?
        }
        $crate::backend::macors::define_runtime_variants! {
            attrs = $attrs,
            $base,
            $version,
            $engine_word,
            $validator_word,
            $converter,
            bytes = $bytes,
            suffixes = [1, 2, 3, 4],
            modes = [ecb],
            cipher_fn = encrypt_inflight,
            $(simd = $simd,)?
        }
    };

    (
        $base:ident,
        $version:tt,
        $engine_word:ty,
        bytes = $bytes:literal
        $(,)?
    ) => {
        $crate::backend::macors::define_runtime_variants! {
            $base,
            $version,
            $engine_word,
            |x| x,
            bytes = $bytes,
            suffixes = [1, 2, 3, 4],
            modes = [ecb],
            cipher_fn = encrypt,
        }
        $crate::backend::macors::define_runtime_variants! {
            $base,
            $version,
            $engine_word,
            |x| x,
            bytes = $bytes,
            suffixes = [1, 2, 3, 4],
            modes = [ecb],
            cipher_fn = decrypt,
        }
        $crate::backend::macors::define_runtime_variants! {
            $base,
            $version,
            $engine_word,
            |x| x,
            bytes = $bytes,
            suffixes = [1, 2, 3, 4],
            modes = [ecb],
            cipher_fn = encrypt_inflight,
        }
    };
}

pub(crate) use define_runtime_variants_default;

macro_rules! dispatch_for_backend {
    (
        $backend:ident,
        $runtime_request:expr,
        $version:expr,
        $mode:expr,
        $suffix:expr,
        $function:expr
    ) => {
        $crate::backend::macors::dispatch_for_backend_with_versions!(
            $backend,
            $runtime_request,
            $version,
            $mode,
            $suffix,
            $function,
            [
                32_64, 48_72, 48_96, 64_96, 64_128, 96_96, 96_144, 128_128, 128_192, 128_256
            ]
        )
    };
}

pub(crate) use dispatch_for_backend;

macro_rules! dispatch_for_backend_with_versions {
    (
        $backend:ident,
        $runtime_request:expr,
        $version:expr,
        $mode:expr,
        $suffix:expr,
        $function:expr,
        [$($version_name:tt),+ $(,)?]
    ) => {
        paste::paste! {
            match ($version, $mode, $suffix, $function) {
                $(
                    (speck::SpeckVersion::[<Speck $version_name>], $crate::api::CipherMode::Ecb, 1, $crate::api::CipherFunction::Encrypt) => {
                        Ok($crate::backend::versions::[<$backend _ $version_name _s1_ecb_encrypt_runtime>]($runtime_request))
                    }
                    (speck::SpeckVersion::[<Speck $version_name>], $crate::api::CipherMode::Ecb, 1, $crate::api::CipherFunction::Decrypt) => {
                        Ok($crate::backend::versions::[<$backend _ $version_name _s1_ecb_decrypt_runtime>]($runtime_request))
                    }
                    (speck::SpeckVersion::[<Speck $version_name>], $crate::api::CipherMode::Ecb, 1, $crate::api::CipherFunction::EncryptInflight) => {
                        Ok($crate::backend::versions::[<$backend _ $version_name _s1_ecb_encrypt_inflight_runtime>]($runtime_request))
                    }
                    (speck::SpeckVersion::[<Speck $version_name>], $crate::api::CipherMode::Ecb, 2, $crate::api::CipherFunction::Encrypt) => {
                        Ok($crate::backend::versions::[<$backend _ $version_name _s2_ecb_encrypt_runtime>]($runtime_request))
                    }
                    (speck::SpeckVersion::[<Speck $version_name>], $crate::api::CipherMode::Ecb, 2, $crate::api::CipherFunction::Decrypt) => {
                        Ok($crate::backend::versions::[<$backend _ $version_name _s2_ecb_decrypt_runtime>]($runtime_request))
                    }
                    (speck::SpeckVersion::[<Speck $version_name>], $crate::api::CipherMode::Ecb, 2, $crate::api::CipherFunction::EncryptInflight) => {
                        Ok($crate::backend::versions::[<$backend _ $version_name _s2_ecb_encrypt_inflight_runtime>]($runtime_request))
                    }
                    (speck::SpeckVersion::[<Speck $version_name>], $crate::api::CipherMode::Ecb, 3, $crate::api::CipherFunction::Encrypt) => {
                        Ok($crate::backend::versions::[<$backend _ $version_name _s3_ecb_encrypt_runtime>]($runtime_request))
                    }
                    (speck::SpeckVersion::[<Speck $version_name>], $crate::api::CipherMode::Ecb, 3, $crate::api::CipherFunction::Decrypt) => {
                        Ok($crate::backend::versions::[<$backend _ $version_name _s3_ecb_decrypt_runtime>]($runtime_request))
                    }
                    (speck::SpeckVersion::[<Speck $version_name>], $crate::api::CipherMode::Ecb, 3, $crate::api::CipherFunction::EncryptInflight) => {
                        Ok($crate::backend::versions::[<$backend _ $version_name _s3_ecb_encrypt_inflight_runtime>]($runtime_request))
                    }
                    (speck::SpeckVersion::[<Speck $version_name>], $crate::api::CipherMode::Ecb, 4, $crate::api::CipherFunction::Encrypt) => {
                        Ok($crate::backend::versions::[<$backend _ $version_name _s4_ecb_encrypt_runtime>]($runtime_request))
                    }
                    (speck::SpeckVersion::[<Speck $version_name>], $crate::api::CipherMode::Ecb, 4, $crate::api::CipherFunction::Decrypt) => {
                        Ok($crate::backend::versions::[<$backend _ $version_name _s4_ecb_decrypt_runtime>]($runtime_request))
                    }
                    (speck::SpeckVersion::[<Speck $version_name>], $crate::api::CipherMode::Ecb, 4, $crate::api::CipherFunction::EncryptInflight) => {
                        Ok($crate::backend::versions::[<$backend _ $version_name _s4_ecb_encrypt_inflight_runtime>]($runtime_request))
                    }
                )+
                _ => Err($crate::api::DispatchError::UnsupportedCombination {
                    version: $version,
                    mode: $mode,
                    suffix: $suffix,
                }),
            }
        }
    };
}

pub(crate) use dispatch_for_backend_with_versions;
