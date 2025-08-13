use super::{Display, Hash, Rank};

#[derive(
    Copy, Clone, PartialEq, Eq, Debug, Ord, PartialOrd, Hash, Display, Default,
)]
pub struct RankIdx(u8);

#[allow(unused)]
impl RankIdx {
    /// Creates a new `RankIdx` from a u8 value
    pub(crate) const fn new(value: u8) -> Self {
        Self(value)
    }

    /// Converts to a u8 value
    pub(crate) const fn to_u8(self) -> u8 {
        self.0
    }

    /// Converts to a usize value
    pub(crate) const fn to_usize(self) -> usize {
        self.0 as usize
    }

    /// Converts to a Card
    pub(crate) fn to_rank(self) -> Rank {
        Rank::from_u8(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_rank_idx_new() {
        let rank_idx = RankIdx::new(5);
        assert_eq!(rank_idx.0, 5);
    }

    #[test]
    fn test_rank_idx_to_u8() {
        let rank_idx = RankIdx::new(7);
        assert_eq!(rank_idx.to_u8(), 7);
    }

    #[test]
    fn test_rank_idx_to_usize() {
        let rank_idx = RankIdx::new(3);
        assert_eq!(rank_idx.to_usize(), 3);
    }

    #[test]
    fn test_rank_idx_to_rank() {
        let rank_idx = RankIdx::new(12);
        let rank = rank_idx.to_rank();
        assert_eq!(rank, Rank::from_u8(12));
    }

    #[test]
    fn test_rank_idx_default() {
        let rank_idx = RankIdx::default();
        assert_eq!(rank_idx.to_u8(), 0);
    }

    #[test]
    fn test_rank_idx_ordering() {
        let rank_idx1 = RankIdx::new(3);
        let rank_idx2 = RankIdx::new(5);
        let rank_idx3 = RankIdx::new(7);

        assert!(rank_idx1 < rank_idx2);
        assert!(rank_idx2 < rank_idx3);
        assert!(rank_idx1 < rank_idx3);
    }
}
