use super::{combination_of_2_ranks_to_index, highest_rank};

/// creates ranking of the form [ kkk0rrrr 0ccccccc ]
#[must_use]
#[inline]
pub const fn mk_masks_rankcomb2(l: u16, h: u16) -> (u8, u8) {
    (combination_of_2_ranks_to_index(l), highest_rank(h))
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
    struct LayoutRankIdxComb2Idx(u16);

    impl LayoutRankIdxComb2Idx {
        const fn rank_idx(self) -> usize {
            const RANK_IDX_BITS: u16 = 0b0000_1111_0000_0000;
            const OFFSET: usize = 8;
            ((self.0 & RANK_IDX_BITS) >> OFFSET) as usize
        }

        const fn comb_idx(self) -> u8 {
            const COMB_IDX_BITS: u16 = 0b0111_1111;

            (self.0 & COMB_IDX_BITS) as u8
        }

        fn comb_rank16(self) -> u16 {
            combination_of_2_index_to_ranks(self.comb_idx())
        }
    }

    #[quickcheck]
    fn test_mk_masks_rankcomb2(l: Rank16, h: Rank16) -> TestResult {
        if h.is_empty() || l.count() < 2 {
            return TestResult::discard();
        }

        let i = to_u16(mk_masks_rankcomb2, l, h);

        let obj = LayoutRankIdxComb2Idx(i);

        TestResult::from_bool(
            obj.rank_idx() == h.max_rank().unwrap() as usize
                && obj.comb_rank16() == retain_leading_2_bits(l.to_u16()),
        )
    }
}
