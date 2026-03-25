use crate::api::request::SearchRangeRequest;
use crate::backend::neon::dispatch::neon_search_range_impl;
use crate::domain::key::Key;
use crate::{SearchEngineBackend, SearchEngineBackendError};

pub struct SearchEngineNeon {}

impl SearchEngineBackend for SearchEngineNeon {
    fn search_range_encrypt(
        search_range_request: SearchRangeRequest,
    ) -> Result<Vec<Key>, SearchEngineBackendError> {
        unsafe { neon_search_range_impl(search_range_request) }
    }

    fn search_range_decrypt(
        search_range_request: SearchRangeRequest,
    ) -> Result<Vec<Key>, SearchEngineBackendError> {
        unsafe { neon_search_range_impl(search_range_request) }
    }
}
