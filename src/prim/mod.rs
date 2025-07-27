use std::mem::transmute;

pub mod eval;
mod integer;
pub mod math;
pub mod rank;

pub use integer::normalize_u64;
use math::combinatorics::{
    combination_of_2_ranks_to_index, combination_of_3_ranks_to_index,
};
