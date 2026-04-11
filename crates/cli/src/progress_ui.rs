use crate::helpers::build_progress_bar;
use crate::progress_scale::ProgressScale;
use crossbeam::channel::Receiver;
use indicatif::ProgressBar;
use primitive_types::U256;
use runtime::TaskDone;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

pub struct ProgressUi {
    reader: JoinHandle<()>,
    renderer: JoinHandle<()>,
}

impl ProgressUi {
    pub fn start(rx: Receiver<TaskDone>, start: &[u8], end: &[u8]) -> Self {
        let (scale, pb_len) = ProgressScale::from_range(start, end);
        let pb = build_progress_bar(pb_len);

        let delta = Arc::new(AtomicU64::new(0));
        let done = Arc::new(AtomicBool::new(false));

        let reader = spawn_reader(rx, Arc::clone(&delta), Arc::clone(&done));
        let renderer = spawn_renderer(pb, scale, delta, done);

        Self { reader, renderer }
    }

    pub fn join(self) {
        self.reader.join().unwrap();
        self.renderer.join().unwrap();
    }
}

fn spawn_reader(
    rx: Receiver<TaskDone>,
    delta: Arc<AtomicU64>,
    done: Arc<AtomicBool>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        for _ in rx.iter() {
            delta.fetch_add(1, Ordering::Relaxed);
        }
        done.store(true, Ordering::Relaxed);
    })
}

fn spawn_renderer(
    pb: ProgressBar,
    scale: ProgressScale,
    delta: Arc<AtomicU64>,
    done: Arc<AtomicBool>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut accumulated = U256::zero();
        let mut tick = || {
            let chunk = delta.swap(0, Ordering::Relaxed);
            accumulated += U256::from(chunk);
            pb.set_position(scale.scale(accumulated));
        };
        while !done.load(Ordering::Relaxed) {
            tick();
            thread::sleep(Duration::from_millis(100));
        }
        tick(); // final flush
        pb.finish();
    })
}
