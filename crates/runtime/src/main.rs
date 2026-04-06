use runtime::api::BackendHint::Auto;
use runtime::api::CipherMode::Ecb;
use runtime::api::{CipherConfig, RuntimeConfig, RuntimeRequest, SearchSpace};
use speck::SpeckVersion::Speck32_64;
use std::time::Instant;

fn main() {
    let cipher_config: CipherConfig = CipherConfig {
        cipher_mode: Ecb,
        speck_version: Speck32_64,
    };

    let runtime_config: RuntimeConfig = RuntimeConfig {
        suffix_bytes_size: 3,
        num_threads: 16,
        cap: 256,
        backend_hint: Auto,
    };

    let search_space: SearchSpace = SearchSpace {
        start: vec![0; 5],
        end: vec![255, 0, 0, 0, 0],
        data: vec![[0, 0], [1, 1]],
        expected: vec![[0, 0], [1, 1]],
    };

    let runtime_request: RuntimeRequest = RuntimeRequest {
        cipher_config,
        runtime_config,
        search_space,
    };

    let t0 = Instant::now();

    runtime::backend::dispatch::dispatch(runtime_request).expect("TODO: panic message");

    let t1 = t0.elapsed();

    println!("{:?}", t1);
}
