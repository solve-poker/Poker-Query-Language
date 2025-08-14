use super::{Card, HandN};

/// Iterator for generating all possible combinations of N cards
///
/// Generates all unique combinations of N cards from either the full deck (52 cards)
/// or the short deck (36 cards, 6-A only). The generic parameter SD determines
/// which deck to use: true for short deck, false for full deck.
///
/// # Examples
///
/// ```
/// use open_pql::{HandIter, HandN};
///
/// let mut iter = HandN::<2>::iter_all();
/// let first_hand = iter.next().unwrap();
/// assert_eq!(first_hand.len(), 2);
/// ```
#[derive(Debug, Clone)]
pub struct HandIter<const SD: bool, const N: usize> {
    indices: [u8; N],
    done: bool,
}

impl<const SD: bool, const N: usize> HandIter<SD, N> {
    /// Returns the array of all cards for the current deck type
    ///
    /// Returns either all 52 cards or the 36-card short deck (6-A only)
    /// depending on the SD generic parameter.
    const fn all_cards() -> &'static [Card] {
        if SD {
            Card::ARR_ALL_SHORT.as_slice()
        } else {
            Card::ARR_ALL.as_slice()
        }
    }
}

#[allow(clippy::cast_possible_truncation)]
impl<const SD: bool, const N: usize> Default for HandIter<SD, N> {
    /// Creates a new `HandIter` starting with the first combination
    fn default() -> Self {
        let mut indices = [0; N];
        for i in 0..N as u8 {
            indices[i as usize] = i;
        }
        Self {
            indices,
            done: N == 0,
        }
    }
}

impl<const SD: bool, const N: usize> Iterator for HandIter<SD, N> {
    type Item = HandN<N>;

    /// Generates the next unique combination of N cards
    ///
    /// Returns `Some(HandN<N>)` containing the next combination, or `None`
    /// when all combinations have been generated. Uses a combination algorithm
    /// to ensure each hand is unique and generated in lexicographic order.
    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let all = Self::all_cards();
        let max_i = all.len();

        let mut cards = [Card::default(); N];
        for i in 0..N {
            cards[i] = all[self.indices[i] as usize];
        }

        let mut pos = N - 1;
        self.indices[pos] += 1;

        while self.indices[pos] as usize >= max_i - (N - 1 - pos) {
            if pos == 0 {
                self.done = true;
                return Some(HandN::new(cards));
            }

            pos -= 1;
            self.indices[pos] += 1;
        }

        for i in (pos + 1)..N {
            self.indices[i] = self.indices[i - 1] + 1;
        }

        Some(HandN::new(cards))
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_hand_iter_empty() {
        let mut iter = HandIter::<false, 0>::default();
        assert!(
            iter.next().is_none(),
            "Empty hand iterator should return None"
        );
    }

    #[test]
    fn test_hand_iter_single_card() {
        let mut iter = HandN::<1>::iter_all();

        for i in 0..Card::ARR_ALL.len() {
            assert_eq!(iter.next(), Some([Card::ARR_ALL[i]].into()));
        }
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_hand_iter_holdem() {
        let iter = HandN::<2>::iter_all();

        let expected = 52 * 51 / 2;
        assert_eq!(iter.count(), expected);
    }

    #[test]
    fn test_hand_iter_shortdeck() {
        let iter = HandN::<2>::iter_all_short();

        let expected = 36 * 35 / 2;
        assert_eq!(iter.count(), expected);
    }
}
