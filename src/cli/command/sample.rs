use crate::error::ProbeError;
use crate::store;
use crate::store::StoreError;
use std::fs;
use std::path::{Path, PathBuf};

/// Writes a sample search config to `path`.
pub fn handle_sample_search(path: PathBuf, force: bool) -> Result<(), ProbeError> {
    execute_search(path, force)
}

/// Writes a sample benchmark config to `path`.
pub fn handle_sample_benchmark(path: PathBuf, force: bool) -> Result<(), ProbeError> {
    execute_benchmark(path, force)
}

/// Writes a sample search config to `path`.
fn execute_search(path: PathBuf, force: bool) -> Result<(), ProbeError> {
    prepare_path(&path, force)?;
    store::save_config(&crate::search::sample(), &path)?;
    Ok(())
}

/// Writes a sample benchmark config to `path`.
fn execute_benchmark(path: PathBuf, force: bool) -> Result<(), ProbeError> {
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
