//! Domain logic for running SPECK key-search operations, benchmarks, and related utilities.
//!
//! The main entry points are in [`ops`]. Configuration is loaded from TOML files via [`store`].
pub mod error;

pub use error::ProbeError;
