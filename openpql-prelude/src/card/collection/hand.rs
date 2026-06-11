use std::{
    ops::{Deref, DerefMut, Index, IndexMut},
    slice,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use smallvec::SmallVec;

use crate::{
    Card, IsomorphicCard, IsomorphicHandN, SuitMap,
    card::equiv::{n_flush_suits, place_card},
};

/// Maximum number of hole cards stored inline.
pub const MAX_HOLECARDS: usize = 5;

type Inner<C> = SmallVec<[C; MAX_HOLECARDS]>;

/// A variable-length, sorted-on-demand hand of cards of type `C`.
#[cfg_attr(feature = "speedy", derive(speedy::Readable, speedy::Writable))] // LCOV_EXCL_LINE
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CardArray<C>(pub Inner<C>);

/// A variable-length hand of concrete [`Card`]s.
pub type Hand = CardArray<Card>;

/// A variable-length hand of suit-isomorphic [`IsomorphicCard`]s.
pub type IsomorphicHand = CardArray<IsomorphicCard>;

impl IsomorphicHand {
    /// Materializes this suit-isomorphic hand as a concrete [`Hand`] with
    /// suits placed in canonical order.
    #[must_use]
    pub fn to_hand(&self) -> Hand {
        let mut next = n_flush_suits(&self.0);

        self.0
            .iter()
            .map(|&c| {
                let (card, k) = place_card(c, next);
                next = k;
                card
            })
            .collect()
    }

    /// Builds a suit-isomorphic hand from `cards`, relabeling suits via
    /// `map`.
    #[must_use]
    pub fn from_slice_and_map(cards: &[Card], map: SuitMap) -> Self {
        match cards.len() {
            2 => Self::from_arr(
                IsomorphicHandN::<2>::from_slice_and_map(cards, map).0,
            ),
            4 => Self::from_arr(
                IsomorphicHandN::<4>::from_slice_and_map(cards, map).0,
            ),
            _ => unimplemented!(), // LCOV_EXCL_LINE
        }
    }
}

impl<C> CardArray<C> {
    /// Creates an empty hand without allocating.
    #[must_use]
    pub const fn new() -> Self {
        Self(SmallVec::new_const())
    }

    /// Appends `card` to the end of the hand.
    pub fn push(&mut self, card: C) {
        self.0.push(card);
    }

    /// Returns the cards as a slice.
    #[must_use]
    pub fn as_slice(&self) -> &[C] {
        &self.0
    }
}

impl<C> CardArray<C>
where
    C: Copy,
{
    #[must_use]
    #[inline]
    pub(crate) const fn from_arr<const N: usize>(arr: [C; N]) -> Self {
        const { assert!(N <= MAX_HOLECARDS, "from_arr: N must be <= MAX_HOLECARDS") };

        let inner = unsafe {
            match N {
                0 => SmallVec::new_const(),
                1 => SmallVec::from_const_with_len_unchecked(
                    [arr[0], arr[0], arr[0], arr[0], arr[0]],
                    1,
                ),
                2 => SmallVec::from_const_with_len_unchecked(
                    [arr[0], arr[1], arr[0], arr[0], arr[0]],
                    2,
                ),
                3 => SmallVec::from_const_with_len_unchecked(
                    [arr[0], arr[1], arr[2], arr[0], arr[0]],
                    3,
                ),
                4 => SmallVec::from_const_with_len_unchecked(
                    [arr[0], arr[1], arr[2], arr[3], arr[0]],
                    4,
                ),
                5 => SmallVec::from_const_with_len_unchecked(
                    [arr[0], arr[1], arr[2], arr[3], arr[4]],
                    5,
                ),
                _ => unreachable!(), // LCOV_EXCL_LINE
            }
        };

        Self(inner)
    }
}

impl<C: Ord> CardArray<C> {
    /// Collect `cards` into a hand sorted into canonical order, suitable for use
    /// as a lookup key.
    #[must_use]
    pub fn sorted(cards: impl IntoIterator<Item = C>) -> Self {
        let mut hand: Self = cards.into_iter().collect();
        hand.0.sort_unstable();
        hand
    }
}

impl<C> Deref for CardArray<C> {
    type Target = [C];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<C> DerefMut for CardArray<C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<C> Index<usize> for CardArray<C> {
    type Output = C;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }
}

impl<C> IndexMut<usize> for CardArray<C> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.0[idx]
    }
}

impl<C> From<Inner<C>> for CardArray<C> {
    fn from(inner: Inner<C>) -> Self {
        Self(inner)
    }
}

impl<C> From<Vec<C>> for CardArray<C> {
    fn from(cards: Vec<C>) -> Self {
        Self(SmallVec::from_vec(cards))
    }
}

impl<C: Copy, const N: usize> From<[C; N]> for CardArray<C> {
    fn from(cards: [C; N]) -> Self {
        Self(SmallVec::from_slice(&cards))
    }
}

