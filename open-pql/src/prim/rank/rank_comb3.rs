use super::{combination_of_3_ranks_to_index, highest_rank};

/// creates ranking of the form [ kkkrrrrc cccccccc ]
#[must_use]
#[inline]
pub const fn mk_masks_rankcomb3(l: u16, h: u16) -> (u8, u8) {
    let [idx_l, idx_h] = combination_of_3_ranks_to_index(l).to_le_bytes();

    (idx_l, highest_rank(h) << 1 | idx_h & 0b1)
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
    struct LayoutRankIdxComb3Idx(u16);

    impl LayoutRankIdxComb3Idx {
        const fn rank_idx(self) -> usize {
            const RANK_IDX_BITS: u16 = 0b0001_1110_0000_0000;
            const OFFSET: usize = 9;
            ((self.0 & RANK_IDX_BITS) >> OFFSET) as usize
        }

        const fn comb_idx(self) -> u16 {
            const COMB_IDX_BITS: u16 = 0b1_1111_1111;

            self.0 & COMB_IDX_BITS
        }

        fn comb_rank16(self) -> u16 {
            combination_of_3_index_to_ranks(self.comb_idx())
        }
    }

    #[quickcheck]
    fn test_mk_masks_rankcomb3(l: Rank16, h: Rank16) -> TestResult {
        if h.is_empty() || l.count() < 3 {
            return TestResult::discard();
        }

        let i = to_u16(mk_masks_rankcomb3, l, h);

        let obj = LayoutRankIdxComb3Idx(i);

        TestResult::from_bool(
            obj.rank_idx() == h.max_rank().unwrap() as usize
                && obj.comb_rank16() == retain_leading_3_bits(l.to_u16()),
        )
    }
}
