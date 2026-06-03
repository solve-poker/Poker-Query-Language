use std::ops::{Index, IndexMut};

use crate::{FlushingSuit, IsomorphicCard, IsomorphicHandN, Rank};

const N_CARDS: usize = 2;
type Hand = IsomorphicHandN<N_CARDS>;

/// A poker hand chart: one `T` per distinct preflop hand.
///
/// `SD` selects the deck — `false` for the 13-rank deck (`13 x 13`, 169 hands),
/// `true` for the 9-rank short deck (`9 x 9`, 81 hands). The diagonal holds
/// pocket pairs, the upper triangle suited hands, the lower triangle offsuit.
/// Index by [`IsomorphicHandN`], or visit cells with [`Self::iter`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HandMatrix<T, const SD: bool = false> {
    inner: Box<[T]>,
}

impl<T, const SD: bool> HandMatrix<T, SD> {
    /// Builds a matrix by applying `proc` to every preflop hand.
    pub fn new(proc: &dyn Fn(Hand) -> T) -> Self {
        let n = n::<SD>();
        Self {
            inner: (0..n * n).map(|i| proc(hand_at::<SD>(i))).collect(),
        }
    }

    /// Iterates over every `(hand, &value)` pair in row-major order.
    pub fn iter(&self) -> impl Iterator<Item = (Hand, &T)> {
        self.inner
            .iter()
            .enumerate()
            .map(|(i, value)| (hand_at::<SD>(i), value))
    }

    /// Iterates over every `(hand, &mut value)` pair in row-major order.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Hand, &mut T)> {
        self.inner
            .iter_mut()
            .enumerate()
            .map(|(i, value)| (hand_at::<SD>(i), value))
    }
}

/// Grid side length (rank count) for the selected deck.
const fn n<const SD: bool>() -> usize {
    Rank::all::<SD>().len()
}

const fn hand_at<const SD: bool>(i: usize) -> Hand {
    let ranks = Rank::all::<SD>();
    let n = ranks.len();
    let (row, col) = (i / n, i % n);

    let (lo, hi, suit) = if row < col {
        (row, col, FlushingSuit::X)
    } else {
        (col, row, FlushingSuit::N)
    };

    IsomorphicHandN([
        IsomorphicCard::new(ranks[lo], suit),
        IsomorphicCard::new(ranks[hi], suit),
    ])
}

const fn index_of<const SD: bool>(hand: Hand) -> usize {
    let ranks = Rank::all::<SD>();
    let n = ranks.len();
    let base = ranks[0] as usize;

    let (r0, r1) = (
        hand.0[0].rank as usize - base,
        hand.0[1].rank as usize - base,
    );
    let (lo, hi) = if r0 <= r1 { (r0, r1) } else { (r1, r0) };

    match hand.0[0].suit {
        FlushingSuit::X => lo * n + hi,
        _ => hi * n + lo,
    }
}

impl<T, const SD: bool> Index<Hand> for HandMatrix<T, SD> {
    type Output = T;

    fn index(&self, hand: Hand) -> &T {
        &self.inner[index_of::<SD>(hand)]
    }
}

impl<T, const SD: bool> IndexMut<Hand> for HandMatrix<T, SD> {
    fn index_mut(&mut self, hand: Hand) -> &mut T {
        &mut self.inner[index_of::<SD>(hand)]
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;

    const N: usize = n::<false>();

    fn suited(lo: Rank, hi: Rank) -> Hand {
        IsomorphicHandN([
            IsomorphicCard::new(lo, FlushingSuit::X),
            IsomorphicCard::new(hi, FlushingSuit::X),
        ])
    }

    fn offsuit(lo: Rank, hi: Rank) -> Hand {
        IsomorphicHandN([
            IsomorphicCard::new(lo, FlushingSuit::N),
            IsomorphicCard::new(hi, FlushingSuit::N),
        ])
    }

    #[test]
    fn test_round_trip_every_cell() {
        for i in 0..N * N {
            assert_eq!(index_of::<false>(hand_at::<false>(i)), i);
        }
        let m = n::<true>();
        for i in 0..m * m {
            assert_eq!(index_of::<true>(hand_at::<true>(i)), i);
        }
    }

    #[test]
    fn test_layout() {
        const A: usize = 12;
        const K: usize = 11;
        assert_eq!(index_of::<false>(offsuit(Rank::RA, Rank::RA)), A * N + A);
        assert_eq!(index_of::<false>(suited(Rank::RK, Rank::RA)), K * N + A);
        assert_eq!(index_of::<false>(offsuit(Rank::RK, Rank::RA)), A * N + K);
    }

    #[test]
    fn test_layout_shortdeck() {
        const A: usize = 8;
        const K: usize = 7;
        let m = n::<true>();
        assert_eq!(m, 9);
        assert_eq!(index_of::<true>(offsuit(Rank::RA, Rank::RA)), A * m + A);
        assert_eq!(index_of::<true>(suited(Rank::RK, Rank::RA)), K * m + A);
        assert_eq!(index_of::<true>(offsuit(Rank::RK, Rank::RA)), A * m + K);
        assert_eq!(index_of::<true>(offsuit(Rank::R6, Rank::R6)), 0);
    }

    #[test]
    fn test_index_order_independent() {
        assert_eq!(
            index_of::<false>(suited(Rank::RK, Rank::RA)),
            index_of::<false>(suited(Rank::RA, Rank::RK)),
        );
    }

    #[test]
    fn test_new_and_index() {
        let matrix = HandMatrix::<_>::new(&|hand: Hand| hand.to_string());
        assert_eq!(matrix[suited(Rank::RK, Rank::RA)], "KxAx");
        assert_eq!(matrix[offsuit(Rank::RA, Rank::RA)], "AnAn");

        let sd = HandMatrix::<_, true>::new(&|hand: Hand| hand.to_string());
        assert_eq!(sd[suited(Rank::RK, Rank::RA)], "KxAx");
        assert_eq!(sd[offsuit(Rank::R6, Rank::R6)], "6n6n");
    }

    #[test]
    fn test_index_mut() {
        let mut matrix = HandMatrix::<_>::new(&|_| 0);
        matrix[suited(Rank::R2, Rank::R7)] = 42;
        assert_eq!(matrix[suited(Rank::R2, Rank::R7)], 42);
        assert_eq!(matrix[offsuit(Rank::R2, Rank::R7)], 0);
    }

    #[test]
    fn test_iter_visits_every_distinct_hand() {
        let matrix = HandMatrix::<_>::new(&|hand: Hand| hand);
        assert_eq!(matrix.iter().count(), N * N);

        for (hand, value) in matrix.iter() {
            assert_eq!(hand, *value);
            assert_eq!(matrix[hand], hand);
        }

        let sd = HandMatrix::<_, true>::new(&|hand: Hand| hand);
        assert_eq!(sd.iter().count(), n::<true>() * n::<true>());
        for (hand, value) in sd.iter() {
            assert_eq!(hand, *value);
            assert_eq!(sd[hand], hand);
        }
    }

    #[test]
    fn test_iter_mut() {
        let mut matrix = HandMatrix::<usize>::new(&|_| 0);
        for (i, (_, value)) in matrix.iter_mut().enumerate() {
            *value = i;
        }
        assert_eq!(matrix.iter().map(|(_, v)| *v).sum::<usize>(), {
            let n = N * N;
            n * (n - 1) / 2
        });
    }
}
