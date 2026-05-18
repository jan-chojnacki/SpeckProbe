use criterion::Criterion;
use std::time::Duration;

pub fn criterion_slow_config() -> Criterion {
    Criterion::default()
        .warm_up_time(Duration::from_secs(3))
        .measurement_time(Duration::from_secs(5))
        .sample_size(30)
        .nresamples(100_000)
        .confidence_level(0.95)
        .significance_level(0.05)
        .noise_threshold(0.02)
        .without_plots()
}

pub fn criterion_fast_config() -> Criterion {
    Criterion::default()
        .warm_up_time(Duration::from_secs(3))
        .measurement_time(Duration::from_secs(5))
        .sample_size(100)
        .nresamples(100_000)
        .confidence_level(0.95)
        .significance_level(0.05)
        .noise_threshold(0.02)
        .without_plots()
}

#[macro_export]
macro_rules! define_cipher_bench {
    (
        $(#[$meta:meta])*
        $fn_name:ident,
        prefix = $prefix:literal,
        key = $key:expr,
        pt = $pt:expr,
        encrypt = $encrypt:path,
        encrypt_inflight = $encrypt_inflight:path,
        decrypt = $decrypt:path
    ) => {
        $(#[$meta])*
        fn $fn_name(g: &mut criterion::BenchmarkGroup<criterion::measurement::WallTime>) {
            let key = $key;
            let pt = $pt;
            let ct = $encrypt(pt, key);

            g.bench_function(criterion::BenchmarkId::new("encrypt", format!("{}", $prefix)), |b| {
                b.iter(|| {
                    let out = $encrypt(black_box(pt), black_box(key));
                    black_box(out);
                })
            });

            g.bench_function(criterion::BenchmarkId::new("encrypt_inflight", format!("{}", $prefix)), |b| {
                b.iter(|| {
                    let out = $encrypt_inflight(black_box(pt), black_box(key));
                    black_box(out);
                })
            });

            g.bench_function(criterion::BenchmarkId::new("decrypt", format!("{}", $prefix)), |b| {
                b.iter(|| {
                    let out = $decrypt(black_box(ct), black_box(key));
                    black_box(out);
                })
            });
        }
    };
}

#[macro_export]
macro_rules! calculate_end {
    ($n:expr) => {{
        match $n {
            0 => 0,
            1..=7 => (1u64 << ($n * 8)) - 1,
            _ => u64::MAX,
        }
    }};
}

#[macro_export]
macro_rules! define_engine_bench {
    (
        $(#[$meta:meta])*
        $fn_name:ident,
        bytes = $bytes:expr,
        word = $word:ty,
        prefix = $prefix:literal,
        encrypt = $encrypt:path,
        encrypt_inflight = $encrypt_inflight:path,
        decrypt = $decrypt:path
    ) => {
        $(#[$meta])*
        fn $fn_name(g: &mut criterion::BenchmarkGroup<criterion::measurement::WallTime>) {
            seq_macro::seq!(I in 1..=3{
                let end = $crate::calculate_end!(I);
                let zero: $word = unsafe { std::mem::zeroed() };
                let task = Task::<$word, { $bytes }, { $bytes - I }> {
                    prefix: [0; $bytes - I],
                    start: 0,
                    end,
                    data: [zero; 2],
                    expected: [zero; 2],
                };

                let mut out: Vec<Key<{ $bytes }, { $bytes - I }>> = Vec::new();

                g.throughput(Throughput::Elements(end.saturating_add(1)));

                g.bench_function(criterion::BenchmarkId::new("encrypt", format!("{}/{}", $prefix, I)), |b| {
                    b.iter(|| {
                        $encrypt(black_box(task), black_box(&mut out));
                        black_box(&out);
                    })
                });

                g.bench_function(criterion::BenchmarkId::new("encrypt_inflight", format!("{}/{}", $prefix, I)), |b| {
                    b.iter(|| {
                        $encrypt_inflight(black_box(task), black_box(&mut out));
                        black_box(&out);
                    })
                });

                g.bench_function(criterion::BenchmarkId::new("decrypt", format!("{}/{}", $prefix, I)), |b| {
                    b.iter(|| {
                        $decrypt(black_box(task), black_box(&mut out));
                        black_box(&out);
                    })
                });
            });
        }
    };
}
