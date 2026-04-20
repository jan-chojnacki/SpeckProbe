use crate::application::error::ApplicationError;
use crate::domain::config::sample::generate_sample;
use crate::infrastructure::config_repository::{create_config_file, save_config};
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

    let config = generate_sample();
    save_config(&config, &path)?;

    Ok(())
}
