use super::highest_rank;

/// creates ranking of the form [ kkkrrrr0 00000000 ]
#[must_use]
#[inline]
pub const fn mk_masks_rank(_l: u16, h: u16) -> (u8, u8) {
    (0, highest_rank(h) << 1)
}

#[cfg(test)]
mod tests {
    use prim::rank::tests::to_u16;

    use super::*;
    use crate::*;

    struct LayoutRankIdx(u16);

    impl LayoutRankIdx {
        const fn rank_idx(self) -> usize {
            const RANK_IDX_BITS: u16 = 0b0001_1110_0000_0000;
            const OFFSET: usize = 9;
            ((self.0 & RANK_IDX_BITS) >> OFFSET) as usize
        }
    }

    #[quickcheck]
    fn test_mk_masks_rank(l: Rank16, h: Rank16) -> TestResult {
        if h.is_empty() {
            return TestResult::discard();
        }

        let i = to_u16(mk_masks_rank, l, h);

        let obj = LayoutRankIdx(i);

        TestResult::from_bool(obj.rank_idx() == h.max_rank().unwrap() as usize)
    }
}
