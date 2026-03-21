use crate::api::request::SearchRangeRequest;
use crate::backend::avx512::dispatch::avx512_search_range_impl;
use crate::domain::key::Key;
use crate::{SearchEngineBackend, SearchEngineBackendError};

pub struct SearchEngineAVX512 {}

impl SearchEngineBackend for SearchEngineAVX512 {
    fn search_range_encrypt(
        search_range_request: SearchRangeRequest,
    ) -> Result<Option<Vec<Key>>, SearchEngineBackendError> {
        unsafe { avx512_search_range_impl(search_range_request) }
    }

    fn search_range_decrypt(
        search_range_request: SearchRangeRequest,
    ) -> Result<Option<Vec<Key>>, SearchEngineBackendError> {
        unsafe { avx512_search_range_impl(search_range_request) }
    }
}
