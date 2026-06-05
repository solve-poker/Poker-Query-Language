use crate::{
    Card, FlushingSuit, IsomorphicCard, IsomorphicHandN,
    card::equiv::IsomorphicTurnEv,
};

const N_HOLDEM: usize = 2;
const N_OMAHA: usize = 4;

impl IsomorphicHandN<N_HOLDEM> {
    #[must_use]
    pub const fn from_slice_preflop(cards: &[Card]) -> Self {
        debug_assert!(
            cards.len() >= N_HOLDEM,
            "not enough cards for IsomorphicHandN<2>"
        );

        let (c0, c1) = (cards[0], cards[1]);

        let (r0, r1) = if c0.rank.const_lt(c1.rank) {
            (c0.rank, c1.rank)
        } else {
            (c1.rank, c0.rank)
        };

        let suit = if c0.suit.const_eq(c1.suit) {
            FlushingSuit::X
        } else {
            FlushingSuit::N
        };

        Self([IsomorphicCard::new(r0, suit), IsomorphicCard::new(r1, suit)])
    }
}

impl IsomorphicHandN<N_OMAHA> {
    #[must_use]
    pub const fn from_slice_preflop(cards: &[Card]) -> Self {
        debug_assert!(
            cards.len() >= N_OMAHA,
            "not enough cards for IsomorphicHandN<4>"
        );

        Self(IsomorphicTurnEv::from_cards(cards).0.0)
    }
}
