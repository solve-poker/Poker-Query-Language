/// creates ranking of the form [ kkkrrrrr rrrrrrrr ]
#[allow(clippy::tuple_array_conversions)]
#[must_use]
#[inline]
pub const fn mk_masks_rank13(_l: u16, h: u16) -> (u8, u8) {
    let [lo, hi] = h.to_le_bytes();

    (lo, hi)
}

#[cfg(test)]
mod tests {
    use prim::rank::tests::to_u16;

    use super::*;
    use crate::*;

    struct LayoutRank13(u16);

    impl LayoutRank13 {
        const fn rank16(self) -> Rank16 {
            Rank16::from_u16(self.0)
        }
    }

    #[quickcheck]
    fn test_mk_masks_rank13(l: Rank16, h: Rank16) -> TestResult {
        if h.is_empty() {
            return TestResult::discard();
        }

        let i = to_u16(mk_masks_rank13, l, h);

        let obj = LayoutRank13(i);

        TestResult::from_bool(obj.rank16() == h)
    }
}
