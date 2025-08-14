use super::{Display, Hash, Suit};

#[derive(
    Copy, Clone, PartialEq, Eq, Debug, Ord, PartialOrd, Hash, Display, Default,
)]
pub struct SuitIdx(u8);

#[allow(unused)]
impl SuitIdx {
    /// Creates a new `SuitIdx` from a u8 value
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
    pub(crate) fn to_suit(self) -> Suit {
        Suit::from_u8(self.0)
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_suit_idx_new() {
        let suit_idx = SuitIdx::new(2);
        assert_eq!(suit_idx.0, 2);
    }

    #[test]
    fn test_suit_idx_to_u8() {
        let suit_idx = SuitIdx::new(1);
        assert_eq!(suit_idx.to_u8(), 1);
    }

    #[test]
    fn test_suit_idx_to_usize() {
        let suit_idx = SuitIdx::new(3);
        assert_eq!(suit_idx.to_usize(), 3);
    }

    #[test]
    fn test_suit_idx_to_suit() {
        let suit_idx = SuitIdx::new(0);
        let suit = suit_idx.to_suit();
        assert_eq!(suit, Suit::from_u8(0));
    }

    #[test]
    fn test_suit_idx_default() {
        let suit_idx = SuitIdx::default();
        assert_eq!(suit_idx.to_u8(), 0);
    }

    #[test]
    fn test_suit_idx_ordering() {
        let suit_idx1 = SuitIdx::new(0);
        let suit_idx2 = SuitIdx::new(1);
        let suit_idx3 = SuitIdx::new(2);

        assert!(suit_idx1 < suit_idx2);
        assert!(suit_idx2 < suit_idx3);
        assert!(suit_idx1 < suit_idx3);
    }
}
