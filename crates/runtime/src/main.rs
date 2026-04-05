#[cfg(target_arch = "aarch64")]
use runtime::versions::neon_runtime;
#[cfg(target_arch = "x86_64")]
use runtime::versions::{avx2_runtime, avx512_runtime, sse2_runtime};
use std::time::Instant;

use runtime::versions::scalar_runtime;

fn main() {
    let start = [0; 5];
    let end = [16, 0, 0, 0, 0];
    let data = vec![[0, 0], [1, 1]];
    let expected = vec![[0, 0], [1, 1]];

    #[cfg(target_arch = "x86_64")]
    unsafe {
        // let t0 = Instant::now();
        //
        // let results = scalar_runtime(start, end, &data, &expected, 16, 128);
        // let t1 = t0.elapsed();
        //
        // println!("scalar");
        // dbg!(t1);
        // dbg!(results.1);
        //
        // let t0 = Instant::now();
        //
        // let results = sse2_runtime(start, end, &data, &expected, 16, 128);
        // let t1 = t0.elapsed();
        //
        // println!("sse2");
        // dbg!(t1);
        // dbg!(results.1);
        //
        // let t0 = Instant::now();
        //
        // let results = avx2_runtime(start, end, &data, &expected, 16, 128);
        // let t1 = t0.elapsed();
        //
        // println!("avx2");
        // dbg!(t1);
        // dbg!(results.1);

        let t0 = Instant::now();

        let results = avx512_runtime(start, end, &data, &expected, 16, 128);
        let t1 = t0.elapsed();

        println!("avx512");
        dbg!(t1);
        dbg!(results.1);
    }
    #[cfg(target_arch = "aarch64")]
    unsafe {
        let t0 = Instant::now();

        let results = scalar_runtime(start, end, &data, &expected, 16, 128);
        let t1 = t0.elapsed();

        println!("scalar");
        dbg!(t1);
        dbg!(results.1);

        let t0 = Instant::now();

        let results = neon_runtime(start, end, &data, &expected, 16, 128);
        let t1 = t0.elapsed();

        println!("neon");
        dbg!(t1);
        dbg!(results.1);
    }
}
