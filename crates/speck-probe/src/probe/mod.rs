//! Domain logic for running SPECK key-search operations, benchmarks, and related utilities.
//!
//! The main entry points are in [`ops`]. Configuration is loaded from TOML files via [`store`].

pub mod backend;
pub mod config;
pub mod error;
pub mod ops;
pub mod record;
pub mod store;

pub use error::ProbeError;
