use crate::application::error::ApplicationError;
use crate::domain::config::benchmark_sample::generate_benchmark_sample;
use crate::infrastructure::benchmark_config_repository::save_benchmark_config;
use crate::infrastructure::config_repository::create_config_file;
use std::path::PathBuf;

pub fn execute(path: PathBuf, force: bool) -> Result<(), ApplicationError> {
    if path.exists() {
        if !force {
            eprintln!("File already exists: {:?}. Use -f to overwrite.", path);
            std::process::exit(1);
        }
    } else {
        create_config_file(&path)?;
    }

    let config = generate_benchmark_sample();
    save_benchmark_config(&config, &path)?;

    Ok(())
}
