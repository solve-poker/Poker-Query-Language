use super::{Card, Card64, Flop, Hash, fmt};
use crate::{CardCount, Suit, Suit4};

#[macro_export]
macro_rules! board {
    ($s:expr) => {
        $crate::Board::from(cards!($s).as_slice())
    };
}

/// Board representation for poker games.
///
/// Represents community cards (flop, turn, river) with a macro for convenient creation.
#[cfg_attr(feature = "speedy", derive(speedy::Readable, speedy::Writable))]
#[derive(Copy, Clone, derive_more::Debug, PartialEq, Eq, Hash, Default)]
#[debug("Board<{}>", self)]
pub struct Board {
    pub flop: Option<Flop>,
    pub turn: Option<Card>,
    pub river: Option<Card>,
}

impl Board {
    /// Index of the turn card in a board array
    pub const IDX_TURN: usize = 3;
    /// Index of the river card in a board array
    pub const IDX_RIVER: usize = 4;
    /// Number of board cards preflop
    pub const N_PREFLOP: usize = 0;
    /// Number of board cards in a flop
    pub const N_FLOP: usize = 3;
    /// Number of board cards in a flop + turn
    pub const N_TURN: usize = 4;
    /// Number of board cards in a flop + turn + river
    pub const N_RIVER: usize = 5;

    /// Creates a board from a slice of cards.
    ///
    /// Expects cards in order: flop (3 cards), turn, river.
    /// Cards beyond the first 5 are ignored.
    pub fn from_slice(cards: &[Card]) -> Self {
        let flop = if cards.len() >= Self::N_FLOP {
            Some(Flop::from_slice(&cards[0..Self::N_FLOP]))
        } else {
            None
        };
        let turn = cards.get(Self::IDX_TURN).copied();
        let river = cards.get(Self::IDX_RIVER).copied();

        Self { flop, turn, river }
    }

    /// Returns `true` if the board has no cards (no flop).
    #[must_use]
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.flop.is_none()
    }

    /// Returns the number of cards on the board (0, 3, 4, or 5).
    pub fn len(&self) -> usize {
        match self.flop {
            Some(_) => {
                Self::N_FLOP
                    + self.turn.iter().count()
                    + self.river.iter().count()
            }
            None => 0,
        }
    }

    pub fn to_vec(&self) -> Vec<Card> {
        match (self.flop, self.turn, self.river) {
            (Some(flop), Some(turn), Some(river)) => {
                vec![flop[0], flop[1], flop[2], turn, river]
            }
            (Some(flop), Some(turn), _) => {
                vec![flop[0], flop[1], flop[2], turn]
            }
            (Some(flop), _, _) => flop.to_vec(),
            _ => vec![],
        }
    }

    /// Returns an iterator over all cards on the board.
    pub fn iter(&self) -> impl Iterator<Item = Card> + '_ {
        self.flop
            .iter()
            .flat_map(|flop| flop.iter().copied())
            .chain(self.turn)
            .chain(self.river)
    }

    /// Returns `true` if the board contains the specified card.
    pub const fn contains_card(&self, card: Card) -> bool {
        #[inline]
        const fn inner_eq(op: Option<Card>, rhs: Card) -> bool {
            match op {
                Some(lhs) => lhs.eq(rhs),
                None => false,
            }
        }

        if let Some(flop) = self.flop
            && flop.contains_card(card)
        {
            return true;
        }

        inner_eq(self.turn, card) || inner_eq(self.river, card)
    }

    /// Returns the suits that can still produce a flush by the river.
    pub const fn flush_suits(&self) -> Suit4 {
        match (self.flop, self.turn, self.river) {
            (Some(flop), None, None) => {
                let mut suits = Suit4(0);

                suits.set(flop.0[0].suit);
                suits.set(flop.0[1].suit);
                suits.set(flop.0[2].suit);
                suits
            }
            (Some(flop), Some(turn), None) => {
                let (s1, s2, s3, s4) =
                    (flop.0[0].suit, flop.0[1].suit, flop.0[2].suit, turn.suit);
                let mut suits = Suit4(0);

                if (s1.eq(s2)) || (s1.eq(s3)) || (s1.eq(s4)) {
                    suits.set(s1);
                }

                if (s2.eq(s3)) || (s2.eq(s4)) {
                    suits.set(s2);
                }

                if s3.eq(s4) {
                    suits.set(s3);
                }

                suits
            }
            (Some(flop), Some(turn), Some(river)) => {
                #[inline]
                pub const fn count(l: Suit, r: Suit) -> CardCount {
                    if l.eq(r) { 1 } else { 0 }
                }
                const N_OTHER: CardCount = 2;
                let (s1, s2, s3, s4, s5) = (
                    flop.0[0].suit,
                    flop.0[1].suit,
                    flop.0[2].suit,
                    turn.suit,
                    river.suit,
                );

                if count(s1, s2) + count(s1, s3) + count(s1, s4) + count(s1, s5)
                    >= N_OTHER
                {
                    Suit4::from_suit(s1)
                } else if count(s2, s1)
                    + count(s2, s3)
                    + count(s2, s4)
                    + count(s2, s5)
                    >= N_OTHER
                {
                    Suit4::from_suit(s2)
                } else if count(s3, s1)
                    + count(s3, s2)
                    + count(s3, s4)
                    + count(s3, s5)
                    >= N_OTHER
                {
                    Suit4::from_suit(s3)
                } else {
                    Suit4(0)
                }
            }
            _ => Suit4::ALL,
        }
    }

    pub(crate) const fn to_c64_flop(self) -> Card64 {
        match self.flop {
            Some(flop) => flop.to_c64(),
            None => Card64::EMPTY,
        }
    }

    pub(crate) const fn to_c64_turn(self) -> Card64 {
        match self.turn {
            Some(turn) => turn.to_c64(),
            None => Card64::EMPTY,
        }
    }

    pub(crate) const fn to_c64_river(self) -> Card64 {
        match self.river {
            Some(river) => river.to_c64(),
            None => Card64::EMPTY,
        }
    }
}

