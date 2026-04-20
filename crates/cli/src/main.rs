use clap::Parser;
use cli::args::{Args, Commands};
use cli::domain::config::generate_sample::generate_sample;
use cli::domain::config::io::{load_config, save_config};
use cli::helpers::display_results;
use cli::{ProgressUi, display_banner, display_info};
use runtime::Runtime;
use runtime::api::{CipherConfig, RuntimeConfig, SearchSpace};
use std::fs;
use std::path::PathBuf;
use terminal_size::{Width, terminal_size};

fn handle_sample_config(
    config_path: PathBuf,
    force: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    match config_path.exists() {
        true => {
            if !force {
                eprintln!(
                    "File already exists: {:?}. Use -f to overwrite.",
                    config_path
                );
                std::process::exit(1);
            }
        }
        false => {
            if let Some(parent) = config_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::File::create(&config_path)?;
        }
    }

    let config = generate_sample();
    save_config(&config, &config_path)?;

    Ok(())
}

fn handle_search(config_path: PathBuf, spurious: bool) -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config(&config_path)?;

    let cipher_config: CipherConfig = CipherConfig {
        cipher_mode: config.cipher_mode.into(),
        speck_version: config.speck_version.into(),
        cipher_function: config.cipher_function.into(),
    };

    let runtime_config: RuntimeConfig = RuntimeConfig {
        suffix_bytes_size: config.suffix_bytes_size,
        num_threads: config.num_threads,
        cap: config.cap,
        backend_hint: config.backend_hint.into(),
    };

    let search_space: SearchSpace = SearchSpace {
        start: config.start,
        end: config.end,
        data: config.data,
        expected: config.expected,
    };

    display_banner();
    display_info(
        cipher_config.clone(),
        runtime_config.clone(),
        search_space.clone(),
    );

    let mut runtime = Runtime::new(cipher_config, runtime_config.clone(), search_space.clone());
    let rx = runtime.get_rx_channel();

    let ui = ProgressUi::start(
        rx,
        &search_space.start,
        &search_space.end,
        runtime_config.suffix_bytes_size,
    );

    let results = runtime.run().unwrap();
    ui.join();

    display_results(results, spurious);

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match args.command {
        Commands::Search {
            config_path,
            spurious,
        } => handle_search(config_path, spurious)?,
        Commands::SampleConfig { config_path, force } => handle_sample_config(config_path, force)?,
    }

    Ok(())
}
