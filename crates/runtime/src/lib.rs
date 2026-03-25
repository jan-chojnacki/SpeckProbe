use crate::worker::WorkerError;
use engine::api::request::SearchRangeRequest;
use engine::domain::key::Key;

mod orchestrator;
mod worker;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Task {
    pub id: u64,
    pub payload: SearchRangeRequest,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TaskResult {
    pub task_id: u64,
    pub worker_id: usize,
    pub value: Result<Vec<Key>, WorkerError>,
}
