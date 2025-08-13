use super::{Card64, Rank16, Suit};

/// Iterator over the ranks in each suit of a Card64.
///
/// This iterator yields tuples of (Rank16, Suit) for each suit,
/// where the Rank16 contains all ranks present in that suit.
#[derive(Debug, Clone)]
pub struct Rank16Iter {
    c64: Card64,
    suit_idx: u8,
}

impl Rank16Iter {
    /// Creates a new `RanksIter` for the given Card64.
    pub(crate) const fn new(c64: Card64) -> Self {
        Self { c64, suit_idx: 0 }
    }
}

impl Iterator for Rank16Iter {
    type Item = (Rank16, Suit);

    fn next(&mut self) -> Option<Self::Item> {
        let suit = match self.suit_idx {
            0 => Suit::S,
            1 => Suit::H,
            2 => Suit::D,
            3 => Suit::C,
            _ => return None,
        };

        self.suit_idx += 1;

        Some((self.c64.ranks_by_suit(suit), suit))
    }
}
