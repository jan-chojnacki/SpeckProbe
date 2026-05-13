use crate::store::StoreError;

#[derive(Debug, thiserror::Error)]
pub enum ProbeError {
    #[error("configuration error: {0}")]
    Store(#[from] StoreError),
    #[error("dispatch error: {0}")]
    Dispatch(#[from] crate::search::executor::DispatchError),
    #[error("cipher error: {0}")]
    Cipher(#[from] crate::cipher::error::SPECKError),
    #[error("file already exists: {0:?}, use -f/--force to overwrite")]
    FileAlreadyExists(std::path::PathBuf),
}
