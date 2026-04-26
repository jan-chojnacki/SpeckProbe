use crate::application::error::ApplicationError;
use crate::infrastructure::config_repository::load_config;
use crate::presentation::display::{display_banner, display_info, display_results};
use crate::presentation::progress::ProgressUi;
use runtime::Runtime;
use runtime::api::{CipherConfig, RuntimeConfig, SearchSpace};
use std::path::PathBuf;

pub fn execute(path: PathBuf, spurious: bool) -> Result<(), ApplicationError> {
    let config = load_config(&path)?;

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
