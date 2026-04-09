use crate::api::{BackendHint, CipherMode, DispatchError, DispatchOutput, RuntimeRequest};
use crate::backend::macors::dispatch_for_backend;
use speck::SpeckVersion;

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

    match runtime_request.runtime_config.backend_hint {
        BackendHint::Auto => dispatch_backend(runtime_request, suffix, version, mode),
        BackendHint::Scalar => {
            dispatch_for_backend!(scalar, runtime_request, version, mode, suffix)
        }
        #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
        BackendHint::Sse2 => unsafe {
            dispatch_for_backend!(sse2, runtime_request, version, mode, suffix)
        },
        #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
        BackendHint::Avx2 => unsafe {
            dispatch_for_backend!(avx2, runtime_request, version, mode, suffix)
        },
        #[cfg(all(target_arch = "x86_64", target_feature = "avx512bw"))]
        BackendHint::Avx512 => unsafe {
            dispatch_for_backend!(avx512, runtime_request, version, mode, suffix)
        },
        #[cfg(target_arch = "aarch64")]
        BackendHint::Neon => unsafe {
            dispatch_for_backend!(neon, runtime_request, version, mode, suffix)
        },
    }
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
