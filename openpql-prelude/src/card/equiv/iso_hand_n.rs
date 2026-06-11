use std::{fmt, hash::Hash};

#[cfg(feature = "serde")]
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{self, Visitor},
};
#[cfg(feature = "speedy")]
use speedy::{Context, Readable, Reader, Writable, Writer};

use crate::{
    Card, IsomorphicCard, Rank, Suit, SuitMap,
    card::equiv::util::{n_flush_suits, place_card},
};

/// Sorted hand of exactly `N` cards.
#[derive(
    Copy,
    Clone,
    derive_more::Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    derive_more::Into,
    derive_more::Deref,
    derive_more::Index,
)]
#[debug("IsomorphicHandN<{}>({})", N, self)]
pub struct IsomorphicHandN<const N: usize>(pub [IsomorphicCard; N]);

#[cfg(feature = "serde")]
impl<const N: usize> Serialize for IsomorphicHandN<N> {
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
impl<'de, const N: usize> Deserialize<'de> for IsomorphicHandN<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use std::marker::PhantomData;

        struct IsomorphicHandNVisitor<const N: usize>(PhantomData<[IsomorphicCard; N]>);

        impl<'de, const N: usize> Visitor<'de> for IsomorphicHandNVisitor<N> {
            type Value = IsomorphicHandN<N>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "an isomorphic hand of {N} cards")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let mut array = [IsomorphicCard::default(); N];
                let mut i = 0;

                while let Some(card) = seq.next_element::<IsomorphicCard>()? {
                    array[i] = card;
                    i += 1;
                }

                if i != N {
                    return Err(de::Error::custom(format!(
                        "expected {N} isomorphic cards, got {i}",
                    )));
                }

                Ok(IsomorphicHandN(array))
            }
        }

        deserializer.deserialize_tuple(N, IsomorphicHandNVisitor(PhantomData))
    }
}

impl<const N: usize> IsomorphicHandN<N> {
    /// # Panics
    /// panics if slice doesn't contain enough cards
    #[must_use]
    pub fn from_slice_and_map(cards: &[Card], map: SuitMap) -> Self {
        debug_assert!(
            cards.len() >= N,
            "from_slice: not enough cards for Hand<{}> slice has {} elements",
            N,
            cards.len()
        );

        let arr: [Card; N] = cards.try_into().unwrap();

        let mut inner = arr.map(|c| map.iso_card(c));
        inner.sort_unstable();

        Self(inner)
    }

    /// Materializes this representative as a concrete card array with placed suits.
    #[must_use]
    pub const fn to_array(self) -> [Card; N] {
        let mut out = [Card::new(Rank::R2, Suit::S); N];
        let mut k = n_flush_suits(&self.0);
        let mut i = 0;

        while i < N {
            let (card, next) = place_card(self.0[i], k);
            out[i] = card;
            k = next;
            i += 1;
        }

        out
    }
}

impl<const N: usize> fmt::Display for IsomorphicHandN<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.iter().try_for_each(|card| write!(f, "{card}"))
    }
}

#[cfg(feature = "speedy")]
impl<'a, C: Context, const N: usize> Readable<'a, C> for IsomorphicHandN<N>
where
    IsomorphicCard: Readable<'a, C>,
{
    fn read_from<R: Reader<'a, C>>(reader: &mut R) -> Result<Self, C::Error> {
        let mut cards = [IsomorphicCard::default(); N];
        for card in &mut cards {
            *card = reader.read_value()?;
        }
        Ok(Self(cards))
    }
}

