use super::{combination_of_2_ranks_to_index, combination_of_3_ranks_to_index};

mod comb2_rank;
mod rank1;
mod rank13;
mod rank_comb2;
mod rank_comb3;
mod rank_rank;

pub use comb2_rank::*;
pub use rank_comb2::*;
pub use rank_comb3::*;
pub use rank_rank::*;
pub use rank1::*;
pub use rank13::*;

/// returns highest rank index as u8
/// input must have 1 or more ranks
#[must_use]
#[inline]
const fn highest_rank(r16: u16) -> u8 {
    15 - r16.leading_zeros().to_le_bytes()[0]
}

#[cfg(test)]
pub mod tests {
    use crate::*;

    pub fn to_u16(f: fn(u16, u16) -> (u8, u8), l: Rank16, h: Rank16) -> u16 {
        let (lo, hi) = f(l.to_u16(), h.to_u16());

        u16::from_le_bytes((lo, hi).into())
    }
}
