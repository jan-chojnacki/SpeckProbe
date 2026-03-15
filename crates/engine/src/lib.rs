pub mod block;
pub mod key;
pub mod key_iterator;
pub mod search_engine_scalar;
mod search_range_request;
pub mod speck_version;

use crate::block::BlockError;
use crate::key::Key;
use crate::key_iterator::KeyIteratorError;
use search_range_request::SearchRangeRequest;
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
