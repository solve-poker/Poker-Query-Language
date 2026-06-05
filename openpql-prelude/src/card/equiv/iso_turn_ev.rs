use crate::{
    Board, Card, IsomorphicCard, SuitMap,
    card::{equiv::isomorphic_turn::TurnTexture, util::sort4},
};

/// Canonical suit-isomorphic representative of a four-card turn hand for equity evaluation.
#[cfg_attr(feature = "speedy", derive(speedy::Readable, speedy::Writable))] // LCOV_EXCL_LINE
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub(super) struct IsomorphicTurnEv(
    /// The relabeled and rank-sorted turn cards.
    pub [IsomorphicCard; Board::N_TURN],
);

impl IsomorphicTurnEv {
    /// Canonical suit-isomorphic form of `cards` and the [`SuitMap`] that produced it.
    ///
    /// # Panics
    /// Panics if `cards` contains fewer than 4 cards.
    pub const fn from_cards(cards: &[Card]) -> (Self, SuitMap) {
        let [a, b, c, d] = sort4!(Card, cards[0], cards[1], cards[2], cards[3]);
        let map = TurnTexture::from_turn(a, b, c, d).to_suit_map();

        let c0 = map.iso_card(cards[0]);
        let c1 = map.iso_card(cards[1]);
        let c2 = map.iso_card(cards[2]);
        let c3 = map.iso_card(cards[3]);

        (Self(sort4!(IsomorphicCard, c0, c1, c2, c3)), map)
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_iso_turn_eq() {
        assert_eq!(
            IsomorphicTurnEv::from_cards(&cards!("As Ks Ah Qh")).0,
            IsomorphicTurnEv::from_cards(&cards!("Qh As Ks Ah")).0
        );
    }

    fn assert_roundtrip(cs: &[Card]) {
        let (iso, _) = IsomorphicTurnEv::from_cards(cs);
        let hand = IsomorphicHand::from(iso.0).to_hand();
        let (back, _) = IsomorphicTurnEv::from_cards(&hand);

        assert_eq!(back, iso, "{cs:?}: {iso:?} -> {hand:?} -> {back:?}");
    }

    #[quickcheck]
    fn test_to_array_roundtrip(cards: CardN<4>) {
        assert_roundtrip(cards.as_slice());
    }
}
