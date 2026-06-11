use std::{mem::MaybeUninit, ops, ptr};

use rayon::{
    iter::plumbing::{Consumer, Producer, ProducerCallback, UnindexedConsumer, bridge},
    prelude::*,
};

use crate::{Card, HandIter, HandN};

/// Parallel iterator over every `N`-card combination, short-deck when `SD` is true.
pub struct HandParIter<const SD: bool, const N: usize> {
    range: ops::Range<usize>,
    cards: [Card; Card::N_CARDS as usize],
    n_cards: usize,
}

impl<const SD: bool, const N: usize> IntoParallelIterator for HandIter<SD, N> {
    type Item = HandN<N>;
    type Iter = HandParIter<SD, N>;

    fn into_par_iter(self) -> Self::Iter {
        let mut cards = [Card::default(); Card::N_CARDS as usize];
        let mut n_cards = 0;
        for &c in Card::all::<SD>() {
            if !self.dead.contains_card(c) {
                cards[n_cards] = c;
                n_cards += 1;
            }
        }
        let len = super::hand_iter::ncr(n_cards, N);
        HandParIter {
            range: 0..len,
            cards,
            n_cards,
        }
    }
}

impl<const SD: bool, const N: usize> ParallelIterator for HandParIter<SD, N> {
    type Item = HandN<N>;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        self.drive(consumer)
    }

    fn opt_len(&self) -> Option<usize> {
        Some(self.range.len())
    }
}

impl<const SD: bool, const N: usize> IndexedParallelIterator for HandParIter<SD, N> {
    fn drive<C>(self, consumer: C) -> C::Result
    where
        C: Consumer<Self::Item>,
    {
        bridge(self, consumer)
    }

    fn len(&self) -> usize {
        self.range.len()
    }

    fn with_producer<CB>(self, callback: CB) -> CB::Output
    where
        CB: ProducerCallback<Self::Item>,
    {
        callback.callback(self)
    }
}

impl<const SD: bool, const N: usize> Producer for HandParIter<SD, N> {
    type Item = HandN<N>;
    type IntoIter = UnrankIter<N>;

    fn into_iter(self) -> Self::IntoIter {
        UnrankIter {
            range: self.range,
            cards: self.cards,
            n_cards: self.n_cards,
            ncr_cache: NcrCache::new(self.n_cards),
        }
    }

    fn split_at(self, index: usize) -> (Self, Self) {
        let mid = self.range.start + index;
        (
            Self {
                range: self.range.start..mid,
                cards: self.cards,
                n_cards: self.n_cards,
            },
            Self {
                range: mid..self.range.end,
                cards: self.cards,
                n_cards: self.n_cards,
            },
        )
    }
}

/// Sequential half of `HandParIter` that unranks combinations from an index range.
pub struct UnrankIter<const N: usize> {
    range: ops::Range<usize>,
    cards: [Card; Card::N_CARDS as usize],
    n_cards: usize,
    ncr_cache: NcrCache,
}

#[derive(Clone)]
struct NcrCache {
    // Pascal's triangle stored as a flat vector
    // triangle[i][j] is stored at index i*(i+1)/2 + j
    triangle: Vec<usize>,
    max_n: usize,
}

impl NcrCache {
    fn new(max_n: usize) -> Self {
        let size = (max_n + 1) * (max_n + 2) / 2;
        let mut triangle = vec![0; size];

        for n in 0..=max_n {
            let row_start = n * (n + 1) / 2;
            triangle[row_start] = 1; // C(n, 0) = 1

            for k in 1..=n {
                let idx = row_start + k;
                if k == n {
                    triangle[idx] = 1; // C(n, n) = 1
                } else {
                    let prev_row = (n - 1) * n / 2;
                    triangle[idx] = triangle[prev_row + k - 1] + triangle[prev_row + k];
                }
            }
        }

        Self { triangle, max_n }
    }

    #[inline]
    fn get(&self, n: usize, k: usize) -> usize {
        if k > n || n > self.max_n {
            return 0;
        }
        let idx = n * (n + 1) / 2 + k;
        self.triangle[idx]
    }
}

