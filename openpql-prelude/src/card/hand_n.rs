#[cfg(feature = "serde")]
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{self, Visitor},
};
#[cfg(feature = "speedy")]
use speedy::{Context, Readable, Reader, Writable, Writer};

use super::{Card, Card64, Deref, HandIter, Hash, Index, Into, fmt};

/// Fixed-size hand representation.
///
/// Represents exactly N cards. Cards are stored sorted and deduplicated.
#[derive(
    Copy,
    Clone,
    derive_more::Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Into,
    Deref,
    Index,
)]
#[debug("Hand<{}>({})", N, self)]
pub struct HandN<const N: usize>(pub(crate) [Card; N]);

impl<const N: usize> HandN<N> {
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub(crate) fn new(array: [Card; N]) -> Self {
        debug_assert!(N > 1, "HandN should have at least two cards.");
        debug_assert!(
            array.is_sorted(),
            "Hand initialized from unsorted array {array:?}"
        );
        Self(array)
    }

    /// Creates a sorted hand from a slice.
    #[cfg_attr(coverage_nightly, coverage(off))]
    pub fn from_slice(cs: &[Card]) -> Self {
        debug_assert!(
            cs.len() >= N,
            "from_slice: not enough cards for Hand<{}> slice has {} elements",
            N,
            cs.len()
        );

        let mut cards = [Card::default(); N];
        cards.copy_from_slice(&cs[..N]);
        cards.sort_unstable();

        Self(cards)
    }

    /// Returns an iterator over all possible N-card hands (card combination)
    pub fn iter_all<const SD: bool>() -> HandIter<SD, N> {
        HandIter::default()
    }
}

impl<const N: usize> fmt::Display for HandN<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.iter().try_for_each(|card| write!(f, "{card}"))
    }
}

impl<const N: usize> From<HandN<N>> for Card64 {
    fn from(hand: HandN<N>) -> Self {
        hand.iter().copied().collect()
    }
}

#[cfg(feature = "serde")]
impl<const N: usize> Serialize for HandN<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeTuple;
        let mut seq = serializer.serialize_tuple(N)?;
        for card in &self.0 {
            seq.serialize_element(card)?;
        }
        seq.end()
    }
}

#[cfg(feature = "serde")]
impl<'de, const N: usize> Deserialize<'de> for HandN<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use std::{fmt, marker::PhantomData};

        struct HandNVisitor<const N: usize>(PhantomData<[Card; N]>);

        impl<'de, const N: usize> Visitor<'de> for HandNVisitor<N> {
            type Value = HandN<N>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "a hand of {N} cards")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let mut array = [Card::default(); N];
                let mut i = 0;

                while let Some(card) = seq.next_element::<Card>()? {
                    array[i] = card;
                    i += 1;
                }

                if i != N {
                    return Err(de::Error::custom(format!(
                        "expected {N} cards, got {i}",
                    )));
                }

                Ok(HandN(array))
            }
        }

        deserializer.deserialize_tuple(N, HandNVisitor(PhantomData))
    }
}

#[cfg(feature = "speedy")]
impl<'a, C: Context, const N: usize> Readable<'a, C> for HandN<N>
where
    Card: Readable<'a, C>,
{
    fn read_from<R: Reader<'a, C>>(reader: &mut R) -> Result<Self, C::Error> {
        let mut cards = [Card::default(); N];
        for card in &mut cards {
            *card = reader.read_value()?;
        }
        Ok(Self(cards))
    }
}

#[cfg(feature = "speedy")]
impl<C: Context, const N: usize> Writable<C> for HandN<N>
where
    Card: Writable<C>,
{
    fn write_to<W: Writer<C> + ?Sized>(
        &self,
        writer: &mut W,
    ) -> Result<(), C::Error> {
        for card in &self.0 {
            writer.write_value(card)?;
        }
        Ok(())
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    #[should_panic(expected = "Hand initialized from unsorted array")]
    #[cfg(debug_assertions)]
    fn test_new_err() {
        let unsorted = [
            Card::from_str("Ad").unwrap(),
            Card::from_str("2s").unwrap(),
            Card::from_str("Kc").unwrap(),
        ];

        let _hand = HandN::new(unsorted);
    }

    #[test]
    #[should_panic(expected = "not enough cards for Hand")]
    #[cfg(debug_assertions)]
    fn test_from_slice_err() {
        let cards = cards!("2s Kc");

        let _hand: HandN<3> = HandN::from_slice(&cards);
    }

    #[test]
    fn test_from_slice() {
        let cards = cards!("2s Kc Ad Kh");
        let [c1, c2, c3, c4] = cards.clone().try_into().unwrap();

        assert_eq!(HandN::<3>::from_slice(&cards).0, [c1, c2, c3]);
        assert_eq!(HandN::<4>::from_slice(&cards).0, [c1, c4, c2, c3]);
    }

    #[test]
    fn test_display() {
        let hand: HandN<3> = HandN::<3>::from_slice(&cards!("2s Ad Kc"));

        assert_eq!(format!("{hand}"), "2sKcAd");
        assert_eq!(format!("{hand:?}"), "Hand<3>(2sKcAd)");
    }

    #[test]
    fn test_ord() {
        let unsorted_cards = cards!("Ad Kc 2s");
        let hand: HandN<3> = HandN::from_slice(&unsorted_cards);

        assert_eq!(hand[0], cards!("2s")[0]);
        assert_eq!(hand[1], cards!("Kc")[0]);
        assert_eq!(hand[2], cards!("Ad")[0]);
    }

    #[quickcheck]
    fn test_to_card64(cards: CardN<7>) {
        let cs: Vec<_> = cards.into_iter().collect();
        let c64 = Card64::from(cs.as_slice());
        let hand = HandN::<7>::from_slice(&cs);

        assert_eq!(c64, Card64::from(hand));
    }
}

#[cfg(all(test, feature = "serde"))]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests_serde {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_hand_ser_de(cards: CardN<3>) {
        let mut cards = cards.to_vec();
        cards.sort_unstable();
        let hand: HandN<3> = HandN::from_slice(&cards);

        assert_tokens(
            &hand.readable(),
            &[
                Token::Tuple { len: 3 },
                Token::Str(to_s(cards[0])),
                Token::Str(to_s(cards[1])),
                Token::Str(to_s(cards[2])),
                Token::TupleEnd,
            ],
        );

        assert_tokens(
            &hand.compact(),
            &[
                Token::Tuple { len: 3 },
                Token::U8(to_i(cards[0])),
                Token::U8(to_i(cards[1])),
                Token::U8(to_i(cards[2])),
                Token::TupleEnd,
            ],
        );
    }
}
