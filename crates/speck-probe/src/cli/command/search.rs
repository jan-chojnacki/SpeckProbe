use crate::cli::display::{display_banner, display_info, display_results};
use crate::cli::progress::ProgressUi;
use crate::probe::ProbeError;
use crate::runtime::Runtime;
use crate::runtime::api::{CipherConfig, RuntimeConfig, SearchSpace};
use crate::search::SearchConfig;
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

/// Converts a [`SearchConfig`] into the runtime API structs needed to start a search.
fn into_runtime_configs(config: SearchConfig) -> (CipherConfig, RuntimeConfig, SearchSpace) {
    let cipher_config = CipherConfig {
        cipher_mode: config.cipher_mode.into(),
        speck_version: config.speck_version.into(),
        cipher_function: config.cipher_function.into(),
    };
    let runtime_config = RuntimeConfig {
        suffix_bytes_size: config.suffix_bytes_size,
        num_threads: config.num_threads,
        backend_hint: config.backend_hint.into(),
    };
    let search_space = SearchSpace {
        start: config.start,
        end: config.end,
        data: config.data,
        expected: config.expected,
    };
    (cipher_config, runtime_config, search_space)
}
