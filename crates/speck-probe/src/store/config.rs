use crate::store::StoreError;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fs;
use std::path::Path;

/// Deserializes a TOML file at `path` into `Config`.
pub fn load<Config>(path: &Path) -> Result<Config, StoreError>
where
    Config: DeserializeOwned,
{
    let content = fs::read_to_string(path)?;
    let config = toml::from_str(&content)?;
    Ok(config)
}

/// Serializes `config` as pretty TOML and writes it to `path`.
pub fn save<Config>(config: &Config, path: &Path) -> Result<(), StoreError>
where
    Config: Serialize,
{
    let toml_str = toml::to_string_pretty(config)?;
    fs::write(path, toml_str)?;
    Ok(())
}
