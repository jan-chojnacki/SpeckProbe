pub mod api;
pub mod backend;
pub mod domain;

use crate::api::request::SearchRangeRequest;
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
    fn search_range_encrypt(
        search_range_request: SearchRangeRequest,
    ) -> Result<Option<Vec<Key>>, SearchEngineBackendError>;
    fn search_range_decrypt(
        search_range_request: SearchRangeRequest,
    ) -> Result<Option<Vec<Key>>, SearchEngineBackendError>;
}
