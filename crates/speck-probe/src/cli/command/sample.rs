use crate::probe::ProbeError;
use crate::probe::ops::sample;
use std::path::PathBuf;

/// Writes a sample search config to `path`.
pub fn search(path: PathBuf, force: bool) -> Result<(), ProbeError> {
    sample::search(path, force)
}

/// Writes a sample encrypt config to `path`.
pub fn encrypt(path: PathBuf, force: bool) -> Result<(), ProbeError> {
    sample::encrypt(path, force)
}

/// Writes a sample benchmark config to `path`.
pub fn benchmark(path: PathBuf, force: bool) -> Result<(), ProbeError> {
    sample::benchmark(path, force)
}
