use super::{Card, Idx, RankIdx, SuitIdx};

/// Card index representation.
///
/// Converts cards to unique numeric indices (0-51).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct CardIdx(pub(crate) Idx);

impl CardIdx {
    pub const fn to_card(self) -> Option<Card> {
        match (RankIdx(self.0 / 4).to_rank(), SuitIdx(self.0 % 4).to_suit()) {
            (Some(r), Some(s)) => Some(Card::new(r, s)),
            _ => None,
        }
    }
}

impl From<Card> for CardIdx {
    fn from(card: Card) -> Self {
        let idx_rank = RankIdx::from(card.rank).0;
        let idx_suit = SuitIdx::from(card.suit).0;

        Self(idx_rank * 4 + idx_suit)
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::cast_possible_wrap)]
    fn test_from_card() {
        for i in 0..Card::N_CARDS {
            assert_eq!(
                CardIdx::from(Card::all::<false>()[i as usize]).0,
                i as Idx
            );
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
