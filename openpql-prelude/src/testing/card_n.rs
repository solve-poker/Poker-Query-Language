use derive_more::{Deref, Index, Into, IntoIterator};

use crate::{Card, Card64};

/// Fixed array of `N` cards, short-deck when `SD` is true.
#[derive(Debug, Clone, Index, derive_more::From, Into, Deref, IntoIterator)]
pub struct CardN<const N: usize, const SD: bool = false>([Card; N]);

impl<const N: usize, const S: bool> quickcheck::Arbitrary for CardN<N, S> {
    fn arbitrary(_g: &mut quickcheck::Gen) -> Self {
        let mut rng = fastrand::Rng::new();

        let v = rng.choose_multiple(Card::all::<S>(), N);

        <[_; N]>::try_from(v).unwrap().map(|c| *c).into()
    }
}

impl<const N: usize, const S: bool> From<CardN<N, S>> for Card64 {
    fn from(cards: CardN<N, S>) -> Self {
        cards.as_ref().into()
    }
}

#[cfg_attr(coverage_nightly, coverage(off))]
#[allow(clippy::fallible_impl_from)]
impl<const X: usize, const Y: usize, const Z: usize, const S: bool>
    From<CardN<Z, S>> for (CardN<X, S>, CardN<Y, S>)
{
    fn from(cards: CardN<Z, S>) -> Self {
        assert!(X + Y <= Z, "Not enough cards {Z} < {X} + {Y} = {}", X + Y);

        let mut x: [_; X] = [Card::default(); X];
        let mut y: [_; Y] = [Card::default(); Y];

        for i in 0..X {
            x[i] = cards[i];
        }

        for j in 0..Y {
            y[j] = cards[X + j];
        }

        (x.into(), y.into())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_cards_n_destruct(cards: CardN<10>) -> TestResult {
        let (c3, c7): (CardN<3>, CardN<7>) = cards.clone().into();

        TestResult::from_bool(
            c3.as_slice() == &cards[..3] && c7.as_slice() == &cards[3..],
        )
    }

    #[test]
    #[should_panic(expected = "Not enough cards 3 < 3 + 7")]
    fn test_cards_n_destruct_err() {
        let _: (CardN<3>, CardN<7>) = CardN(<[_; 3]>::default()).into();
    }

    #[quickcheck]
    fn test_cards_n_distinct(cards: CardN<7>) -> TestResult {
        let v: FxHashSet<_> = cards.into_iter().collect();

        TestResult::from_bool(v.len() == 7)
    }
}
