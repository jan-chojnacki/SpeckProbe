use crate::domain::config::Config;
use std::fs;
use std::path::PathBuf;

pub fn save_config(config: &Config, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let toml_str = toml::to_string_pretty(config)?;
    fs::write(path, toml_str)?;
    Ok(())
}

pub fn load_config(path: &PathBuf) -> Result<Config, Box<dyn std::error::Error>> {
    let file = fs::read_to_string(path)?;
    let config = toml::from_str(&file)?;
    Ok(config)
}
