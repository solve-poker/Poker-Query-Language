use super::{Display, Hash, Idx, Rank};

/// Rank index representation.
///
/// Converts ranks to numeric indices (0-12).
#[derive(
    Copy, Clone, PartialEq, Eq, Debug, Ord, PartialOrd, Hash, Display, Default,
)]
pub struct RankIdx(pub(crate) Idx);

impl RankIdx {
    pub const fn to_rank(self) -> Option<Rank> {
        match self.0 {
            0 => Some(Rank::R2),
            1 => Some(Rank::R3),
            2 => Some(Rank::R4),
            3 => Some(Rank::R5),
            4 => Some(Rank::R6),
            5 => Some(Rank::R7),
            6 => Some(Rank::R8),
            7 => Some(Rank::R9),
            8 => Some(Rank::RT),
            9 => Some(Rank::RJ),
            10 => Some(Rank::RQ),
            11 => Some(Rank::RK),
            12 => Some(Rank::RA),
            _ => None,
        }
    }
}

impl From<Rank> for RankIdx {
    fn from(rank: Rank) -> Self {
        Self(rank as Idx)
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;

    #[test]
    fn test_from_rank() {
        assert_eq!(RankIdx::from(Rank::R2).0, 0);
        assert_eq!(RankIdx::from(Rank::R3).0, 1);
        assert_eq!(RankIdx::from(Rank::R4).0, 2);
        assert_eq!(RankIdx::from(Rank::R5).0, 3);
        assert_eq!(RankIdx::from(Rank::R6).0, 4);
        assert_eq!(RankIdx::from(Rank::R7).0, 5);
        assert_eq!(RankIdx::from(Rank::R8).0, 6);
        assert_eq!(RankIdx::from(Rank::R9).0, 7);
        assert_eq!(RankIdx::from(Rank::RT).0, 8);
        assert_eq!(RankIdx::from(Rank::RJ).0, 9);
        assert_eq!(RankIdx::from(Rank::RQ).0, 10);
        assert_eq!(RankIdx::from(Rank::RK).0, 11);
        assert_eq!(RankIdx::from(Rank::RA).0, 12);
    }

    #[test]
    fn test_to_rank() {
        assert_eq!(RankIdx(0).to_rank(), Some(Rank::R2));
        assert_eq!(RankIdx(1).to_rank(), Some(Rank::R3));
        assert_eq!(RankIdx(2).to_rank(), Some(Rank::R4));
        assert_eq!(RankIdx(3).to_rank(), Some(Rank::R5));
        assert_eq!(RankIdx(4).to_rank(), Some(Rank::R6));
        assert_eq!(RankIdx(5).to_rank(), Some(Rank::R7));
        assert_eq!(RankIdx(6).to_rank(), Some(Rank::R8));
        assert_eq!(RankIdx(7).to_rank(), Some(Rank::R9));
        assert_eq!(RankIdx(8).to_rank(), Some(Rank::RT));
        assert_eq!(RankIdx(9).to_rank(), Some(Rank::RJ));
        assert_eq!(RankIdx(10).to_rank(), Some(Rank::RQ));
        assert_eq!(RankIdx(11).to_rank(), Some(Rank::RK));
        assert_eq!(RankIdx(12).to_rank(), Some(Rank::RA));

        assert!(RankIdx(13).to_rank().is_none());
        assert!(RankIdx(-1).to_rank().is_none());
    }
}