impl<const N: usize> Iterator for UnrankIter<N> {
    type Item = HandN<N>;

    fn next(&mut self) -> Option<Self::Item> {
        self.range.next().map(|rank| {
            unrank_combination_with_cards(
                rank,
                &self.cards[..self.n_cards],
                self.n_cards,
                &self.ncr_cache,
            )
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }
}

impl<const N: usize> DoubleEndedIterator for UnrankIter<N> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.range.next_back().map(|rank| {
            unrank_combination_with_cards(
                rank,
                &self.cards[..self.n_cards],
                self.n_cards,
                &self.ncr_cache,
            )
        })
    }
}

impl<const N: usize> ExactSizeIterator for UnrankIter<N> {
    fn len(&self) -> usize {
        self.range.len()
    }
}

#[inline]
fn unrank_combination_with_cards<const N: usize>(
    mut rank: usize,
    cards: &[Card],
    n: usize,
    cache: &NcrCache,
) -> HandN<N> {
    let mut result: [MaybeUninit<Card>; N] = unsafe { MaybeUninit::uninit().assume_init() };

    // Use combinatorial number system with binary search to find the combination
    let mut x = n - 1;
    for i in (0..N).rev() {
        let k = i + 1;

        // Binary search to find largest x where ncr(x, k) <= rank
        let mut left = k - 1;
        let mut right = x + 1;

        while left < right - 1 {
            let mid = left + (right - left) / 2;
            if cache.get(mid, k) <= rank {
                left = mid;
            } else {
                right = mid;
            }
        }

        x = left;
        result[i].write(cards[x]);
        rank -= cache.get(x, k);
    }

    // SAFETY: All elements have been initialized in the loop above
    let cards = unsafe { ptr::read((&raw const result).cast::<[Card; N]>()) };
    HandN::new(cards)
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::*;

    fn handiter_set<const N: usize, const SD: bool>() -> FxHashSet<HandN<N>> {
        HandN::<N>::iter_all::<SD>().collect()
    }

    fn pariter_set<const N: usize, const SD: bool>() -> FxHashSet<HandN<N>> {
        HandN::<N>::iter_all::<SD>().into_par_iter().collect()
    }

    #[test]
    fn test_parallel_iter_holdem() {
        const SD: bool = false;
        assert_eq!(handiter_set::<2, SD>(), pariter_set::<2, SD>());
        assert_eq!(handiter_set::<3, SD>(), pariter_set::<3, SD>());
    }

    #[test]
    fn test_parallel_iter_shortdeck() {
        const SD: bool = true;
        assert_eq!(handiter_set::<2, SD>(), pariter_set::<2, SD>());
        assert_eq!(handiter_set::<3, SD>(), pariter_set::<3, SD>());
    }

    #[test]
    fn test_unrank_iter_methods() {
        use rayon::iter::plumbing::Producer;

        let par = HandN::<2>::iter_all::<false>().into_par_iter();
        let mut iter = Producer::into_iter(par);

        assert_eq!(iter.len(), super::super::hand_iter::ncr(52, 2));
        assert_eq!(iter.size_hint(), (iter.len(), Some(iter.len())));

        let first = iter.next().unwrap();
        let last = iter.next_back().unwrap();
        assert_ne!(first, last);
    }

    #[test]
    fn test_parallel_iter_with_dead() {
        let dead = c64!("As Kd");
        let seq_hands: FxHashSet<HandN<2>> =
            HandN::<2>::iter_all::<false>().with_dead(dead).collect();
        let par_hands: FxHashSet<HandN<2>> = HandN::<2>::iter_all::<false>()
            .with_dead(dead)
            .into_par_iter()
            .collect();

        assert_eq!(seq_hands, par_hands);
        assert_eq!(par_hands.len(), super::super::hand_iter::ncr(50, 2));

        let as_card = Card::from_str("As").unwrap();
        let kd_card = Card::from_str("Kd").unwrap();
        for hand in par_hands {
            assert!(!hand.to_vec().contains(&as_card));
            assert!(!hand.to_vec().contains(&kd_card));
        }
    }
}
