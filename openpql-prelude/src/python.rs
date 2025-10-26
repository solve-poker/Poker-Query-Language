#![cfg_attr(coverage_nightly, coverage(off))]

extern crate pyo3;

pub use pyo3::{exceptions::PyValueError, prelude::*};

#[pyo3::pymodule]
mod opql_prelude {
    #[pymodule_export]
    use super::super::Rank;
}
