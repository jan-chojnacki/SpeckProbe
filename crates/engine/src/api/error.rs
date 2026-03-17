use crate::domain::block::BlockError;
use thiserror::Error;

#[derive(Debug, Error, Eq, PartialEq)]
pub enum RequestError {
    #[error(transparent)]
    BlockError(#[from] BlockError),
}
