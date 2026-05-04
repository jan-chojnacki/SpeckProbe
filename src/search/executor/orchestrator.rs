use crate::search::domain::key::Key;
use crate::search::domain::task::Task;
use crate::search::domain::task_producer::TaskProducer;
use crate::search::executor::TaskDone;
use crate::search::executor::word::{EngineWord, ValidatorWord};
use crossbeam::channel::{Receiver, Sender, bounded};
use rayon::iter::{ParallelBridge, ParallelIterator};
use rayon::{ThreadPool, ThreadPoolBuilder};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::JoinHandle;

pub struct Orchestrator<
    FS,
    FV,
    EW: EngineWord,
    VW: ValidatorWord,
    const BYTES: usize,
    const PREFIX: usize,
> where
    FS: Fn(Task<EW, BYTES, PREFIX>, &mut Vec<Key<BYTES, PREFIX>>) + Sync,
    FV: Fn(&[[VW; 2]], &[[VW; 2]], &Key<BYTES, PREFIX>) -> bool + Send + Copy + 'static,
{
    task_producer: Option<TaskProducer<EW, BYTES, PREFIX>>,
    pool: ThreadPool,
    cli_tx: Option<Sender<TaskDone>>,
    tx: Option<Sender<Vec<Key<BYTES, PREFIX>>>>,
    rx: Receiver<Vec<Key<BYTES, PREFIX>>>,
    stop: Arc<AtomicBool>,
    data: Vec<[VW; 2]>,
    expected: Vec<[VW; 2]>,
    function: FS,
    validator: FV,
}

impl<FS, FV, EW: EngineWord, VW: ValidatorWord, const BYTES: usize, const PREFIX: usize>
    Orchestrator<FS, FV, EW, VW, BYTES, PREFIX>
where
    FS: Fn(Task<EW, BYTES, PREFIX>, &mut Vec<Key<BYTES, PREFIX>>) + Sync,
    FV: Fn(&[[VW; 2]], &[[VW; 2]], &Key<BYTES, PREFIX>) -> bool + Send + Copy + 'static,
{
    pub fn new(
        start: [u8; PREFIX],
        end: [u8; PREFIX],
        data: &[[VW; 2]],
        expected: &[[VW; 2]],
        num_threads: usize,
        cap: usize,
        cli_tx: Option<Sender<TaskDone>>,
        function: FS,
        validator: FV,
        convert: impl Fn([VW; 2]) -> [EW; 2],
    ) -> Self {
        assert!(!data.is_empty());
        assert_eq!(data.len(), expected.len());

        let pool = ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .thread_name(|i| format!("my-pool-{i}"))
            .build()
            .unwrap();

        let (tx, rx) = bounded::<Vec<Key<BYTES, PREFIX>>>(cap);
        let stop = Arc::new(AtomicBool::new(false));

        let task_producer = TaskProducer::<EW, BYTES, PREFIX>::new(
            start,
            end,
            convert(data[0]),
            convert(expected[0]),
        );

        Self {
            task_producer: Some(task_producer),
            pool,
            cli_tx,
            tx: Some(tx),
            rx,
            stop,
            data: data.to_vec(),
            expected: expected.to_vec(),
            function,
            validator,
        }
    }

    pub fn run(&mut self) -> (Vec<Key<BYTES, PREFIX>>, Option<Key<BYTES, PREFIX>>) {
        let validator = self.spawn_validator();

        let tx = self.tx.take().expect("run() can be called only once");
        let cli_tx = self.cli_tx.take();

        self.run_pool(&tx, cli_tx);
        drop(tx);

        validator.join().unwrap()
    }

    fn spawn_validator(&self) -> JoinHandle<(Vec<Key<BYTES, PREFIX>>, Option<Key<BYTES, PREFIX>>)> {
        let rx = self.rx.clone();
        let stop = Arc::clone(&self.stop);
        let data = self.data.clone();
        let expected = self.expected.clone();
        let validator = self.validator;

        thread::spawn(move || {
            let mut global_results = Vec::<Key<BYTES, PREFIX>>::new();

            for hit in rx.into_iter().flatten() {
                let ok = Self::validate(&validator, &data, &expected, &hit);

                if ok {
                    stop.store(true, Ordering::Relaxed);
                    return (global_results, Some(hit));
                }

                global_results.push(hit);
            }

            (global_results, None)
        })
    }

    fn validate(
        validator: &FV,
        data: &[[VW; 2]],
        expected: &[[VW; 2]],
        hit: &Key<BYTES, PREFIX>,
    ) -> bool {
        validator(data, expected, hit)
    }

    fn run_pool(&mut self, tx: &Sender<Vec<Key<BYTES, PREFIX>>>, cli_tx: Option<Sender<TaskDone>>) {
        let task_producer = self
            .task_producer
            .take()
            .expect("run() can be called only once");
        let stop = Arc::clone(&self.stop);
        let function = &self.function;

        self.pool.install(|| {
            let _ = task_producer.par_bridge().try_for_each_init(
                || Vec::<Key<BYTES, PREFIX>>::with_capacity(4),
                move |out, task| -> Result<(), ()> {
                    if stop.load(Ordering::Relaxed) {
                        return Err(());
                    }

                    out.clear();
                    function(task, out);

                    if let Some(ref tx) = cli_tx {
                        tx.send(TaskDone {}).expect("progress channel closed");
                    }
                    if out.is_empty() {
                        return Ok(());
                    }

                    tx.send(std::mem::take(out)).map_err(|_| ())
                },
            );
        });
    }
}
