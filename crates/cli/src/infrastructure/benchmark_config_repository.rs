use crate::domain::config::benchmark::BenchmarkConfig;
use crate::infrastructure::error::ConfigRepositoryError;
use std::fs;
use std::path::Path;

pub fn load_benchmark_config(path: &Path) -> Result<BenchmarkConfig, ConfigRepositoryError> {
    let content = fs::read_to_string(path)?;
    let config = toml::from_str(&content)?;
    Ok(config)
}

pub fn save_benchmark_config(
    config: &BenchmarkConfig,
    path: &Path,
) -> Result<(), ConfigRepositoryError> {
    let toml_str = toml::to_string_pretty(config)?;
    fs::write(path, toml_str)?;
    Ok(())
}
