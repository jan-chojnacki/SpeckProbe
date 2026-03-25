use crate::worker::Worker;
use crate::{Task, TaskResult};
use std::thread::JoinHandle;
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::SendError;

pub struct Orchestrator {
    rx: mpsc::Receiver<Task>,
    worker_txs: Vec<mpsc::Sender<Task>>,
    next_worker_idx: usize,
    _worker_handles: Vec<JoinHandle<()>>,
}

impl Orchestrator {
    pub fn new(
        worker_cpu_ids: Vec<usize>,
        rx: mpsc::Receiver<Task>,
        result_tx: mpsc::Sender<TaskResult>,
        worker_queue_capacity: usize,
    ) -> Self {
        assert!(
            !worker_cpu_ids.is_empty(),
            "at least one worker cpu_id is required"
        );

        let mut worker_txs = Vec::with_capacity(worker_cpu_ids.len());
        let mut worker_handles = Vec::with_capacity(worker_cpu_ids.len());

        for (worker_id, cpu_id) in worker_cpu_ids.into_iter().enumerate() {
            let (worker_tx, worker_rx) = mpsc::channel(worker_queue_capacity);
            let worker = Worker::new(worker_id, cpu_id, worker_rx, result_tx.clone());

            worker_txs.push(worker_tx);
            worker_handles.push(worker.spawn_pinned());
        }

        Self {
            rx,
            worker_txs,
            next_worker_idx: 0,
            _worker_handles: worker_handles,
        }
    }

    pub async fn run(&mut self) {
        while let Some(task) = self.rx.recv().await {
            if self.dispatch(task).await.is_err() {
                break;
            }
        }
    }

    async fn dispatch(&mut self, task: Task) -> Result<(), SendError<Task>> {
        while !self.worker_txs.is_empty() {
            let worker_idx = self.next_worker_idx % self.worker_txs.len();
            self.next_worker_idx = (worker_idx + 1) % self.worker_txs.len();

            match self.worker_txs[worker_idx].send(task).await {
                Ok(()) => return Ok(()),
                Err(err) => {
                    todo!() //TODO
                }
            }
        }

        Err(SendError(task))
    }
}
