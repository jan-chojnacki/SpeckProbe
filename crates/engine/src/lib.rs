pub mod api;
pub mod backend;
pub mod domain;

use crate::api::request::{Operation, SearchRangeRequest};
#[cfg(target_arch = "x86_64")]
use crate::backend::avx2::engine::SearchEngineAVX2;
#[cfg(target_arch = "x86_64")]
use crate::backend::avx512::engine::SearchEngineAVX512;
#[cfg(target_arch = "aarch64")]
use crate::backend::neon::engine::SearchEngineNeon;
use crate::backend::scalar::engine::SearchEngineScalar;
#[cfg(target_arch = "x86_64")]
use crate::backend::sse2::engine::SearchEngineSSE2;
use domain::block::BlockError;
use domain::key::Key;
use domain::key_iterator::KeyIteratorError;
use thiserror::Error;

#[derive(Debug, Error, Eq, PartialEq)]
pub enum SearchEngineBackendError {
    #[error(transparent)]
    BlockError(#[from] BlockError),
    #[error(transparent)]
    KeyIteratorError(#[from] KeyIteratorError),
}

pub trait SearchEngineBackend {
    fn handle_request(
        search_range_request: SearchRangeRequest,
    ) -> Result<Option<Vec<Key>>, SearchEngineBackendError> {
        match search_range_request.operation {
            Operation::Encrypt => Self::search_range_encrypt(search_range_request),
            Operation::Decrypt => Self::search_range_decrypt(search_range_request),
        }
    }
    fn search_range_encrypt(
        search_range_request: SearchRangeRequest,
    ) -> Result<Option<Vec<Key>>, SearchEngineBackendError>;
    fn search_range_decrypt(
        search_range_request: SearchRangeRequest,
    ) -> Result<Option<Vec<Key>>, SearchEngineBackendError>;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, strum::Display)]
pub enum SearchBackend {
    Auto,
    Scalar,
    #[cfg(target_arch = "x86_64")]
    SSE2,
    #[cfg(target_arch = "x86_64")]
    AVX2,
    #[cfg(target_arch = "x86_64")]
    AVX512,
}

impl Default for SearchBackend {
    fn default() -> Self {
        Self::Auto
    }
}

pub fn search_range(
    search_range_request: SearchRangeRequest,
) -> Result<Option<Vec<Key>>, SearchEngineBackendError> {
    search_range_with_backend(search_range_request, SearchBackend::Auto)
}

pub fn search_range_with_backend(
    search_range_request: SearchRangeRequest,
    backend: SearchBackend,
) -> Result<Option<Vec<Key>>, SearchEngineBackendError> {
    match backend {
        SearchBackend::Auto => search_range_auto(search_range_request),
        SearchBackend::Scalar => SearchEngineScalar::handle_request(search_range_request),
        #[cfg(target_arch = "x86_64")]
        SearchBackend::SSE2 => SearchEngineSSE2::handle_request(search_range_request),
        #[cfg(target_arch = "x86_64")]
        SearchBackend::AVX2 => {
            if std::arch::is_x86_feature_detected!("avx2") {
                SearchEngineAVX2::handle_request(search_range_request)
            } else {
                search_range_auto(search_range_request)
            }
        }
        #[cfg(target_arch = "x86_64")]
        SearchBackend::AVX512 => {
            if std::arch::is_x86_feature_detected!("avx512f")
                && std::arch::is_x86_feature_detected!("avx512bw")
            {
                SearchEngineAVX512::handle_request(search_range_request)
            } else {
                search_range_auto(search_range_request)
            }
        }
    }
}

#[cfg(target_arch = "x86_64")]
#[multiversion::multiversion(targets("x86_64+avx512f+avx512bw", "x86_64+avx2", "x86_64+sse2"))]
fn search_range_auto(
    search_range_request: SearchRangeRequest,
) -> Result<Option<Vec<Key>>, SearchEngineBackendError> {
    if multiversion::target::target_cfg_f!(all(
        target_arch = "x86_64",
        target_feature = "avx512f",
        target_feature = "avx512bw"
    )) {
        return SearchEngineAVX512::handle_request(search_range_request);
    }

    if multiversion::target::target_cfg_f!(all(target_arch = "x86_64", target_feature = "avx2")) {
        return SearchEngineAVX2::handle_request(search_range_request);
    }

    if multiversion::target::target_cfg_f!(all(target_arch = "x86_64", target_feature = "sse2")) {
        return SearchEngineSSE2::handle_request(search_range_request);
    }

    SearchEngineScalar::handle_request(search_range_request)
}

#[cfg(target_arch = "aarch64")]
#[multiversion::multiversion(targets("aarch64+neon"))]
fn search_range_auto(
    search_range_request: SearchRangeRequest,
) -> Result<Option<Vec<Key>>, SearchEngineBackendError> {
    if multiversion::target::target_cfg_f!(all(target_arch = "aarch64", target_feature = "neon")) {
        return SearchEngineNeon::handle_request(search_range_request);
    }

    SearchEngineScalar::handle_request(search_range_request)
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
fn search_range_auto(
    search_range_request: SearchRangeRequest,
) -> Result<Option<Vec<Key>>, SearchEngineBackendError> {
    SearchEngineScalar::handle_request(search_range_request)
}
