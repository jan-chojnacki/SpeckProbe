use runtime::runtime::Runtime;
use std::time::Instant;

//base scalar: total: 705.452474ms
fn main() {
    let start = [0; 7];
    let end = [255, 255, 255, 0, 0, 0, 0];
    let data = vec![[0, 0], [0, 0]];
    let expected = vec![[0, 0], [0, 0]];
    let mut runtime = Runtime::new(
        start,
        end,
        data,
        expected,
        16,
        128,
        engine::search_encrypt_inflight_32_64,
        engine::validate_encrypt_32_64,
    );

    let start = Instant::now();
    let results = runtime.run();
    let end = start.elapsed();
    dbg!(end);
    dbg!(results.1);
}
