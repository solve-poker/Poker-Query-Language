use super::{Board, Card, CardCount, Flop, HandN, Suit, SuitMapping};

impl<const N: usize> HandN<N> {
    /// Creates a suit-isomorphic hand using the provided suit mapping.
    pub fn new_iso_with_mapping(
        cards: &[Card],
        mapping: &mut SuitMapping,
    ) -> Self {
        Self::new(create_iso_array(cards, mapping))
    }

    /// Creates a suit-isomorphic hand and returns the suit mapping used.
    pub fn new_iso(cards: &[Card]) -> (Self, SuitMapping) {
        let mut mapping = SuitMapping::default();
        let iso = Self::new_iso_with_mapping(cards, &mut mapping);
        (iso, mapping)
    }
}

impl Board {
    /// Creates a suit-isomorphic board using the provided suit mapping.
    pub fn new_iso_with_mapping(
        cards: &[Card],
        mapping: &mut SuitMapping,
    ) -> Self {
        create_iso_board(cards, mapping)
    }

    /// Creates a suit-isomorphic board and returns the suit mapping used.
    pub fn new_iso(cards: &[Card]) -> (Self, SuitMapping) {
        let mut mapping = SuitMapping::default();
        let iso = Self::new_iso_with_mapping(cards, &mut mapping);
        (iso, mapping)
    }
}

type SuitCount = [CardCount; Suit::N_SUITS as usize];

fn count_suits(cards: &[Card]) -> SuitCount {
    let mut res = SuitCount::default();

    for card in cards {
        res[card.suit as usize] += 1;
    }

    res
}

/// # Panics
/// input must be valid distict cards
fn create_iso_array<const N: usize>(
    slice: &[Card],
    mapping: &mut SuitMapping,
) -> [Card; N] {
    let mut cards: [_; N] = slice[..N].try_into().unwrap();
    let suit_count = count_suits(cards.as_slice());

    cards.sort_unstable_by_key(|&card| {
        (Suit::N_SUITS - suit_count[card.suit as usize], card)
    });

    for card in &mut cards {
        card.suit = mapping.map_suit(card.suit);
    }

    cards.sort_unstable();

    cards
}

fn create_iso_board(board_cards: &[Card], mapping: &mut SuitMapping) -> Board {
    #[inline]
    const fn map_card(card: Card, mapping: &mut SuitMapping) -> Card {
        Card::new(card.rank, mapping.map_suit(card.suit))
    }

    let n = board_cards.len();
    let mut board = Board::default();

    if n >= Board::N_FLOP {
        board.flop = Some(Flop::new_iso_with_mapping(board_cards, mapping));
    }

    if n >= Board::N_TURN {
        board.turn = Some(map_card(board_cards[Board::IDX_TURN], mapping));
    }

    if n >= Board::N_RIVER {
        board.river = Some(map_card(board_cards[Board::IDX_RIVER], mapping));
    }

    board
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_iso() {
        assert_eq!(
            HandN::<5>::new_iso(&cards!("6s8h9dQdQc")).0,
            HandN::<5>::new_iso(&cards!("6s8h9dQcQd")).0
        );
    }

    #[test]
    fn test_iso_flop() {
        let mut res = FxHashMap::default();
        let mut iso_set = FxHashSet::default();

        for hand in Flop::iter_all::<true>() {
            let (iso, _) = HandN::<3>::new_iso(&hand.0);
            res.insert(hand, iso);
            iso_set.insert(iso);
        }

        assert_eq!(res.len(), 7140);
        assert_eq!(iso_set.len(), 573);
    }

    #[test]
    fn test_iso_board() {
        fn assert_iso_eq(lhs: &str, rhs: &str) {
            assert_eq!(
                Board::new_iso(&cards!(lhs)).0,
                Board::new_iso(&cards!(rhs)).0
            );
        }

        assert_iso_eq("", "");
        assert_iso_eq("AsKhQd", "AhKsQc");
        assert_iso_eq("AsKhQdJdTd", "AhKsQcJcTc"); // TODO: maybe ignore turn/river order?
    }
}
