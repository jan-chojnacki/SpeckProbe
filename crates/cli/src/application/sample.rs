use crate::application::error::ApplicationError;
use crate::domain::config::benchmark::generate_benchmark_sample;
use crate::domain::config::cipher::generate_cipher_sample;
use crate::domain::config::search::generate_sample;
use crate::infrastructure::config_repository::save_config;
use crate::infrastructure::error::ConfigRepositoryError;
use std::fs;
use std::path::{Path, PathBuf};

pub fn search(path: PathBuf, force: bool) -> Result<(), ApplicationError> {
    check_if_path_exists(&path, force)?;

    let config = generate_sample();
    save_config(&config, &path)?;

    Ok(())
}

pub fn encrypt(path: PathBuf, force: bool) -> Result<(), ApplicationError> {
    check_if_path_exists(&path, force)?;

    let config = generate_cipher_sample();
    save_config(&config, &path)?;

    Ok(())
}

pub fn benchmark(path: PathBuf, force: bool) -> Result<(), ApplicationError> {
    check_if_path_exists(&path, force)?;

    let config = generate_benchmark_sample();
    save_config(&config, &path)?;

    Ok(())
}

fn check_if_path_exists(path: &Path, force: bool) -> Result<(), ConfigRepositoryError> {
    if path.exists() {
        if !force {
            eprintln!("File already exists: {:?}. Use -f to overwrite.", path);
            std::process::exit(1);
        }
    } else {
        create_file(path)?;
    }

    Ok(())
}

fn create_file(path: &Path) -> Result<(), ConfigRepositoryError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::File::create(path)?;
    Ok(())
}
