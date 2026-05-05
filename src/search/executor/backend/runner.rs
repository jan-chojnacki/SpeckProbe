use crate::search::domain::key::Key;
use crate::search::domain::task::Task;
use crate::search::executor::orchestrator::Orchestrator;
use crate::search::executor::word::{EngineWord, ValidatorWord};
use crate::search::executor::{CAP, CipherFunction, CipherMode, DispatchOutput, RuntimeRequest};

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

    let mut data: Vec<[VW; 2]> = request
        .search_space
        .data
        .iter()
        .map(|[a, b]| [VW::from_u64(*a), VW::from_u64(*b)])
        .collect();
    let mut expected: Vec<[VW; 2]> = request
        .search_space
        .expected
        .iter()
        .map(|[a, b]| [VW::from_u64(*a), VW::from_u64(*b)])
        .collect();

    if request.cipher_config.cipher_mode == CipherMode::Cbc {
        let iv_raw = request.search_space.iv.expect("CBC requires IV");
        let iv = [VW::from_u64(iv_raw[0]), VW::from_u64(iv_raw[1])];
        match request.cipher_config.cipher_function {
            CipherFunction::Encrypt | CipherFunction::EncryptInflight => {
                data[0] = [data[0][0] ^ iv[0], data[0][1] ^ iv[1]];
            }
            CipherFunction::Decrypt => {
                expected[0] = [expected[0][0] ^ iv[0], expected[0][1] ^ iv[1]];
            }
        }
    }

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
