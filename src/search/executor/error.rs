#[derive(Debug, Copy, Clone, Eq, PartialEq, thiserror::Error)]
pub enum DispatchError {
    #[error("unsupported suffix size: {suffix}")]
    UnsupportedSuffix { suffix: usize },
    #[error("missing IV")]
    MissingIv,
}
