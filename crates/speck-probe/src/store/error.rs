/// Errors that can occur when reading or writing persistent data.
#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    TomlDe(#[from] toml::de::Error),
    #[error(transparent)]
    TomlSer(#[from] toml::ser::Error),
    #[error(transparent)]
    Csv(#[from] csv::Error),
}
