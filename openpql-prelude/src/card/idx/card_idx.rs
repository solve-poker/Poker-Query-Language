use crate::{Card, RankIdx, SuitIdx, card::Idx};

/// Numeric index of a card in the range 0-51.
#[cfg_attr(feature = "speedy", derive(speedy::Readable, speedy::Writable))] // LCOV_EXCL_LINE
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CardIdx(pub(crate) Idx);

impl CardIdx {
    /// Returns the card, or `None` if out of range.
    #[must_use]
    pub const fn to_card(self) -> Option<Card> {
        match (RankIdx(self.0 / 4).to_rank(), SuitIdx(self.0 % 4).to_suit()) {
            (Some(r), Some(s)) => Some(Card::new(r, s)),
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

impl From<Card> for CardIdx {
    fn from(card: Card) -> Self {
        let idx_rank = RankIdx::from(card.rank).0;
        let idx_suit = SuitIdx::from(card.suit).0;

        Self(idx_rank * 4 + idx_suit)
    }
}

#[cfg(any(test, feature = "quickcheck"))]
impl quickcheck::Arbitrary for CardIdx {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        Card::arbitrary(g).into()
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;

    #[quickcheck]
    fn test_const_cmp(a: CardIdx, b: CardIdx) {
        assert_eq!(a < b, a.const_lt(b));
        assert_eq!(a == b, a.const_eq(b));
    }

    #[test]
    #[allow(clippy::cast_possible_wrap)]
    fn test_from_card() {
        for i in 0..Card::N_CARDS {
            assert_eq!(CardIdx::from(Card::all::<false>()[i as usize]).0, i as Idx);
        }
    }

    #[test]
    #[allow(clippy::cast_possible_wrap)]
    fn test_to_card() {
        for i in 0..Card::N_CARDS {
            assert_eq!(
                CardIdx(i as Idx).to_card().unwrap(),
                Card::all::<false>()[i as usize]
            );
        }

        assert!(CardIdx(-1).to_card().is_none());
        assert!(CardIdx(53).to_card().is_none());
    }
}
