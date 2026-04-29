use crate::cli::display::{display_banner, display_info, display_results};
use crate::cli::progress::ProgressUi;
use crate::probe::ops::search::into_runtime_configs;
use crate::probe::{ProbeError, config::SearchConfig};
use crate::runtime::Runtime;
use crate::store::load_config;
use std::path::PathBuf;

/// Loads a search config, runs the key search, and prints results.
pub fn execute(path: PathBuf, spurious: bool) -> Result<(), ProbeError> {
    let config = load_config::<SearchConfig>(&path)?;
    let (cipher_config, runtime_config, search_space) = into_runtime_configs(config);

    display_banner();
    display_info(
        cipher_config.clone(),
        runtime_config.clone(),
        search_space.clone(),
    );

    let mut runtime = Runtime::new(cipher_config, runtime_config.clone(), search_space.clone());
    let rx = runtime.enable_progress();
    let ui = ProgressUi::start(
        rx,
        &search_space.start,
        &search_space.end,
        runtime_config.suffix_bytes_size,
    );

    let results = runtime.run()?;
    ui.join();

    display_results(results, spurious);
    Ok(())
}
