use runtime::runtime::Runtime;
use std::time::Instant;

use runtime::versions::{runtime1, runtime2};

fn main() {
    unsafe {
        let start = [0; 7];
        let end = [255, 255, 255, 0, 0, 0, 0];
        let data = vec![[0, 0], [0, 0]];
        let expected = vec![[0, 0], [0, 0]];
        // let mut runtime = Runtime::new(
        //     start,
        //     end,
        //     &data,
        //     &expected,
        //     16,
        //     128,
        //     engine::search_encrypt_inflight_32_64,
        //     engine::validate_encrypt_32_64,
        //     |x| x,
        // );

        let t0 = Instant::now();
        // let results = runtime.run();

        let results = runtime2(start, end, &data, &expected, 16, 128);
        let t1 = t0.elapsed();

        dbg!(t1);
        dbg!(results.1);

        let t0 = Instant::now();

        let results = runtime1(start, end, &data, &expected, 16, 128);
        let t1 = t0.elapsed();

        dbg!(t1);
        dbg!(results.1);
    }
}
