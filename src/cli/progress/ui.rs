use crate::cli::progress::scale::ProgressScale;
use crate::search::executor::TaskDone;
use crossbeam::channel::Receiver;
use human_format::Formatter;
use indicatif::{ProgressBar, ProgressStyle};
use primitive_types::U256;
use std::ops::Add;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

pub struct ProgressUi {
    reader: JoinHandle<()>,
    renderer: JoinHandle<()>,
}

impl ProgressUi {
    pub fn start(rx: Receiver<TaskDone>, start: &[u8], end: &[u8], suffix: usize) -> Self {
        let (scale, pb_len) = ProgressScale::from_range(start, end);
        let pb = build_progress_bar(pb_len);

        let delta = Arc::new(AtomicU64::new(0));
        let done = Arc::new(AtomicBool::new(false));

        let tasks_per_iter = u64::MAX >> (64 - suffix * 8);

        let reader = spawn_reader(rx, Arc::clone(&delta), Arc::clone(&done));
        let renderer = spawn_renderer(pb, scale, delta, done, tasks_per_iter);

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
    tasks_per_iter: u64,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut f = Formatter::new();
        f.with_separator("").with_units(" keys/s");

        let mut t_last = Instant::now();
        let mut accumulated = U256::zero();
        let mut smoothed_rate: f64 = 0.0;

        const ALPHA: f64 = 0.10;

        let mut tick = || {
            let t_now = Instant::now();
            let chunk = delta.swap(0, Ordering::Relaxed);
            accumulated += U256::from(chunk);

            let duration = t_now.duration_since(t_last).as_secs_f64().add(f64::EPSILON);
            let instant_rate = (chunk as f64 / duration) * tasks_per_iter as f64;

            smoothed_rate = if smoothed_rate == 0.0 {
                instant_rate
            } else {
                ALPHA * instant_rate + (1.0 - ALPHA) * smoothed_rate
            };

            pb.set_message(f.format(smoothed_rate));
            pb.set_position(scale.scale(accumulated));
            t_last = t_now;
        };
        while !done.load(Ordering::Relaxed) {
            tick();
            thread::sleep(Duration::from_millis(100));
        }
        tick();
        pb.finish();
    })
}

fn build_progress_bar_inner(len: u64, bar_width: usize, show_msg: bool) -> ProgressBar {
    let template = if show_msg {
        format!(
            "{{spinner:.yellow}} [{{elapsed_precise}}] [{{bar:{bar_width}.cyan/blue}}] {{percent}}% | {{msg}} ({{eta}})"
        )
    } else {
        format!(
            "{{spinner:.yellow}} [{{elapsed_precise}}] [{{bar:{bar_width}.cyan/blue}}] {{percent}}% ({{eta}})"
        )
    };
    let pb = ProgressBar::new(len);
    let style = ProgressStyle::with_template(&template)
        .expect("invalid progress bar template")
        .progress_chars("=>-");
    pb.set_style(style);
    pb
}

pub fn build_progress_bar(len: u64) -> ProgressBar {
    build_progress_bar_inner(len, 36, true)
}
