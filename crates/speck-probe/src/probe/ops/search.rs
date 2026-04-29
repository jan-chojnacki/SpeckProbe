use crate::probe::config::SearchConfig;
use crate::runtime::api::{CipherConfig, RuntimeConfig, SearchSpace};

/// Converts a [`SearchConfig`] into the runtime API structs needed to start a search.
pub fn into_runtime_configs(config: SearchConfig) -> (CipherConfig, RuntimeConfig, SearchSpace) {
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
