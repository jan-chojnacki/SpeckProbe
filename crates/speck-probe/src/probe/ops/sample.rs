use crate::probe::config;
use crate::probe::error::ProbeError;
use crate::store::{self, StoreError};
use std::fs;
use std::path::{Path, PathBuf};

/// Writes a sample search config to `path`.
pub fn search(path: PathBuf, force: bool) -> Result<(), ProbeError> {
    prepare_path(&path, force)?;
    store::save_config(&config::search::sample(), &path)?;
    Ok(())
}

/// Writes a sample encrypt config to `path`.
pub fn encrypt(path: PathBuf, force: bool) -> Result<(), ProbeError> {
    prepare_path(&path, force)?;
    store::save_config(&config::cipher::sample(), &path)?;
    Ok(())
}

/// Writes a sample benchmark config to `path`.
pub fn benchmark(path: PathBuf, force: bool) -> Result<(), ProbeError> {
    prepare_path(&path, force)?;
    store::save_config(&crate::benchmark::sample(), &path)?;
    Ok(())
}

fn prepare_path(path: &Path, force: bool) -> Result<(), ProbeError> {
    if path.exists() && !force {
        return Err(ProbeError::FileAlreadyExists(path.to_path_buf()));
    }
    if !path.exists() {
        create_file(path)?;
    }
    Ok(())
}

fn create_file(path: &Path) -> Result<(), StoreError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::File::create(path)?;
    Ok(())
}
