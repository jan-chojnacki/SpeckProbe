use crate::api::request::SearchRangeRequest;
use crate::backend::avx2::dispatch::avx2_search_range_impl;
use crate::domain::key::Key;
use crate::{SearchEngineBackend, SearchEngineBackendError};

pub struct SearchEngineAVX2 {}

impl SearchEngineBackend for SearchEngineAVX2 {
    fn search_range_encrypt(
        search_range_request: SearchRangeRequest,
    ) -> Result<Option<Vec<Key>>, SearchEngineBackendError> {
        unsafe { avx2_search_range_impl(search_range_request) }
    }

    fn search_range_decrypt(
        search_range_request: SearchRangeRequest,
    ) -> Result<Option<Vec<Key>>, SearchEngineBackendError> {
        unsafe { avx2_search_range_impl(search_range_request) }
    }
}
