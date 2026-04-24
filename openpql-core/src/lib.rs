//! Core PQL value types and the built-in functions that operate on them.

#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#![cfg_attr(test, allow(clippy::needless_pass_by_value))]

/// Built-in PQL functions available to queries.
pub mod functions;
mod types;
mod util;

pub use types::*;
