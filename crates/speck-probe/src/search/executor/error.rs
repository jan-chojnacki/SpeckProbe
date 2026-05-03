use crate::search::executor::CipherMode;
use speck::SpeckVersion;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DispatchError {
    UnsupportedSuffix {
        suffix: usize,
    },
    UnsupportedMode {
        mode: CipherMode,
    },
    UnsupportedCombination {
        version: SpeckVersion,
        mode: CipherMode,
        suffix: usize,
    },
}
