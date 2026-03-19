use crate::api::request::SearchRangeRequest;
use crate::backend::avx::dispatch::avx_search_range_impl;
use crate::domain::key::Key;
use crate::{SearchEngineBackend, SearchEngineBackendError};

pub struct SearchEngineAvx {}

impl SearchEngineBackend for SearchEngineAvx {
    fn search_range_encrypt(
        search_range_request: SearchRangeRequest,
    ) -> Result<Option<Vec<Key>>, SearchEngineBackendError> {
        unsafe { avx_search_range_impl(search_range_request) }
    }

    fn search_range_decrypt(
        search_range_request: SearchRangeRequest,
    ) -> Result<Option<Vec<Key>>, SearchEngineBackendError> {
        unsafe { avx_search_range_impl(search_range_request) }
    }
}
