use crate::infrastructure::error::ConfigRepositoryError;

#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error(transparent)]
    Config(#[from] ConfigRepositoryError),
    #[error("{0:?}")] //TODO thiserror::Error w runtime
    Dispatch(runtime::api::DispatchError),
    #[error("invalid hex: {0}")]
    InvalidHex(String),
    #[error("data length {got} is not a multiple of block size {expected_multiple}")]
    InvalidBlockSize {
        expected_multiple: usize,
        got: usize,
    },
    #[error(transparent)]
    Cipher(#[from] utils::SPECKError),
    #[error("missing required argument: {0}")]
    MissingArg(&'static str),
}

impl From<runtime::api::DispatchError> for ApplicationError {
    fn from(e: runtime::api::DispatchError) -> Self {
        ApplicationError::Dispatch(e)
    }
}
