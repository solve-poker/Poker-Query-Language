use crate::{
    Board, Card, IsomorphicCard, SuitMap,
    card::{
        equiv::{
            isomorphic_turn::TurnTexture,
            util::{n_flush_suits, place_card},
        },
        util::sort4,
    },
};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct IsomorphicTurnEv(pub [IsomorphicCard; Board::N_TURN]);

impl IsomorphicTurnEv {
    /// panics if less than 4 cards
    pub const fn to_isomorphic(cards: &[Card]) -> (Self, SuitMap) {
        let [a, b, c, d] = sort4!(Card, cards[0], cards[1], cards[2], cards[3]);
        let map = TurnTexture::from_turn(a, b, c, d).to_suit_map();

        let c0 = map.iso_card(cards[0]);
        let c1 = map.iso_card(cards[1]);
        let c2 = map.iso_card(cards[2]);
        let c3 = map.iso_card(cards[3]);

        (Self(sort4!(IsomorphicCard, c0, c1, c2, c3)), map)
    }

    pub const fn to_array(self) -> [Card; Board::N_TURN] {
        let k = n_flush_suits(&self.0);

        let (c0, k) = place_card(self.0[0], k);
        let (c1, k) = place_card(self.0[1], k);
        let (c2, k) = place_card(self.0[2], k);
        let (c3, _) = place_card(self.0[3], k);

        [c0, c1, c2, c3]
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_iso_turn_ev() {
        let mut set = FxHashSet::default();
        for cs in HandN::<4>::iter_all::<true>() {
            set.insert(IsomorphicTurnEv::to_isomorphic(cs.as_slice()).0);
        }

        assert_eq!(set.len(), 3747);

        assert_eq!(
            IsomorphicTurnEv::to_isomorphic(&cards!("As Ks Ah Qh")).0,
            IsomorphicTurnEv::to_isomorphic(&cards!("Qh As Ks Ah")).0
        );
    }

    fn assert_roundtrip(cs: &[Card]) {
        let (iso, _) = IsomorphicTurnEv::to_isomorphic(cs);
        let (back, _) = IsomorphicTurnEv::to_isomorphic(&iso.to_array());

        assert_eq!(
            back,
            iso,
            "{cs:?}: {iso:?} -> {:?} -> {back:?}",
            iso.to_array()
        );
    }

    #[quickcheck]
    fn test_to_array_roundtrip(cards: CardN<4>) {
        assert_roundtrip(cards.as_slice());
    }
}
