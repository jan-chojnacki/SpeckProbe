use crate::infrastructure::error::ConfigRepositoryError;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fs;
use std::path::Path;

pub fn load_config<Config>(path: &Path) -> Result<Config, ConfigRepositoryError>
where
    Config: DeserializeOwned,
{
    let content = fs::read_to_string(path)?;
    let config = toml::from_str(&content)?;
    Ok(config)
}

pub fn save_config<Config>(config: &Config, path: &Path) -> Result<(), ConfigRepositoryError>
where
    Config: Serialize,
{
    let toml_str = toml::to_string_pretty(config)?;
    fs::write(path, toml_str)?;
    Ok(())
}
