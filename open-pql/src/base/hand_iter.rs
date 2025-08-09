use super::{Card, HandN};

#[derive(Debug, Clone)]
pub struct HandIter<const SD: bool, const N: usize> {
    indices: [u8; N],
    done: bool,
}

impl<const SD: bool, const N: usize> HandIter<SD, N> {
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
mod tests {
    use super::*;

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
    fn test_hand_iter_shortdeck() {
        let iter = HandN::<2>::iter_all_short();

        let expected = 36 * 35 / 2;
        assert_eq!(iter.count(), expected);
    }
}
