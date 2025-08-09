use super::{Card, Card64, HandN, Hash, N_FLOP, fmt};

pub type Flop = HandN<3>;

/// Represents a poker board (flop, turn, river)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct Board {
    pub flop: Option<Flop>,
    pub turn: Option<Card>,
    pub river: Option<Card>,
}

impl Board {
    /// Creates a board from a slice of cards
    pub fn from_slice(cards: &[Card]) -> Self {
        let flop = if cards.len() >= N_FLOP {
            Some(Flop::from_slice(&cards[0..3]))
        } else {
            None
        };
        let turn = cards.get(3).copied();
        let river = cards.get(4).copied();

        Self { flop, turn, river }
    }

    /// Checks if the board is empty (has no cards)
    #[must_use]
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.flop.is_none()
    }

    /// Returns the number of cards on the board
    pub fn len(&self) -> usize {
        match self.flop {
            Some(_) => 3 + self.turn.iter().count() + self.river.iter().count(),
            None => 0,
        }
    }

    /// Gets a card at the specified index
    pub fn get(&self, i: usize) -> Option<Card> {
        match i {
            0..=2 => self.flop.map(|f| f[i]),
            3 => self.turn,
            4 => self.river,
            _ => None,
        }
    }

    /// Returns an iterator over all cards on the board
    pub fn iter(&self) -> impl Iterator<Item = Card> + '_ {
        let flop_iter = self.flop.iter().flat_map(Flop::iter);
        flop_iter.chain(self.turn).chain(self.river)
    }

    /// Converts the board to a Card64 representation
    pub fn to_card64(&self) -> Card64 {
        let mut result = Card64::default();
        if let Some(flop) = self.flop {
            result |= Card64::from(flop.as_slice());
        }
        if let Some(turn) = self.turn {
            result |= Card64::from(turn);
        }
        if let Some(river) = self.river {
            result |= Card64::from(river);
        }
        result
    }

    /// Returns all cards on the board as a vector
    pub fn to_vec(&self) -> Vec<Card> {
        self.iter().collect()
    }

    /// Clears the board
    pub fn clear(&mut self) {
        *self = Self::default();
    }

    pub fn contains_card(&self, card: Card) -> bool {
        if let Some(flop) = self.flop
            && flop.as_slice().contains(&card)
        {
            return true;
        }

        Some(card) == self.turn || Some(card) == self.river
    }

    pub const fn swap_turn(&self, card: Card) -> Self {
        Self {
            flop: self.flop,
            turn: Some(card),
            river: self.river,
        }
    }

    pub const fn swap_river(&self, card: Card) -> Self {
        Self {
            flop: self.flop,
            turn: self.turn,
            river: Some(card),
        }
    }
}

impl From<&[Card]> for Board {
    fn from(xs: &[Card]) -> Self {
        Self::from_slice(xs)
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Board<")?;
        for c in self.iter() {
            write!(f, "{c}")?;
        }
        write!(f, ">")
    }
}

impl From<Board> for Card64 {
    fn from(board: Board) -> Self {
        board.to_card64()
    }
}

impl From<(Card, Card, Card, Card, Card)> for Board {
    fn from(cs: (Card, Card, Card, Card, Card)) -> Self {
        Self {
            flop: Some(Flop::from_slice(&[cs.0, cs.1, cs.2])),
            turn: Some(cs.3),
            river: Some(cs.4),
        }
    }
}

impl From<Board> for [Card; 5] {
    fn from(board: Board) -> Self {
        if let (Some(flop), Some(turn), Some(river)) =
            (board.flop, board.turn, board.river)
        {
            [flop[0], flop[1], flop[2], turn, river]
        } else {
            panic!()
        }
    }
}

