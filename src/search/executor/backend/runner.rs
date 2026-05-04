use crate::search::domain::key::Key;
use crate::search::domain::task::Task;
use crate::search::executor::orchestrator::Orchestrator;
use crate::search::executor::word::{EngineWord, ValidatorWord};
use crate::search::executor::{CAP, DispatchOutput, RuntimeRequest};

pub(crate) fn run_orchestrator<EW, VW, const BYTES: usize, const PREFIX: usize, FS, FV>(
    request: RuntimeRequest,
    search_fn: FS,
    validate_fn: FV,
    convert_fn: impl Fn([VW; 2]) -> [EW; 2],
) -> DispatchOutput
where
    EW: EngineWord,
    VW: ValidatorWord,
    FS: Fn(Task<EW, BYTES, PREFIX>, &mut Vec<Key<BYTES, PREFIX>>) + Sync,
    FV: Fn(&[[VW; 2]], &[[VW; 2]], &Key<BYTES, PREFIX>) -> bool + Send + Copy + 'static,
{
    let start: [u8; PREFIX] = request
        .search_space
        .start
        .try_into()
        .expect("start length mismatch");
    let end: [u8; PREFIX] = request
        .search_space
        .end
        .try_into()
        .expect("end length mismatch");

    let data: Vec<[VW; 2]> = request
        .search_space
        .data
        .iter()
        .map(|[a, b]| [VW::from_u64(*a), VW::from_u64(*b)])
        .collect();
    let expected: Vec<[VW; 2]> = request
        .search_space
        .expected
        .iter()
        .map(|[a, b]| [VW::from_u64(*a), VW::from_u64(*b)])
        .collect();

    let mut runtime = Orchestrator::<FS, FV, EW, VW, BYTES, PREFIX>::new(
        start,
        end,
        &data,
        &expected,
        request.runtime_config.num_threads,
        CAP,
        request.internal_config.cli_tx,
        search_fn,
        validate_fn,
        convert_fn,
    );

    let (keys, found) = runtime.run();

    (
        keys.into_iter().map(|k| k.to_vec()).collect(),
        found.map(|k| k.to_vec()),
    )
}
