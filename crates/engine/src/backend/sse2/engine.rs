use crate::api::request::SearchRangeRequest;
use crate::backend::sse2::dispatch::sse2_search_range_impl;
use crate::domain::key::Key;
use crate::{SearchEngineBackend, SearchEngineBackendError};

pub struct SearchEngineSSE2 {}

impl SearchEngineBackend for SearchEngineSSE2 {
    fn search_range_encrypt(
        search_range_request: SearchRangeRequest,
    ) -> Result<Vec<Key>, SearchEngineBackendError> {
        unsafe { sse2_search_range_impl(search_range_request) }
    }

    fn search_range_decrypt(
        search_range_request: SearchRangeRequest,
    ) -> Result<Vec<Key>, SearchEngineBackendError> {
        unsafe { sse2_search_range_impl(search_range_request) }
    }
}