impl From<Flop> for Board {
    fn from(flop: Flop) -> Self {
        Self {
            flop: Some(flop),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    impl Arbitrary for Board {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let cards = CardN::<5>::arbitrary(g);

            Self::from_slice(cards.as_ref())
        }
    }

    #[quickcheck]
    fn test_flop_eq(cards: CardN<3>) {
        let flops: Vec<Flop> = vec![
            Flop::from_slice(&[cards[0], cards[1], cards[2]]),
            Flop::from_slice(&[cards[0], cards[2], cards[1]]),
            Flop::from_slice(&[cards[1], cards[0], cards[2]]),
            Flop::from_slice(&[cards[1], cards[2], cards[0]]),
            Flop::from_slice(&[cards[2], cards[1], cards[0]]),
            Flop::from_slice(&[cards[2], cards[0], cards[1]]),
        ];

        for i in 0..flops.len() {
            for j in 0..flops.len() {
                assert_eq!(flops[i], flops[j]);
            }
        }
    }

    #[quickcheck]
    fn test_board_eq(cs: CardN<5>) {
        assert_eq!(
            Board::from([cs[0], cs[1], cs[2], cs[3], cs[4]].as_slice()),
            Board::from([cs[2], cs[1], cs[0], cs[3], cs[4]].as_slice())
        );
    }

    #[test]
    fn test_board_creation() {
        // Empty board
        let empty_board = Board::default();
        assert!(empty_board.is_empty());
        assert_eq!(empty_board.len(), 0);

        // No Flop
        let flop_cards = cards!("AsKhQd");
        for j in 0..=2 {
            assert_eq!(Board::from_slice(&flop_cards[0..j]).len(), 0);
        }

        // Flop only
        let flop_board = Board::from_slice(&flop_cards);
        assert!(!flop_board.is_empty());
        assert_eq!(flop_board.len(), 3);
        assert_eq!(flop_board.get(0), Some(flop_cards[2]));
        assert_eq!(flop_board.get(1), Some(flop_cards[1]));
        assert_eq!(flop_board.get(2), Some(flop_cards[0]));
        assert_eq!(flop_board.get(3), None);
        assert_eq!(flop_board.get(4), None);

        // Flop + Turn
        let turn_card = Card::new(Rank::RJ, Suit::C);
        let mut flop_turn_cards = flop_cards;
        flop_turn_cards.push(turn_card);
        let flop_turn_board = Board::from_slice(&flop_turn_cards);
        assert_eq!(flop_turn_board.len(), 4);
        assert_eq!(flop_turn_board.get(3), Some(turn_card));
        assert_eq!(flop_turn_board.get(4), None);

        // Full board (Flop + Turn + River)
        let river_card = Card::new(Rank::RT, Suit::S);
        let mut full_cards = flop_turn_cards;
        full_cards.push(river_card);
        let full_board = Board::from_slice(&full_cards);
        assert_eq!(full_board.len(), 5);
        assert_eq!(full_board.get(4), Some(river_card));
        assert_eq!(full_board.get(5), None);
    }

    #[test]
    fn test_board_clear() {
        let cards = cards!("QdKhAsJcTs");
        let mut board = Board::from_slice(&cards);

        assert!(!board.is_empty());
        board.clear();
        assert!(board.is_empty());
    }

    #[test]
    fn test_board_iteration() {
        let cards = cards!("QdKhAsJcTs");
        // Test with different board sizes
        for i in N_FLOP..=5 {
            let board = Board::from_slice(&cards[0..i]);
            let collected: Vec<Card> = board.iter().collect();
            assert_eq!(collected, cards[0..i].to_vec());
            assert_eq!(board.to_vec(), cards[0..i].to_vec());
            assert_eq!(board.len(), i);
        }
    }

    #[test]
    fn test_board_card64_conversion() {
        let cards = cards!("AsKhQdJcTs");

        // Test with different board sizes
        for i in N_FLOP..=5 {
            let board = Board::from_slice(&cards[0..i]);
            let card64 = board.to_card64();

            // Verify each card is set in the Card64
            for j in 0..i {
                assert!(card64.contains_card(cards[j]));
            }

            assert_eq!(card64.count() as usize, i);
        }
    }

    #[test]
    fn test_board_debug_format() {
        let cards = cards!["AsKhQd"];

        let board = Board::from_slice(&cards);
        let debug_str = format!("{board:?}");
        assert!(debug_str.contains("Board<QdKhAs>"));
    }
}