impl<C> FromIterator<C> for CardArray<C> {
    fn from_iter<T: IntoIterator<Item = C>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<C> IntoIterator for CardArray<C> {
    type Item = C;
    type IntoIter = smallvec::IntoIter<[C; MAX_HOLECARDS]>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, C> IntoIterator for &'a CardArray<C> {
    type Item = &'a C;
    type IntoIter = slice::Iter<'a, C>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

#[cfg(feature = "serde")]
impl<C: Serialize> Serialize for CardArray<C> {
    fn serialize<S: Serializer>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        self.0.as_slice().serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, C: Deserialize<'de>> Deserialize<'de> for CardArray<C> {
    fn deserialize<D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        let cards = Vec::<C>::deserialize(deserializer)?;
        Ok(Self(SmallVec::from_vec(cards)))
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use std::iter::once;

    use super::*;
    use crate::{CardN, FlushingSuit, cards, isocard, isocards};

    #[quickcheck]
    fn test_from_arr(cards: CardN<5>) {
        fn assert_eq<const N: usize>(cs: &[Card]) {
            let arr = <[Card; N]>::try_from(&cs[0..N]).unwrap();

            assert_eq!(Hand::from(arr), Hand::from_arr(arr));
        }

        assert_eq::<0>(cards.as_slice());
        assert_eq::<1>(cards.as_slice());
        assert_eq::<2>(cards.as_slice());
        assert_eq::<3>(cards.as_slice());
        assert_eq::<4>(cards.as_slice());
        assert_eq::<5>(cards.as_slice());
    }

    #[test]
    fn new_push_and_accessors() {
        let mut hand = IsomorphicHand::new();
        assert!(hand.as_slice().is_empty());

        hand.push(isocard!("Ax"));
        hand.push(isocard!("Kx"));
        assert_eq!(hand.as_slice(), &[isocard!("Ax"), isocard!("Kx")]);
        assert_eq!(hand.len(), 2);
        assert_eq!(hand[0], isocard!("Ax"));

        hand[1] = isocard!("Qx");
        assert_eq!(hand[1], isocard!("Qx"));
        hand.deref_mut()[0] = isocard!("Jx");
        assert_eq!(hand.deref()[0], isocard!("Jx"));
    }

    #[test]
    fn test_from_slice_and_map() {
        use FlushingSuit::*;
        let map = SuitMap([X, Y, N, N]);
        assert_eq!(
            IsomorphicHand::from_slice_and_map(&cards!("AsKhQdJc"), map),
            isocards!("JnQnKyAx").into(),
        );
    }

    #[test]
    fn conversions() {
        let inner: Inner<IsomorphicCard> =
            SmallVec::from_slice(&[isocard!("Ax")]);
        let from_inner = IsomorphicHand::from(inner);
        assert_eq!(from_inner.as_slice(), &[isocard!("Ax")]);

        let from_vec =
            IsomorphicHand::from(vec![isocard!("Ax"), isocard!("Kx")]);
        assert_eq!(from_vec.len(), 2);

        let from_arr = IsomorphicHand::from([isocard!("Ax"), isocard!("Kx")]);
        assert_eq!(from_arr.len(), 2);

        let collected: IsomorphicHand = once(isocard!("Ax")).collect();
        assert_eq!(collected.len(), 1);
    }

    #[test]
    fn iteration() {
        let hand = IsomorphicHand::from([isocard!("Ax"), isocard!("Kx")]);

        let by_ref: Vec<_> = (&hand).into_iter().copied().collect();
        assert_eq!(by_ref, vec![isocard!("Ax"), isocard!("Kx")]);

        let owned: Vec<_> = hand.into_iter().collect();
        assert_eq!(owned, vec![isocard!("Ax"), isocard!("Kx")]);
    }

    #[test]
    fn hand_over_concrete_cards() {
        let mut hand = Hand::new();
        for c in cards!("KsAs") {
            hand.push(c);
        }
        let sorted = Hand::sorted(hand);
        assert!(sorted.as_slice().is_sorted());
        assert_eq!(sorted.len(), 2);
    }
}

#[cfg(all(test, feature = "serde"))]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests_serde {
    use serde_test::{Token, assert_de_tokens_error, assert_tokens};

    use super::*;
    use crate::isocard;

    #[test]
    fn serde_round_trip() {
        let hand = IsomorphicHand::from([isocard!("Ax"), isocard!("Kx")]);
        assert_tokens(
            &hand,
            &[
                Token::Seq { len: Some(2) },
                Token::Str("Ax"),
                Token::Str("Kx"),
                Token::SeqEnd,
            ],
        );
    }

    #[test]
    fn deserialize_propagates_error() {
        assert_de_tokens_error::<IsomorphicHand>(
            &[Token::Seq { len: Some(1) }, Token::Bool(true)],
            "invalid type: boolean `true`, expected an isomorphic card string like \"Ah\"",
        );
    }
}
