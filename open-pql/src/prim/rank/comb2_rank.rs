use super::{combination_of_2_ranks_to_index, highest_rank};

/// creates ranking of the form [ kkk00ccc ccccrrrr ]
#[must_use]
#[inline]
pub const fn mk_masks_comb2rank(l: u16, h: u16) -> (u8, u8) {
    let mid = combination_of_2_ranks_to_index(h);

    (highest_rank(l) | mid << 4, mid >> 4)
}

#[cfg(test)]
mod tests {
    use super::{
        super::{
            super::{integer::*, math::combinatorics::*},
            tests::*,
        },
        *,
    };
    use crate::*;

    #[derive(Clone, Copy)]
    struct LayoutComb2IdxRankIdx(u16);

    impl LayoutComb2IdxRankIdx {
        const fn rank_idx(self) -> usize {
            const RANK_IDX_BITS: u16 = 0b1111;
            (self.0 & RANK_IDX_BITS) as usize
        }

        const fn comb_idx(self) -> u8 {
            const COMB_IDX_BITS: u16 = 0b0111_1111_0000;
            const OFFSET: usize = 4;

            ((self.0 & COMB_IDX_BITS) >> OFFSET) as u8
        }

        fn comb_rank16(self) -> u16 {
            combination_of_2_index_to_ranks(self.comb_idx())
        }
    }

    #[quickcheck]
    fn test_mk_masks_comb2rank(l: Rank16, h: Rank16) -> TestResult {
        if h.count() < 2 || l.is_empty() {
            return TestResult::discard();
        }

        let i = to_u16(mk_masks_comb2rank, l, h);

        let obj = LayoutComb2IdxRankIdx(i);

        TestResult::from_bool(
            obj.rank_idx() == l.max_rank().unwrap() as usize
                && obj.comb_rank16() == retain_leading_2_bits(h.to_u16()),
        )
    }
}
