#[cfg(feature = "serde")]
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{self, Visitor},
};

use super::{
    Card64, CardCount, Display, FromStr, Hash, ParseError, Rank, Suit,
};

#[macro_export]
macro_rules! card {
    ($s:expr) => {
        $s.parse::<$crate::Card>().unwrap()
    };
}

#[macro_export]
macro_rules! cards {
    ($s:expr) => {{
        let s: &str = $s;
        let mut cards = Vec::new();
        let mut chars = s.chars().filter(|c| !c.is_whitespace());
        while let (Some(r), Some(s)) = (chars.next(), chars.next()) {
            cards.push($crate::card![format!("{r}{s}")]);
        }
        cards
    }};
}

/// Playing card representation.
///
/// Represents a single playing card with a rank and suit, with macros for convenient creation.
#[cfg_attr(feature = "speedy", derive(speedy::Readable, speedy::Writable))]
#[derive(Clone, Copy, Debug, Display, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[display("{rank}{suit}")]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    /// Total number of cards in a standard deck
    pub const N_CARDS: CardCount = Suit::N_SUITS * Rank::N_RANKS;
    /// Total number of cards in a short deck
    pub const N_CARDS_SD: CardCount = Suit::N_SUITS * Rank::N_RANKS_SD;

    const ARR_ALL: [Self; Self::N_CARDS as usize] = [
        Self::new(Rank::R2, Suit::S),
        Self::new(Rank::R2, Suit::H),
        Self::new(Rank::R2, Suit::D),
        Self::new(Rank::R2, Suit::C),
        Self::new(Rank::R3, Suit::S),
        Self::new(Rank::R3, Suit::H),
        Self::new(Rank::R3, Suit::D),
        Self::new(Rank::R3, Suit::C),
        Self::new(Rank::R4, Suit::S),
        Self::new(Rank::R4, Suit::H),
        Self::new(Rank::R4, Suit::D),
        Self::new(Rank::R4, Suit::C),
        Self::new(Rank::R5, Suit::S),
        Self::new(Rank::R5, Suit::H),
        Self::new(Rank::R5, Suit::D),
        Self::new(Rank::R5, Suit::C),
        Self::new(Rank::R6, Suit::S),
        Self::new(Rank::R6, Suit::H),
        Self::new(Rank::R6, Suit::D),
        Self::new(Rank::R6, Suit::C),
        Self::new(Rank::R7, Suit::S),
        Self::new(Rank::R7, Suit::H),
        Self::new(Rank::R7, Suit::D),
        Self::new(Rank::R7, Suit::C),
        Self::new(Rank::R8, Suit::S),
        Self::new(Rank::R8, Suit::H),
        Self::new(Rank::R8, Suit::D),
        Self::new(Rank::R8, Suit::C),
        Self::new(Rank::R9, Suit::S),
        Self::new(Rank::R9, Suit::H),
        Self::new(Rank::R9, Suit::D),
        Self::new(Rank::R9, Suit::C),
        Self::new(Rank::RT, Suit::S),
        Self::new(Rank::RT, Suit::H),
        Self::new(Rank::RT, Suit::D),
        Self::new(Rank::RT, Suit::C),
        Self::new(Rank::RJ, Suit::S),
        Self::new(Rank::RJ, Suit::H),
        Self::new(Rank::RJ, Suit::D),
        Self::new(Rank::RJ, Suit::C),
        Self::new(Rank::RQ, Suit::S),
        Self::new(Rank::RQ, Suit::H),
        Self::new(Rank::RQ, Suit::D),
        Self::new(Rank::RQ, Suit::C),
        Self::new(Rank::RK, Suit::S),
        Self::new(Rank::RK, Suit::H),
        Self::new(Rank::RK, Suit::D),
        Self::new(Rank::RK, Suit::C),
        Self::new(Rank::RA, Suit::S),
        Self::new(Rank::RA, Suit::H),
        Self::new(Rank::RA, Suit::D),
        Self::new(Rank::RA, Suit::C),
    ];

    const ARR_ALL_SD: [Self; Self::N_CARDS_SD as usize] = [
        Self::new(Rank::R6, Suit::S),
        Self::new(Rank::R6, Suit::H),
        Self::new(Rank::R6, Suit::D),
        Self::new(Rank::R6, Suit::C),
        Self::new(Rank::R7, Suit::S),
        Self::new(Rank::R7, Suit::H),
        Self::new(Rank::R7, Suit::D),
        Self::new(Rank::R7, Suit::C),
        Self::new(Rank::R8, Suit::S),
        Self::new(Rank::R8, Suit::H),
        Self::new(Rank::R8, Suit::D),
        Self::new(Rank::R8, Suit::C),
        Self::new(Rank::R9, Suit::S),
        Self::new(Rank::R9, Suit::H),
        Self::new(Rank::R9, Suit::D),
        Self::new(Rank::R9, Suit::C),
        Self::new(Rank::RT, Suit::S),
        Self::new(Rank::RT, Suit::H),
        Self::new(Rank::RT, Suit::D),
        Self::new(Rank::RT, Suit::C),
        Self::new(Rank::RJ, Suit::S),
        Self::new(Rank::RJ, Suit::H),
        Self::new(Rank::RJ, Suit::D),
        Self::new(Rank::RJ, Suit::C),
        Self::new(Rank::RQ, Suit::S),
        Self::new(Rank::RQ, Suit::H),
        Self::new(Rank::RQ, Suit::D),
        Self::new(Rank::RQ, Suit::C),
        Self::new(Rank::RK, Suit::S),
        Self::new(Rank::RK, Suit::H),
        Self::new(Rank::RK, Suit::D),
        Self::new(Rank::RK, Suit::C),
        Self::new(Rank::RA, Suit::S),
        Self::new(Rank::RA, Suit::H),
        Self::new(Rank::RA, Suit::D),
        Self::new(Rank::RA, Suit::C),
    ];

    /// Creates a new card with the specified rank and suit.
    #[must_use]
    #[inline]
    pub const fn new(r: Rank, s: Suit) -> Self {
        Self { rank: r, suit: s }
    }

    /// Returns a slice of all cards
    #[inline]
    pub const fn all<const SD: bool>() -> &'static [Self] {
        const {
            if SD {
                &Self::ARR_ALL_SD
            } else {
                &Self::ARR_ALL
            }
        }
    }

    #[inline]
    pub(crate) const fn to_c64(self) -> Card64 {
        let mut res = Card64::EMPTY;
        res.set(self);

        res
    }

    #[inline]
    pub(crate) const fn eq(self, other: Self) -> bool {
        self.rank.eq(other.rank) && self.suit.eq(other.suit)
    }
}

