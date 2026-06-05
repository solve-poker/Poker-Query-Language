use std::hash::Hash;

use crate::{Suit, card::Idx};

/// Numeric index of a suit in the range 0-3.
#[cfg_attr(feature = "speedy", derive(speedy::Readable, speedy::Writable))] // LCOV_EXCL_LINE
#[derive(
    Copy,
    Clone,
    PartialEq,
    Eq,
    Debug,
    Ord,
    PartialOrd,
    Hash,
    derive_more::Display,
    Default,
)]
pub struct SuitIdx(pub(crate) Idx);

impl SuitIdx {
    /// Returns the suit, or `None` if out of range.
    #[must_use]
    pub const fn to_suit(self) -> Option<Suit> {
        match self.0 {
            0 => Some(Suit::S),
            1 => Some(Suit::H),
            2 => Some(Suit::D),
            3 => Some(Suit::C),
            _ => None,
        }
    }

    /// Const-context equality, equivalent to [`PartialEq::eq`].
    #[inline]
    #[must_use]
    pub const fn const_eq(self, other: Self) -> bool {
        self.0 == other.0
    }

    /// Const-context less-than, equivalent to [`PartialOrd::lt`].
    #[inline]
    #[must_use]
    pub const fn const_lt(self, other: Self) -> bool {
        self.0 < other.0
    }
}

impl From<Suit> for SuitIdx {
    fn from(suit: Suit) -> Self {
        Self(suit as Idx)
    }
}

#[cfg(any(test, feature = "quickcheck"))]
impl quickcheck::Arbitrary for SuitIdx {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        Suit::arbitrary(g).into()
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;

    #[quickcheck]
    fn test_const_cmp(a: SuitIdx, b: SuitIdx) {
        assert_eq!(a < b, a.const_lt(b));
        assert_eq!(a == b, a.const_eq(b));
    }

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
