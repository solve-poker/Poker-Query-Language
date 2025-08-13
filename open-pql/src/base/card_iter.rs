use super::{Card, Card64, N_CARDS};

/// Iterator over individual cards in a Card64 set.
///
/// This iterator yields each Card that is present in the Card64.
#[derive(Debug, Clone)]
pub struct CardIter {
    c64: Card64,
    idx: u8,
}

impl CardIter {
    /// Creates a new `CardIter` for the given Card64.
    pub(crate) const fn new(c64: Card64) -> Self {
        Self { c64, idx: 0 }
    }
}

impl Iterator for CardIter {
    type Item = Card;

    fn next(&mut self) -> Option<Self::Item> {
        while self.idx < N_CARDS {
            let c = Card::ARR_ALL[self.idx as usize];
            self.idx += 1;

            if self.c64.contains_card(c) {
                return Some(c);
            }
        }

        None
    }
}
