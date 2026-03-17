use thiserror::Error;
use crate::domain::block::BlockError;

#[derive(Debug, Error, Eq, PartialEq)]
pub enum RequestError {
    #[error(transparent)]
    BlockError(#[from] BlockError),
}