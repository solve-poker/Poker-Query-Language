use super::{Card, Card64, CardCount};

/// Iterator over cards in a Card64 bitset.
///
/// Iterates through all cards set in a Card64 bitset.
/// Produces cards grouped by rank across suits: 2s2h2c2d, then 3s3h3c3d, etc.
#[derive(Debug, Clone)]
pub struct CardIter {
    c64: Card64,
    idx: CardCount, // need this for column-major iter
}

impl CardIter {
    pub(crate) const fn new(c64: Card64) -> Self {
        Self { c64, idx: 0 }
    }
}

impl Iterator for CardIter {
    type Item = Card;

    fn next(&mut self) -> Option<Self::Item> {
        while self.idx < Card::N_CARDS {
            let c = Card::all::<false>()[self.idx as usize];
            self.idx += 1;

            if self.c64.contains_card(c) {
                return Some(c);
            }
        }

        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let count = self.c64.count() as usize;
        (count, Some(count))
    }
}

impl ExactSizeIterator for CardIter {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order() {
        let iter = CardIter::new(Card64::all::<false>());
        let cards: Vec<_> = iter.collect();

        assert_eq!(cards, Card::all::<false>());
    }
}
