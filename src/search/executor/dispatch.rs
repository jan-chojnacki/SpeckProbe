use crate::search::executor::backend::scalar;
use crate::speck::SpeckVersion;

#[cfg(target_arch = "x86_64")]
use crate::search::executor::backend::x86_64;

#[cfg(target_arch = "aarch64")]
use crate::search::executor::backend::aarch64;
use crate::search::executor::error::DispatchError;
use crate::search::executor::{
    BackendHint, CipherFunction, CipherMode, DispatchOutput, RuntimeRequest,
};

pub fn dispatch(runtime_request: RuntimeRequest) -> Result<DispatchOutput, DispatchError> {
    let suffix: usize = runtime_request.runtime_config.suffix_bytes_size;
    if !(1..=4).contains(&suffix) {
        return Err(DispatchError::UnsupportedSuffix { suffix });
    }

    let version: SpeckVersion = runtime_request.cipher_config.speck_version;
    let mode: CipherMode = runtime_request.cipher_config.cipher_mode;
    let function: CipherFunction = runtime_request.cipher_config.cipher_function;

    if mode == CipherMode::Cbc && runtime_request.search_space.iv.is_none() {
        return Err(DispatchError::MissingIv);
    }

    Ok(match runtime_request.runtime_config.backend_hint {
        BackendHint::Auto => dispatch_backend(runtime_request, suffix, version, function, mode),
        BackendHint::Scalar => scalar::dispatch(runtime_request, version, suffix, function, mode),
        #[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
        BackendHint::Sse2 => unsafe {
            x86_64::sse2::dispatch(runtime_request, version, suffix, function, mode)
        },
        #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
        BackendHint::Avx2 => unsafe {
            x86_64::avx2::dispatch(runtime_request, version, suffix, function, mode)
        },
        #[cfg(all(target_arch = "x86_64", target_feature = "avx512bw"))]
        BackendHint::Avx512 => unsafe {
            x86_64::avx512::dispatch(runtime_request, version, suffix, function, mode)
        },
        #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
        BackendHint::Neon => unsafe {
            aarch64::neon::dispatch(runtime_request, version, suffix, function, mode)
        },
    })
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
    request: RuntimeRequest,
    suffix: usize,
    version: SpeckVersion,
    function: CipherFunction,
    mode: CipherMode,
) -> DispatchOutput {
    multiversion::target::match_target! {
        "x86_64+avx512bw" => unsafe { x86_64::avx512::dispatch(request, version, suffix, function, mode) },
        "x86_64+avx2" => unsafe { x86_64::avx2::dispatch(request, version, suffix, function, mode) },
        "x86_64+sse2" => unsafe { x86_64::sse2::dispatch(request, version, suffix, function, mode) },
        "aarch64+neon" => unsafe { aarch64::neon::dispatch(request, version, suffix, function, mode) },
        _ => scalar::dispatch(request, version, suffix, function, mode),
    }
}
