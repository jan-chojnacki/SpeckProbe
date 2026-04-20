use crate::domain::config::Config;
use crate::infrastructure::error::ConfigRepositoryError;
use std::fs;
use std::path::Path;

pub fn load_config(path: &Path) -> Result<Config, ConfigRepositoryError> {
    let content = fs::read_to_string(path)?;
    let config = toml::from_str(&content)?;
    Ok(config)
}

pub fn save_config(config: &Config, path: &Path) -> Result<(), ConfigRepositoryError> {
    let toml_str = toml::to_string_pretty(config)?;
    fs::write(path, toml_str)?;
    Ok(())
}

pub fn create_config_file(path: &Path) -> Result<(), ConfigRepositoryError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::File::create(path)?;
    Ok(())
}
