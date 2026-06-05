use crate::{Board, Card, IsomorphicCard, Suit, SuitMap, card::util::sort5};

const fn flush_suit(
    s0: Suit,
    s1: Suit,
    s2: Suit,
    s3: Suit,
    s4: Suit,
) -> Option<Suit> {
    match (s0, s1, s2, s3, s4) {
        (a, b, c, _, _) if a.const_eq(b) && b.const_eq(c) => Some(a),
        (a, b, _, d, _) if a.const_eq(b) && b.const_eq(d) => Some(a),
        (a, b, _, _, e) if a.const_eq(b) && b.const_eq(e) => Some(a),
        (a, _, c, d, _) if a.const_eq(c) && c.const_eq(d) => Some(a),
        (a, _, c, _, e) if a.const_eq(c) && c.const_eq(e) => Some(a),
        (a, _, _, d, e) if a.const_eq(d) && d.const_eq(e) => Some(a),
        (_, b, c, d, _) if b.const_eq(c) && c.const_eq(d) => Some(b),
        (_, b, c, _, e) if b.const_eq(c) && c.const_eq(e) => Some(b),
        (_, b, _, d, e) if b.const_eq(d) && d.const_eq(e) => Some(b),
        (_, _, c, d, e) if c.const_eq(d) && d.const_eq(e) => Some(c),
        _ => None,
    }
}

/// Canonical suit-isomorphic representative of a five-card river hand for equity evaluation.
#[cfg_attr(feature = "speedy", derive(speedy::Readable, speedy::Writable))] // LCOV_EXCL_LINE
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub(super) struct IsomorphicRiverEv(
    /// The relabeled and rank-sorted river cards.
    pub [IsomorphicCard; Board::N_RIVER],
);

impl IsomorphicRiverEv {
    /// Canonical suit-isomorphic form of `cards` and the [`SuitMap`] that produced it.
    ///
    /// # Panics
    /// Panics if `cards` contains fewer than 5 cards.
    pub const fn from_cards(cards: &[Card]) -> (Self, SuitMap) {
        let map = match flush_suit(
            cards[0].suit,
            cards[1].suit,
            cards[2].suit,
            cards[3].suit,
            cards[4].suit,
        ) {
            Some(suit) => SuitMap::map1(suit),
            None => SuitMap::map0(),
        };

        let c0 = map.iso_card(cards[0]);
        let c1 = map.iso_card(cards[1]);
        let c2 = map.iso_card(cards[2]);
        let c3 = map.iso_card(cards[3]);
        let c4 = map.iso_card(cards[4]);

        (Self(sort5!(IsomorphicCard, c0, c1, c2, c3, c4)), map)
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_flush_suit(cards: CardN<5>) {
        let c64 = Card64::from(cards.as_slice());
        let lhs = if c64.max_same_suit_count() >= 3 {
            Some(c64.most_frequent_suits())
        } else {
            None
        };

        let rhs = flush_suit(
            cards[0].suit,
            cards[1].suit,
            cards[2].suit,
            cards[3].suit,
            cards[4].suit,
        )
        .map(Suit4::from);

        assert_eq!(lhs, rhs);
    }

    fn assert_roundtrip(cs: &[Card]) {
        let (iso, _) = IsomorphicRiverEv::from_cards(cs);
        let hand = IsomorphicHand::from(iso.0).to_hand();
        let (back, _) = IsomorphicRiverEv::from_cards(&hand);

        assert_eq!(back, iso, "{cs:?}: {iso:?} -> {hand:?} -> {back:?}");
    }

    #[quickcheck]
    fn test_to_array_roundtrip(cards: CardN<5>) {
        assert_roundtrip(cards.as_slice());
    }
}
