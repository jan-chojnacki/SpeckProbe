use crate::api::request::SearchRangeRequest;
use crate::backend::scalar::dispatch::search_range_impl;
use crate::domain::key::Key;
use crate::{SearchEngineBackend, SearchEngineBackendError};

pub struct SearchEngineScalar {}

impl SearchEngineBackend for SearchEngineScalar {
    fn search_range_encrypt(
        search_range_request: SearchRangeRequest,
    ) -> Result<Vec<Key>, SearchEngineBackendError> {
        search_range_impl(search_range_request)
    }

    fn search_range_decrypt(
        search_range_request: SearchRangeRequest,
    ) -> Result<Vec<Key>, SearchEngineBackendError> {
        search_range_impl(search_range_request)
    }
}