impl Default for Card {
    fn default() -> Self {
        Self::new(Rank::R2, Suit::S)
    }
}

impl FromStr for Card {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cs = s.chars().filter(|c| !c.is_whitespace());

        if let Some(c) = cs.next()
            && let Ok(r) = Rank::try_from(c)
            && let Some(c) = cs.next()
            && let Ok(s) = Suit::try_from(c)
            && cs.next().is_none()
        {
            return Ok(Self::new(r, s));
        }

        Err(ParseError::InvalidCard(s.into()))
    }
}

#[cfg(feature = "serde")]
impl Serialize for Card {
    #[allow(clippy::cast_sign_loss)]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            serializer.serialize_str(&self.to_string())
        } else {
            use super::card_idx::CardIdx;

            serializer.serialize_u8(CardIdx::from(*self).0 as u8)
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Card {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use std::fmt;

        struct CardVisitor;

        impl Visitor<'_> for CardVisitor {
            type Value = Card;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("card index interger or card string")
            }

            fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                self.visit_u8(value.cast_unsigned())
            }

            fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                use super::card_idx::CardIdx;

                CardIdx(value.cast_signed()).to_card().map_or_else(
                    || Err(E::custom("invalid card")),
                    |card| Ok(card),
                )
            }

            fn visit_str<E>(self, text: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                text.parse::<Card>().map_err(E::custom)
            }
        }

        if deserializer.is_human_readable() {
            deserializer.deserialize_str(CardVisitor)
        } else {
            deserializer.deserialize_u8(CardVisitor)
        }
    }
}

#[cfg(any(test, feature = "quickcheck"))]
impl quickcheck::Arbitrary for Card {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        *g.choose(&Self::ARR_ALL).unwrap()
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_all(cards: CardN<3>) {
        for c in cards {
            if c.rank >= Rank::R6 {
                assert!(Card::all::<true>().contains(&c));
            }

            assert!(Card::all::<false>().contains(&c));
        }
    }

    #[test]
    fn test_default() {
        assert_eq!(card!("2s"), Card::default());
    }

    #[test]
    fn test_from_str() {
        let c = |s| Ok(cards!(s)[0]);

        assert_eq!(c("2s"), "2s".parse());
        assert_eq!(c("2s"), " 2 S ".parse());
        assert_eq!(
            Err(ParseError::InvalidCard("2s?".to_owned())),
            "2s?".parse::<Card>()
        );
        assert!("".parse::<Card>().is_err());
        assert!("?".parse::<Card>().is_err());
        assert!("2".parse::<Card>().is_err());
        assert!("2k".parse::<Card>().is_err());
    }

    #[quickcheck]
    fn test_to_string(c: Card) {
        assert_eq!(format!("{}{}", c.rank, c.suit), c.to_string());
    }

    #[quickcheck]
    fn test_macro(cs: CardN<3>) {
        assert_eq!(cs[0], card!(&cs[0].to_string()));
        let v = cs.as_slice().to_vec();
        let s: String = v.iter().map(ToString::to_string).collect();
        assert_eq!(v, cards!(&s));
    }

    #[test]
    fn test_ord() {
        let mut sorted = Card::ARR_ALL.to_vec();
        sorted.sort_unstable();

        assert_eq!(sorted, Card::ARR_ALL);
    }
}

#[cfg(all(test, feature = "serde"))]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests_serde {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_card_ser_de(card: Card) {
        assert_tokens(&card.compact(), &[Token::U8(to_i(card))]);
        assert_tokens(&card.readable(), &[Token::Str(to_s(card))]);
    }

    #[test]
    fn test_card_invalid() {
        assert_de_tokens_error::<Compact<Card>>(
            &[Token::I8(-1)],
            "invalid card",
        );
    }
}
