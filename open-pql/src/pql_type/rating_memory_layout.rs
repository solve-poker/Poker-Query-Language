use super::*;

/// Memory Layout of the last 13bits of i16
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RatingMemoryLayout {
    Rank,      // [ kkkrrrr0 00000000 ]
    RankRank,  // [ kkk00000 hhhhllll ]
    RankComb2, // [ kkk0rrrr 0ccccccc ]
    Comb2Rank, // [ kkk00ccc ccccrrrr ]
    RankComb3, // [ kkkrrrrc cccccccc ]
    Rank13,    // [ kkkrrrrr rrrrrrrr ]
}

impl RatingMemoryLayout {
    pub fn masks_to_ranks(self, l: u8, h: u8) -> (u16, u16) {
        match self {
            Self::Rank => r16_rank(l, h),
            Self::RankRank => r16_rankrank(l, h),
            Self::RankComb2 => r16_rankcomb2(l, h),
            Self::Comb2Rank => r16_comb2rank(l, h),
            Self::RankComb3 => r16_rankcomb3(l, h),
            Self::Rank13 => (0, u16::from_le_bytes([l, h & 0b0001_1111])),
        }
    }
}

const fn rank_to_r16(v: u8) -> u16 {
    1 << (v & 0b1111)
}

const fn r16_rank(_lo: u8, hi: u8) -> (u16, u16) {
    (0, rank_to_r16(hi >> 1))
}

const fn r16_rankrank(lo: u8, _hi: u8) -> (u16, u16) {
    (rank_to_r16(lo), rank_to_r16(lo >> 4))
}

fn r16_rankcomb2(lo: u8, hi: u8) -> (u16, u16) {
    (combination_of_2_index_to_ranks(lo), rank_to_r16(hi))
}

fn r16_comb2rank(lo: u8, hi: u8) -> (u16, u16) {
    let idx = lo >> 4 | (hi & 0b111) << 4;
    (rank_to_r16(lo), combination_of_2_index_to_ranks(idx))
}

fn r16_rankcomb3(lo: u8, hi: u8) -> (u16, u16) {
    let idx = u16::from_le_bytes([lo, hi & 0b1]);
    (combination_of_3_index_to_ranks(idx), rank_to_r16(hi >> 1))
}

#[cfg(test)]
mod tests {
    use prim::rank::*;

    use self::RatingMemoryLayout as ML;
    use super::*;

    #[cfg(test)]
    impl RatingMemoryLayout {
        pub const fn ranks_to_masks(self, l: u16, h: u16) -> (u8, u8) {
            match self {
                Self::Rank => mk_masks_rank(l, h),
                Self::RankRank => mk_masks_rankrank(l, h),
                Self::RankComb2 => mk_masks_rankcomb2(l, h),
                Self::Comb2Rank => mk_masks_comb2rank(l, h),
                Self::RankComb3 => mk_masks_rankcomb3(l, h),
                Self::Rank13 => mk_masks_rank13(l, h),
            }
        }
    }

    const fn msb(v: u16) -> u16 {
        let n = v.leading_zeros();

        if n == 16 { 0 } else { 1 << (15 - n) }
    }

    const fn msb2(v: u16) -> u16 {
        let r1 = msb(v);

        r1 | msb(v & !r1)
    }

    const fn msb3(v: u16) -> u16 {
        let r1 = msb(v);

        r1 | msb2(v & !r1)
    }

    #[quickcheck]
    fn test_rank(h: Rank16) -> TestResult {
        if h.count() < 1 {
            return TestResult::discard();
        }

        let flags = ML::Rank.ranks_to_masks(0, h.to_u16());

        let (_low, high) = ML::Rank.masks_to_ranks(flags.0, flags.1);

        TestResult::from_bool(high == msb(h.to_u16()))
    }

    #[quickcheck]
    fn test_rankrank(h: Rank16, l: Rank16) -> TestResult {
        if h.count() < 1 || l.count() < 1 {
            return TestResult::discard();
        }

        let flags = ML::RankRank.ranks_to_masks(l.to_u16(), h.to_u16());

        let (low, high) = ML::RankRank.masks_to_ranks(flags.0, flags.1);

        TestResult::from_bool(low == msb(l.to_u16()) && high == msb(h.to_u16()))
    }

    #[quickcheck]
    fn test_rankcomb2(h: Rank16, l: Rank16) -> TestResult {
        if h.count() < 1 || l.count() < 2 {
            return TestResult::discard();
        }

        let flags = ML::RankComb2.ranks_to_masks(l.to_u16(), h.to_u16());

        let (low, high) = ML::RankComb2.masks_to_ranks(flags.0, flags.1);

        TestResult::from_bool(
            low == msb2(l.to_u16()) && high == msb(h.to_u16()),
        )
    }

    #[quickcheck]
    fn test_comb2rank(h: Rank16, l: Rank16) -> TestResult {
        if h.count() < 2 || l.count() < 1 {
            return TestResult::discard();
        }

        let flags = ML::Comb2Rank.ranks_to_masks(l.to_u16(), h.to_u16());

        let (low, high) = ML::Comb2Rank.masks_to_ranks(flags.0, flags.1);

        TestResult::from_bool(
            low == msb(l.to_u16()) && high == msb2(h.to_u16()),
        )
    }

    #[quickcheck]
    fn test_rankcomb3(h: Rank16, l: Rank16) -> TestResult {
        if h.count() < 1 || l.count() < 3 {
            return TestResult::discard();
        }

        let flags = ML::RankComb3.ranks_to_masks(l.to_u16(), h.to_u16());

        let (low, high) = ML::RankComb3.masks_to_ranks(flags.0, flags.1);

        TestResult::from_bool(
            low == msb3(l.to_u16()) && high == msb(h.to_u16()),
        )
    }

    #[quickcheck]
    fn test_rank13(h: Rank16) {
        let flags = ML::Rank13.ranks_to_masks(0, h.to_u16());

        let (_low, high) = ML::Rank13.masks_to_ranks(flags.0, flags.1);

        assert_eq!(high, h.to_u16());
    }
}
