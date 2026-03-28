use engine::domain::key::Key;
use engine::domain::task_producer::TaskProducer;
use rayon::ThreadPoolBuilder;
use rayon::prelude::*;
use std::time::Instant;

fn main() {
    let start = [0, 0, 0, 0, 0, 0];
    let end = [255, 0, 0, 0, 0, 0];
    let data = [0, 0];
    let expected = [0, 0];

    let producer = TaskProducer::<u16, 8, 6>::new(start, end, data, expected);

    let pool = ThreadPoolBuilder::new()
        .num_threads(16)
        .thread_name(|i| format!("my-pool-{i}"))
        .build()
        .unwrap();

    let start = Instant::now();

    pool.install(|| {
        producer.par_bridge().into_par_iter().for_each(|i| {
            let mut out: Vec<Key<8, 6>> = Vec::new();
            engine::backend::search::search_decrypt_32_64(i, &mut out);
        });
    });

    let elapsed = start.elapsed();
    println!("total: {:?}", elapsed);
}
