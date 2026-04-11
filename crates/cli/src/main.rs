use cli::ProgressUi;
use runtime::Runtime;
use runtime::api::BackendHint::Auto;
use runtime::api::CipherMode::Ecb;
use runtime::api::{CipherConfig, RuntimeConfig, SearchSpace};
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
        end: vec![255, 15, 0, 0, 0],
        data: vec![[0, 0], [1, 1]],
        expected: vec![[0, 0], [1, 1]],
    };

    let mut runtime = Runtime::new(cipher_config, runtime_config, search_space.clone());
    let rx = runtime.get_rx_channel();

    let ui = ProgressUi::start(rx, &search_space.start, &search_space.end);

    let t0 = Instant::now();
    let result = runtime.run().unwrap();
    let elapsed = t0.elapsed();

    ui.join();

    println!("{:?}", elapsed);
    println!("{}", result.0.len());
}
