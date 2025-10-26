use super::{Card, CardCount, HandN};

/// Iterator for generating all possible hands of N cards.
///
/// Iterates through all possible combinations of N cards from either a standard 52-card deck
/// or a short deck (36 cards).
#[derive(Debug, Clone)]
pub struct HandIter<const SD: bool, const N: usize> {
    indices: [CardCount; N],
    done: bool,
}

#[allow(clippy::cast_possible_truncation)]
impl<const SD: bool, const N: usize> Default for HandIter<SD, N> {
    fn default() -> Self {
        let mut indices = [0; N];
        for i in 0..N as CardCount {
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

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let all = Card::all::<SD>();
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
    use crate::*;

    fn handiter_vec<const N: usize, const SD: bool>() -> Vec<Vec<Card>> {
        HandN::<N>::iter_all::<SD>()
            .map(|hand| hand.to_vec())
            .collect()
    }

    fn itertool_vec<const N: usize, const SD: bool>() -> Vec<Vec<Card>> {
        Card::all::<SD>().iter().copied().combinations(N).collect()
    }

    #[test]
    fn test_hand_iter_holdem() {
        const SD: bool = false;
        assert_eq!(handiter_vec::<2, SD>(), itertool_vec::<2, SD>());
        assert_eq!(handiter_vec::<3, SD>(), itertool_vec::<3, SD>());
    }

    #[test]
    fn test_hand_iter_shortdeck() {
        const SD: bool = true;
        assert_eq!(handiter_vec::<2, SD>(), itertool_vec::<2, SD>());
        assert_eq!(handiter_vec::<3, SD>(), itertool_vec::<3, SD>());
    }
}