#[cfg(feature = "speedy")]
impl<C: Context, const N: usize> Writable<C> for IsomorphicHandN<N>
where
    IsomorphicCard: Writable<C>,
{
    fn write_to<W: Writer<C> + ?Sized>(&self, writer: &mut W) -> Result<(), C::Error> {
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

    fn mk_map() -> SuitMap {
        board!("AsKhQd").to_isomorphic().1
    }

    #[test]
    fn test_from_slice_and_map() {
        let cards = cards!("AsKhQd");
        let hand = IsomorphicHandN::<3>::from_slice_and_map(&cards, mk_map());
        assert!(hand.0.is_sorted());
    }

    #[test]
    #[should_panic(expected = "from_slice: not enough cards for Hand")]
    #[cfg(debug_assertions)]
    fn test_from_slice_and_map_too_few() {
        let cards = cards!("As");
        let _ = IsomorphicHandN::<3>::from_slice_and_map(&cards, mk_map());
    }

    #[test]
    fn test_display_and_debug() {
        let cards = cards!("AsKhQd");
        let hand = IsomorphicHandN::<3>::from_slice_and_map(&cards, mk_map());
        let display = hand.to_string();
        assert_eq!(display.len(), 6);
        assert!(format!("{hand:?}").starts_with("IsomorphicHandN<3>("));
    }

    #[test]
    fn test_to_array_roundtrip_preflop() {
        for cs in HandN::<2>::iter_all::<false>() {
            let iso = IsomorphicHandN::<2>::from_slice_preflop(cs.as_slice());
            let back = IsomorphicHandN::<2>::from_slice_preflop(&iso.to_array());
            assert_eq!(back, iso, "{cs:?}: {iso} -> {:?}", iso.to_array());
        }
    }

    #[test]
    fn test_into_and_index_and_deref() {
        let cards = cards!("AsKhQd");
        let hand = IsomorphicHandN::<3>::from_slice_and_map(&cards, mk_map());
        let arr: [IsomorphicCard; 3] = hand.into();
        assert_eq!(arr[0], hand[0]);
        assert_eq!(hand.len(), 3);
    }
}

#[cfg(all(test, feature = "serde"))]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests_serde {
    use super::*;
    use crate::*;

    fn mk_hand() -> IsomorphicHandN<3> {
        let cards = cards!("AsKhQd");
        let map = board!("AsKhQd").to_isomorphic().1;
        IsomorphicHandN::<3>::from_slice_and_map(&cards, map)
    }

    #[test]
    fn test_iso_hand_ser_de() {
        let hand = mk_hand();
        let s0 = hand[0].to_string();
        let s1 = hand[1].to_string();
        let s2 = hand[2].to_string();

        assert_tokens(
            &hand,
            &[
                Token::Tuple { len: 3 },
                Token::Str(Box::leak(s0.into_boxed_str())),
                Token::Str(Box::leak(s1.into_boxed_str())),
                Token::Str(Box::leak(s2.into_boxed_str())),
                Token::TupleEnd,
            ],
        );
    }

    #[test]
    fn test_iso_hand_de_short_seq_err() {
        assert_de_tokens_error::<IsomorphicHandN<3>>(
            &[
                Token::Tuple { len: 2 },
                Token::Str("Ax"),
                Token::Str("Ky"),
                Token::TupleEnd,
            ],
            "expected 3 isomorphic cards, got 2",
        );
    }

    #[test]
    fn test_iso_hand_de_unexpected_type() {
        assert_de_tokens_error::<IsomorphicHandN<3>>(
            &[Token::Bool(true)],
            "invalid type: boolean `true`, expected an isomorphic hand of 3 cards",
        );
    }
}

#[cfg(all(test, feature = "speedy"))]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests_speedy {
    use speedy::{Readable, Writable};

    use super::*;
    use crate::*;

    #[test]
    fn test_iso_hand_speedy_roundtrip() {
        let cards = cards!("AsKhQd");
        let map = board!("AsKhQd").to_isomorphic().1;
        let hand = IsomorphicHandN::<3>::from_slice_and_map(&cards, map);
        let bytes = hand.write_to_vec().unwrap();
        let back = IsomorphicHandN::<3>::read_from_buffer(&bytes).unwrap();
        assert_eq!(hand, back);
    }
}