impl From<&[Card]> for Board {
    fn from(xs: &[Card]) -> Self {
        Self::from_slice(xs)
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.iter().try_for_each(|card| write!(f, "{card}"))
    }
}

impl From<Board> for Card64 {
    fn from(board: Board) -> Self {
        board.iter().collect()
    }
}

#[cfg(any(test, feature = "quickcheck"))]
impl quickcheck::Arbitrary for Board {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let cards = crate::CardN::<{ Self::N_RIVER }>::arbitrary(g);
        let street = crate::Street::arbitrary(g);

        Self::from_slice(&cards.as_ref()[..street.board_card_count() as usize])
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_from_slice(cs: CardN<5>) {
        assert_eq!(
            Board::from([cs[0], cs[1], cs[2], cs[3], cs[4]].as_slice()),
            Board::from([cs[2], cs[1], cs[0], cs[3], cs[4]].as_slice()),
        );
        assert_ne!(
            Board::from([cs[0], cs[1], cs[2], cs[3], cs[4]].as_slice()),
            Board::from([cs[0], cs[1], cs[2], cs[4], cs[3]].as_slice()),
        );
    }

    #[test]
    fn test_empty() {
        assert!(Board::default().is_empty());
    }

    #[test]
    fn test_board_iter_and_len() {
        let board = board!("Qd As Kh Jc");

        assert_eq!(board.iter().collect::<Vec<_>>(), cards!("Qd Kh As Jc"));
        assert_eq!(board.len(), 4);
        assert_eq!(Board::default().len(), 0);
    }

    #[quickcheck]
    fn test_board_contains_card(board: Board, card: Card) {
        assert_eq!(board.iter().any(|x| x == card), board.contains_card(card));
        assert!(!Board::default().contains_card(card));
    }

    #[quickcheck]
    fn test_flush_suits(board: Board) {
        const N_FLUSH: CardCount = 3;

        let street = Street::from(board);
        let n_future_cards = 5 - street.board_card_count();
        let cs = Card64::from((board, street));

        assert_eq!(
            board.flush_suits(),
            Suit::ARR_ALL
                .into_iter()
                .filter(|&suit| {
                    cs.count_by_suit(suit) + n_future_cards >= N_FLUSH
                })
                .collect()
        );
    }

    #[quickcheck]
    fn test_to_vec(board: Board) {
        let expected = match Street::from(board) {
            Street::Preflop => vec![],
            Street::Flop => board.flop.unwrap().to_vec(),
            Street::Turn => board
                .flop
                .unwrap()
                .into_iter()
                .chain([board.turn.unwrap()])
                .collect(),
            Street::River => board
                .flop
                .unwrap()
                .into_iter()
                .chain([board.turn.unwrap(), board.river.unwrap()])
                .collect(),
        };

        assert_eq!(board.to_vec(), expected);
    }

    #[test]
    fn test_display() {
        let cards = cards!["AsKhQd3s2s"];
        let board = Board::from_slice(&cards);

        assert_eq!(format!("{board}"), "QdKhAs3s2s");
        assert_eq!(format!("{board:?}"), "Board<QdKhAs3s2s>");
    }

    #[test]
    fn test_to_c64() {
        let cards = cards!("AsKhQdJcTs");

        for i in 3..=5 {
            let board = Board::from_slice(&cards[0..i]);
            let card64 = Card64::from(board);

            for j in 0..i {
                assert!(card64.contains_card(cards[j]));
            }

            assert_eq!(card64.count() as usize, i);
        }
    }
}
