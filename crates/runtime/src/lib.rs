use crate::api::{
    CipherConfig, DispatchError, DispatchOutput, InternalConfig, RuntimeConfig, RuntimeRequest,
    SearchSpace,
};
use crate::backend::dispatch::dispatch;
use crossbeam::channel::{Receiver, Sender, bounded};

pub mod api;
pub mod backend;
pub mod domain;
pub mod orchestrator;

pub const CAP: usize = 256;

#[derive(Debug)]
pub struct TaskDone {}

pub struct Runtime {
    tx: Option<Sender<TaskDone>>,
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
        Self {
            tx: None,
            cipher_config: Some(cipher_config),
            runtime_config: Some(runtime_config),
            search_space: Some(search_space),
        }
    }

    pub fn enable_progress(&mut self) -> Receiver<TaskDone> {
        let num_threads = self
            .runtime_config
            .as_ref()
            .expect("enable_progress() must be called before run()")
            .num_threads;
        let (tx, rx) = bounded::<TaskDone>(CAP * num_threads);
        self.tx = Some(tx);
        rx
    }

    pub fn run(&mut self) -> Result<DispatchOutput, DispatchError> {
        let cli_tx = self.tx.take();
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
