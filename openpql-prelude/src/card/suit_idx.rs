use super::{Display, Hash, Idx, Suit};

/// Suit index representation.
///
/// Converts suits to numeric indices (0-3).
#[derive(
    Copy, Clone, PartialEq, Eq, Debug, Ord, PartialOrd, Hash, Display, Default,
)]
pub struct SuitIdx(pub(crate) Idx);

impl SuitIdx {
    pub const fn to_suit(self) -> Option<Suit> {
        match self.0 {
            0 => Some(Suit::S),
            1 => Some(Suit::H),
            2 => Some(Suit::D),
            3 => Some(Suit::C),
            _ => None,
        }
    }
}

impl From<Suit> for SuitIdx {
    fn from(suit: Suit) -> Self {
        Self(suit as Idx)
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;

    #[test]
    fn test_from_suit() {
        assert_eq!(SuitIdx::from(Suit::S).0, 0);
        assert_eq!(SuitIdx::from(Suit::H).0, 1);
        assert_eq!(SuitIdx::from(Suit::D).0, 2);
        assert_eq!(SuitIdx::from(Suit::C).0, 3);
    }

    #[test]
    fn test_to_suit() {
        assert_eq!(SuitIdx(0).to_suit(), Some(Suit::S));
        assert_eq!(SuitIdx(1).to_suit(), Some(Suit::H));
        assert_eq!(SuitIdx(2).to_suit(), Some(Suit::D));
        assert_eq!(SuitIdx(3).to_suit(), Some(Suit::C));

        assert_eq!(SuitIdx(-1).to_suit(), None);
        assert_eq!(SuitIdx(5).to_suit(), None);
    }
}
