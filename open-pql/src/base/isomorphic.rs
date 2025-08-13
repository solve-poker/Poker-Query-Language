use super::{
    Board, Card, Flop, HandN, IDX_RIVER, IDX_TURN, N_FLOP, Suit, SuitMapping,
    Vec,
};

impl<const N: usize> HandN<N> {
    pub fn new_iso_with_mapping(
        cards: &[Card],
        mapping: &mut SuitMapping,
    ) -> Self {
        Self::new(create_iso_array(cards, mapping))
    }

    pub fn new_iso(cards: &[Card]) -> (Self, SuitMapping) {
        let mut mapping = SuitMapping::default();
        let iso = Self::new_iso_with_mapping(cards, &mut mapping);
        (iso, mapping)
    }
}

impl Board {
    pub fn new_iso_with_mapping(
        cards: &[Card],
        mapping: &mut SuitMapping,
    ) -> Self {
        create_iso_board(cards, mapping)
    }

    pub fn new_iso(cards: &[Card]) -> (Self, SuitMapping) {
        let mut mapping = SuitMapping::default();
        let iso = Self::new_iso_with_mapping(cards, &mut mapping);
        (iso, mapping)
    }
}

fn create_iso_array<const N: usize>(
    cards: &[Card],
    mapping: &mut SuitMapping,
) -> [Card; N] {
    let mut sorted = cards[..N]
        .iter()
        .map(|c| {
            (
                cards[..N].iter().filter(|&el| el.suit == c.suit).count(),
                c.rank,
                c,
            )
        })
        .collect::<Vec<_>>();

    sorted.sort_unstable();

    let mut res = [Card::default(); N];

    for (i, (_, _, card)) in sorted.into_iter().enumerate() {
        res[i] = Card::new(card.rank, mapping.map_suit(card.suit));
    }

    res.sort_unstable();

    res
}

fn create_iso_board(board_cards: &[Card], mapping: &mut SuitMapping) -> Board {
    let n = board_cards.len();
    let mut board = Board::default();

    if n >= N_FLOP {
        board.flop =
            Some(Flop::new(create_iso_array(&board_cards[..N_FLOP], mapping)));
    }

    if n > IDX_TURN {
        let card = board_cards[IDX_TURN];
        board.turn = Some(Card::new(card.rank, mapping.map_suit(card.suit)));
    }

    if n > IDX_RIVER {
        let card = board_cards[IDX_RIVER];
        board.river = Some(Card::new(card.rank, mapping.map_suit(card.suit)));
    }

    board
}

/// Converts a suit to its isomorphic character representation
pub const fn to_suitvar_char(s: Suit) -> char {
    match s {
        Suit::S => 'w',
        Suit::H => 'x',
        Suit::D => 'y',
        Suit::C => 'z',
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use super::*;
    use crate::*;

    #[test]
    fn test_to_suitvar_char() {
        assert_eq!(to_suitvar_char(Suit::S), 'w');
        assert_eq!(to_suitvar_char(Suit::H), 'x');
        assert_eq!(to_suitvar_char(Suit::D), 'y');
        assert_eq!(to_suitvar_char(Suit::C), 'z');
    }

    #[test]
    fn test_iso() {
        assert_eq!(
            HandN::<5>::new_iso(&cards!("6s8h9dQdQc")).0,
            HandN::<5>::new_iso(&cards!("6s8h9dQcQd")).0
        );
    }

    #[test]
    fn test_iso_flop() {
        let mut res: HashMap<HandN<3>, HandN<3>> = HashMap::new();
        let mut iso_set: HashSet<HandN<3>> = HashSet::default();
        for hand in HandN::<3>::iter_all_short() {
            let (iso, _) = HandN::new_iso(&hand.0);
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
