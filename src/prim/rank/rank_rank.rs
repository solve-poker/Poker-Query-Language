use super::highest_rank;

/// creates ranking of the form [ kkk00000 hhhhllll ]
#[must_use]
#[inline]
pub const fn mk_masks_rankrank(l: u16, h: u16) -> (u8, u8) {
    (highest_rank(h) << 4 | highest_rank(l), 0)
}

#[cfg(test)]
mod tests {
    use prim::rank::tests::to_u16;

    use super::*;
    use crate::*;

    #[derive(Clone, Copy)]
    struct LayoutRankIdxRankIdx(u16);

    impl LayoutRankIdxRankIdx {
        const fn rank_idx_h(self) -> usize {
            const RANK_IDX_BITS: u16 = 0b1111_0000;
            const OFFSET: usize = 4;
            ((self.0 & RANK_IDX_BITS) >> OFFSET) as usize
        }

        const fn rank_idx_l(self) -> usize {
            const RANK_IDX_BITS: u16 = 0b1111;
            (self.0 & RANK_IDX_BITS) as usize
        }
    }

    #[quickcheck]
    fn test_mk_masks_rankrank(l: Rank16, h: Rank16) -> TestResult {
        if h.is_empty() || l.is_empty() {
            return TestResult::discard();
        }

        let i = to_u16(mk_masks_rankrank, l, h);

        let obj = LayoutRankIdxRankIdx(i);

        TestResult::from_bool(
            obj.rank_idx_h() == h.max_rank().unwrap() as usize
                && obj.rank_idx_l() == l.max_rank().unwrap() as usize,
        )
    }
}
