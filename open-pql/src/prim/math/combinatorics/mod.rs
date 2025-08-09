mod comb_ranks;
mod perm_range_cond_idx;

pub use comb_ranks::{
    combination_of_2_index_to_ranks, combination_of_2_ranks_to_index,
    combination_of_3_index_to_ranks, combination_of_3_ranks_to_index,
};
pub use perm_range_cond_idx::range_cond_indices;

#[inline]
const fn nc2(n: u8) -> u8 {
    n * (n - 1) / 2
}

// #[inline]
// pub fn ncr_usize(n: usize, r: usize) -> usize {
//     #[inline]
//     fn factorial(n: usize) -> usize {
//         (2..=n).fold(1, usize::wrapping_mul)
//     }
//
//     if n == 0 {
//         0
//     } else {
//         factorial(n) / factorial(n.saturating_sub(r)) / factorial(r)
//     }
// }
