use crate::api::{
    CipherConfig, DispatchError, DispatchOutput, InternalConfig, RuntimeConfig, RuntimeRequest,
    SearchSpace,
};
use crate::backend::dispatch::dispatch;
use crossbeam::channel::{Receiver, Sender, bounded};
use engine::domain::key::Key;

pub mod api;
pub mod backend;
pub mod domain;
pub mod orchestrator;

#[derive(Debug)]
pub struct TaskDone {}

pub struct Runtime {
    tx: Option<Sender<TaskDone>>,
    rx: Option<Receiver<TaskDone>>,
    cipher_config: Option<CipherConfig>,
    runtime_config: Option<RuntimeConfig>,
    search_space: Option<SearchSpace>,
}

impl Runtime {
    pub fn new(
        cipher_config: CipherConfig,
        runtime_config: RuntimeConfig,
        search_space: SearchSpace,
    ) -> Self {
        let cap = runtime_config.cap;
        let num_threads = runtime_config.num_threads;
        let (tx, rx) = bounded::<TaskDone>(cap * num_threads);

        Self {
            tx: Some(tx),
            rx: Some(rx),
            cipher_config: Some(cipher_config),
            runtime_config: Some(runtime_config),
            search_space: Some(search_space),
        }
    }

    pub fn get_rx_channel(&mut self) -> Receiver<TaskDone> {
        self.rx.take().expect("TODO")
    }

    pub fn run(&mut self) -> Result<DispatchOutput, DispatchError> {
        let cli_tx = self.tx.take().expect("run() can be called only once");
        let cipher_config = self
            .cipher_config
            .take()
            .expect("run() can be called only once");
        let runtime_config = self
            .runtime_config
            .take()
            .expect("run() can be called only once");
        let search_space = self
            .search_space
            .take()
            .expect("run() can be called only once");

        let internal_config = InternalConfig { cli_tx };

        let request = RuntimeRequest {
            cipher_config,
            runtime_config,
            search_space,
            internal_config,
        };

        dispatch(request)
    }
}
