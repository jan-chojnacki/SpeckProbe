macro_rules! define_cipher_bench {
    (
        $(#[$meta:meta])*
        $fn_name:ident,
        prefix = $prefix:literal,
        key = $key:expr,
        pt = $pt:expr,
        encrypt = $encrypt:path,
        decrypt = $decrypt:path
    ) => {
        $(#[$meta])*
        fn $fn_name(g: &mut BenchmarkGroup<WallTime>) {
            let key = $key;
            let pt = $pt;
            let ct = $encrypt(pt, key);

            g.bench_function(format!("{}/encrypt", $prefix), |b| {
                b.iter(|| {
                    let out = $encrypt(black_box(pt), black_box(key));
                    black_box(out);
                })
            });

            g.bench_function(format!("{}/decrypt", $prefix), |b| {
                b.iter(|| {
                    let out = $decrypt(black_box(ct), black_box(key));
                    black_box(out);
                })
            });
        }
    };
}

use criterion::Criterion;
pub(crate) use define_cipher_bench;
use std::time::Duration;

pub(crate) fn criterion_config() -> Criterion {
    Criterion::default()
        .warm_up_time(Duration::from_secs(1))
        .measurement_time(Duration::from_secs(1))
        .sample_size(100)
        .nresamples(100_000)
        .confidence_level(0.95)
        .significance_level(0.05)
        .noise_threshold(0.02)
}
