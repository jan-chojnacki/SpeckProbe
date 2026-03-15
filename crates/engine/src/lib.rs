pub mod block;
pub mod key;
pub mod key_iterator;
pub mod search_engine_scalar;
mod search_range_request;
pub mod speck_version;

use search_range_request::SearchRangeRequest;
use crate::key::Key;

pub trait SearchEngineBackend {
    fn search_range_encrypt(search_range_request: SearchRangeRequest) -> Option<Vec<Key>>;
    fn search_range_decrypt(search_range_request: SearchRangeRequest) -> Option<Vec<Key>>;
}
