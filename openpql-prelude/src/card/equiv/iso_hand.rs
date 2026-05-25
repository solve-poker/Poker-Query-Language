use std::{fmt, hash::Hash};

#[cfg(feature = "serde")]
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{self, Visitor},
};
#[cfg(feature = "speedy")]
use speedy::{Context, Readable, Reader, Writable, Writer};

use crate::{Card, IsomorphicCard, SuitMap};

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

        struct IsomorphicHandNVisitor<const N: usize>(
            PhantomData<[IsomorphicCard; N]>,
        );

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
