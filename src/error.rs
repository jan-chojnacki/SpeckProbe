use crate::store::StoreError;

#[derive(Debug, thiserror::Error)]
pub enum ProbeError {
    /// A file-system or serialization error from the store layer.
    #[error(transparent)]
    Store(#[from] StoreError),
    /// The executor dispatch failed (e.g. unsupported backend or bad config).
    #[error(transparent)]
    Dispatch(#[from] crate::search::executor::DispatchError),
    /// The cipher rejected the provided key or IV.
    #[error(transparent)]
    Cipher(#[from] crate::cipher::error::SPECKError),
    /// The output file already exists and `--force` was not supplied.
    #[error("file already exists: {0:?}, use -f/--force to overwrite")]
    FileAlreadyExists(std::path::PathBuf),
}
