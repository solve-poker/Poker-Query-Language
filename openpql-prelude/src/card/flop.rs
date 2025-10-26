use super::{Board, Card, Card64, HandN, ops};

#[macro_export]
macro_rules! flop {
    ($s:expr) => {
        $crate::Flop::from(<[$crate::Card; 3]>::try_from(cards!($s)).unwrap())
    };
}

/// Flop representation.
///
/// Type alias for `HandN<3>` representing the first three community cards.
/// Includes a macro for convenient flop creation.
pub type Flop = HandN<3>;

impl Flop {
    pub(crate) const fn to_c64(self) -> Card64 {
        let mut res = Card64::EMPTY;
        res.set(self.0[0]);
        res.set(self.0[1]);
        res.set(self.0[2]);

        res
    }

    pub(crate) const fn contains_card(self, card: Card) -> bool {
        self.0[0].eq(card) || self.0[1].eq(card) || self.0[2].eq(card)
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

impl ops::BitOrAssign<Flop> for Card64 {
    fn bitor_assign(&mut self, rhs: Flop) {
        self.set(rhs[0]);
        self.set(rhs[1]);
        self.set(rhs[2]);
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_to_board(cards: CardN<3>) {
        let flop = Flop::from_slice(cards.as_slice());
        let board = Board::from_slice(cards.as_slice());

        assert_eq!(Board::from(flop), board);
    }

    #[quickcheck]
    fn bit_or_assign(cards: CardN<3>, c64: Card64) {
        let flop = Flop::from_slice(cards.as_slice());
        let mut copy = c64;
        copy |= flop;

        assert_eq!(copy, Card64::from(flop) | c64);
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
}
