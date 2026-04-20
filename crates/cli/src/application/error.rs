use crate::infrastructure::error::ConfigRepositoryError;

#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error(transparent)]
    Config(#[from] ConfigRepositoryError),
    #[error("{0:?}")] //TODO thiserror::Error w runtime
    Dispatch(runtime::api::DispatchError),
}

impl From<runtime::api::DispatchError> for ApplicationError {
    fn from(e: runtime::api::DispatchError) -> Self {
        ApplicationError::Dispatch(e)
    }
}
