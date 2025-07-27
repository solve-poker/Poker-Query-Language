use super::{mem, Card, Card64, From, Into, PQLCardCount, Rank, Rank16, *};

mod flop;
mod river;
mod turn;

pub use flop::Flop;
pub use river::River;
pub use turn::Turn;

#[derive(Clone, Copy, Into, From, Default, Eq, PartialEq)]
pub struct Board {
    pub flop: Flop,
    pub turn: Turn,
    pub river: River,
}

impl Board {
    #[must_use]
    pub fn swap_turn(self, c: Card) -> Self {
        Self {
            flop: self.flop,
            turn: c.into(),
            river: self.river,
        }
    }

    #[must_use]
    pub fn swap_river(self, c: Card) -> Self {
        Self {
            flop: self.flop,
            turn: self.turn,
            river: c.into(),
        }
    }

    pub fn contains_card(self, c: Card) -> bool {
        self.flop.0 == c
            || self.flop.1 == c
            || self.flop.2 == c
            || self.turn.0 == c
            || self.river.0 == c
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!(
            "<{:?}{:?}{:?} {:?} {:?}>",
            self.flop.0, self.flop.1, self.flop.2, self.turn.0, self.river.0,
        ))
    }
}

impl From<Board> for Flop {
    fn from(board: Board) -> Self {
        board.flop
    }
}

impl From<Board> for Turn {
    fn from(board: Board) -> Self {
        board.turn
    }
}

impl From<Board> for River {
    fn from(board: Board) -> Self {
        board.river
    }
}

impl From<(Card, Card, Card, Card, Card)> for Board {
    fn from(c: (Card, Card, Card, Card, Card)) -> Self {
        Self {
            flop: (c.0, c.1, c.2).into(),
            turn: c.3.into(),
            river: c.4.into(),
        }
    }
}

impl From<Board> for [Card; 5] {
    fn from(b: Board) -> Self {
        [b.flop.0, b.flop.1, b.flop.2, b.turn.into(), b.river.into()]
    }
}

impl From<Board> for Rank16 {
    fn from(b: Board) -> Self {
        let mut r: Self = b.flop.into();

        r.set(b.turn.into());
        r.set(b.river.into());

        r
    }
}

#[cfg(any(test, feature = "benchmark"))]
impl From<&[Card]> for Board {
    fn from(cs: &[Card]) -> Self {
        Self {
            flop: cs[0..3].into(),
            turn: cs[3].into(),
            river: cs[4].into(),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::*;

    impl Arbitrary for Board {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            CardN::<5>::arbitrary(g).as_ref().into()
        }
    }

    impl Board {
        pub fn to_vec(self) -> Vec<Card> {
            vec![
                self.flop.0,
                self.flop.1,
                self.flop.2,
                self.turn.0,
                self.river.0,
            ]
        }
    }

    #[quickcheck]
    fn test_into_rank16_and_array(board: Board) {
        let r: Rank16 = board.into();
        let arr: [_; 5] = board.into();

        assert!(r.contains_rank(board.flop.0.r));
        assert!(r.contains_rank(board.flop.1.r));
        assert!(r.contains_rank(board.flop.2.r));
        assert!(r.contains_rank(board.turn.0.r));
        assert!(r.contains_rank(board.river.0.r));

        assert_eq!(arr[0], board.flop.0);
        assert_eq!(arr[1], board.flop.1);
        assert_eq!(arr[2], board.flop.2);
        assert_eq!(arr[3], board.turn.0);
        assert_eq!(arr[4], board.river.0);
    }
}
