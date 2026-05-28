use crate::{
    Board, Card, IsomorphicCard, Suit, SuitMap,
    card::{
        equiv::util::{n_flush_suits, place_card},
        util::sort5,
    },
};

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
pub struct IsomorphicRiverEv(
    /// The relabeled and rank-sorted river cards.
    pub [IsomorphicCard; Board::N_RIVER],
);

impl IsomorphicRiverEv {
    /// Canonical suit-isomorphic form of `cards` and the [`SuitMap`] that produced it.
    ///
    /// # Panics
    /// Panics if `cards` contains fewer than 5 cards.
    pub const fn to_isomorphic(cards: &[Card]) -> (Self, SuitMap) {
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

    /// Materializes this representative as a concrete card array with placed suits.
    pub const fn to_array(self) -> [Card; Board::N_RIVER] {
        let k = n_flush_suits(&self.0);

        let (c0, k) = place_card(self.0[0], k);
        let (c1, k) = place_card(self.0[1], k);
        let (c2, k) = place_card(self.0[2], k);
        let (c3, k) = place_card(self.0[3], k);
        let (c4, _) = place_card(self.0[4], k);

        [c0, c1, c2, c3, c4]
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use crate::{card::equiv::iso_river_ev::flush_suit, *};

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

    #[test]
    fn test_iso_river_ev() {
        let mut set = FxHashSet::default();
        for cs in HandN::<5>::iter_all::<true>() {
            set.insert(IsomorphicRiverEv::to_isomorphic(cs.as_slice()).0);
        }

        assert_eq!(set.len(), 6318);
    }

    fn assert_roundtrip(cs: &[Card]) {
        let (iso, _) = IsomorphicRiverEv::to_isomorphic(cs);
        let (back, _) = IsomorphicRiverEv::to_isomorphic(&iso.to_array());

        assert_eq!(
            back,
            iso,
            "{cs:?}: {iso:?} -> {:?} -> {back:?}",
            iso.to_array()
        );
    }

    #[quickcheck]
    fn test_to_array_roundtrip(cards: CardN<5>) {
        assert_roundtrip(cards.as_slice());
    }
}
