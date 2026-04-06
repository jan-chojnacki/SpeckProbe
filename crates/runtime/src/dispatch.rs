use crate::versions::{self, RuntimeRequest};
use paste::paste;
use speck::SpeckVersion;

pub type DispatchOutput = (Vec<Vec<u8>>, Option<Vec<u8>>);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CipherMode {
    Ecb,
    Cbc,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DispatchError {
    UnsupportedSuffix {
        suffix: usize,
    },
    UnsupportedMode {
        mode: CipherMode,
    },
    UnsupportedCombination {
        version: SpeckVersion,
        mode: CipherMode,
        suffix: usize,
    },
}

pub fn dispatch(runtime_request: RuntimeRequest) -> Result<DispatchOutput, DispatchError> {
    let suffix: usize = runtime_request.runtime_config.suffix_bytes_size;
    if !(1..=4).contains(&suffix) {
        return Err(DispatchError::UnsupportedSuffix { suffix });
    }

    let version: SpeckVersion = runtime_request.cipher_config.speck_version;

    let mode: CipherMode = runtime_request.cipher_config.cipher_mode;

    if mode != CipherMode::Ecb {
        return Err(DispatchError::UnsupportedMode { mode });
    }

    dispatch_backend(runtime_request, suffix, version, mode)
}

macro_rules! dispatch_for_backend {
    (
        $backend:ident,
        $runtime_request:expr,
        $version:expr,
        $mode:expr,
        $suffix:expr
    ) => {
        dispatch_for_backend_with_versions!(
            $backend,
            $runtime_request,
            $version,
            $mode,
            $suffix,
            [
                32_64, 48_72, 48_96, 64_96, 64_128, 96_96, 96_144, 128_128, 128_192, 128_256
            ]
        )
    };
}

macro_rules! dispatch_for_backend_with_versions {
    (
        $backend:ident,
        $runtime_request:expr,
        $version:expr,
        $mode:expr,
        $suffix:expr,
        [$($version_name:tt),+ $(,)?]
    ) => {
        paste! {
            match ($version, $mode, $suffix) {
                $(
                    (SpeckVersion::[<Speck $version_name>], CipherMode::Ecb, 1) => {
                        Ok(versions::[<$backend _ $version_name _s1_ecb_runtime>]($runtime_request))
                    }
                    (SpeckVersion::[<Speck $version_name>], CipherMode::Ecb, 2) => {
                        Ok(versions::[<$backend _ $version_name _s2_ecb_runtime>]($runtime_request))
                    }
                    (SpeckVersion::[<Speck $version_name>], CipherMode::Ecb, 3) => {
                        Ok(versions::[<$backend _ $version_name _s3_ecb_runtime>]($runtime_request))
                    }
                    (SpeckVersion::[<Speck $version_name>], CipherMode::Ecb, 4) => {
                        Ok(versions::[<$backend _ $version_name _s4_ecb_runtime>]($runtime_request))
                    }
                )+
                _ => Err(DispatchError::UnsupportedCombination {
                    version: $version,
                    mode: $mode,
                    suffix: $suffix,
                }),
            }
        }
    };
}

#[cfg_attr(
    target_arch = "x86_64",
    multiversion::multiversion(
        targets("x86_64+avx512bw", "x86_64+avx2", "x86_64+sse2"),
        dispatcher = "default"
    )
)]
#[cfg_attr(
    target_arch = "aarch64",
    multiversion::multiversion(targets("aarch64+neon"), dispatcher = "default")
)]
fn dispatch_backend(
    runtime_request: RuntimeRequest,
    suffix: usize,
    version: SpeckVersion,
    mode: CipherMode,
) -> Result<DispatchOutput, DispatchError> {
    multiversion::target::match_target! {
        "x86_64+avx512bw" => unsafe { dispatch_for_backend!(avx512, runtime_request, version, mode, suffix) },
        "x86_64+avx2" => unsafe { dispatch_for_backend!(avx2, runtime_request, version, mode, suffix) },
        "x86_64+sse2" => unsafe { dispatch_for_backend!(sse2, runtime_request, version, mode, suffix) },
        "aarch64+neon" => unsafe { dispatch_for_backend!(neon, runtime_request, version, mode, suffix) },
        _ => dispatch_for_backend!(scalar, runtime_request, version, mode, suffix),
    }
}
