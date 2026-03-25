use crate::{Task, TaskResult};
use engine::domain::key::Key;
use engine::{SearchEngineBackendError, search_range};
use std::thread;
use thiserror::Error;
use tokio::runtime::Builder;
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::SendError;

pub struct Worker {
    id: usize,
    cpu_id: usize,
    rx: mpsc::Receiver<Task>,
    result_tx: mpsc::Sender<TaskResult>,
}

#[derive(Debug, Clone, Error, Eq, PartialEq)]
pub enum WorkerError {
    #[error(transparent)]
    SearchEngineError(#[from] SearchEngineBackendError),
}

impl Worker {
    pub fn new(
        id: usize,
        cpu_id: usize,
        rx: mpsc::Receiver<Task>,
        result_tx: mpsc::Sender<TaskResult>,
    ) -> Self {
        Self {
            id,
            cpu_id,
            rx,
            result_tx,
        }
    }

    pub fn spawn_pinned(mut self) -> thread::JoinHandle<()> {
        thread::Builder::new()
            .name(format!("worker-{}", self.id))
            .spawn(move || {
                pin_current_thread_to_cpu(self.cpu_id)
                    .unwrap_or_else(|e| panic!("failed to pin worker {}: {}", self.id, e));

                let rt = Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .expect("failed to build tokio current-thread runtime");

                rt.block_on(self.run());
            })
            .expect("failed to spawn worker thread")
    }

    async fn run(&mut self) {
        while let Some(task) = self.rx.recv().await {
            let value = search_range(&task.payload).map_err(WorkerError::from);

            if self.return_results(&task, value).await.is_err() {
                break;
            }
        }
    }

    async fn return_results(
        &mut self,
        task: &Task,
        value: Result<Vec<Key>, WorkerError>,
    ) -> Result<(), SendError<TaskResult>> {
        self.result_tx
            .send(TaskResult {
                task_id: task.id,
                worker_id: self.id,
                value,
            })
            .await
    }
}

fn pin_current_thread_to_cpu(cpu_id: usize) -> Result<(), String> {
    let cores = core_affinity::get_core_ids().ok_or("cannot read core IDs")?;
    let core = cores
        .into_iter()
        .find(|c| c.id == cpu_id)
        .ok_or_else(|| format!("cpu_id {} not available", cpu_id))?;

    if core_affinity::set_for_current(core) {
        Ok(())
    } else {
        Err(format!("set_for_current failed for cpu_id {}", cpu_id))
    }
}
