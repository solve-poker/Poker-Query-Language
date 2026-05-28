//! Cards relabeled to flush-relevant suits for isomorphic board forms.

use std::str::FromStr;

use crate::{FlushingSuit, ParseError, Rank};

/// Parses a single [`IsomorphicCard`] from a string, panicking on failure.
#[macro_export]
macro_rules! isocard {
    ($s:expr) => {
        $s.parse::<$crate::IsomorphicCard>().unwrap()
    };
}

/// Parses a sequence of [`IsomorphicCard`]s from a string, panicking on failure.
#[macro_export]
macro_rules! isocards {
    ($s:expr) => {{
        let s: &str = $s;
        let mut cards = Vec::new();
        let mut chars = s.chars().filter(|c| !c.is_whitespace());
        while let (Some(r), Some(s)) = (chars.next(), chars.next()) {
            cards.push($crate::isocard![format!("{r}{s}")]);
        }
        cards
    }};
}

#[cfg_attr(feature = "speedy", derive(speedy::Readable, speedy::Writable))] // LCOV_EXCL_LINE
#[derive(
    Clone,
    Copy,
    Default,
    derive_more::Display,
    derive_more::Debug,
    PartialOrd,
    Ord,
    PartialEq,
    Eq,
    Hash,
)]
#[display("{rank}{suit}")]
#[debug("{}", self)]
/// A card whose suit is relabeled to a flush-relevant [`FlushingSuit`].
pub struct IsomorphicCard {
    /// Card rank.
    pub rank: Rank,
    /// Flush-relevant suit label.
    pub suit: FlushingSuit,
}

impl IsomorphicCard {
    /// Creates an `IsomorphicCard` from a rank and flush-relevant suit.
    pub const fn new(rank: Rank, suit: FlushingSuit) -> Self {
        Self { rank, suit }
    }

    /// Orders cards by rank, breaking ties by suit label.
    ///
    /// Const-context less-than, equivalent to [`PartialOrd::lt`].
    #[inline]
    #[must_use]
    pub const fn const_lt(self, other: Self) -> bool {
        if self.rank.const_eq(other.rank) {
            self.suit.const_lt(other.suit)
        } else {
            self.rank.const_lt(other.rank)
        }
    }
}

impl FromStr for IsomorphicCard {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cs = s.chars().filter(|c| !c.is_whitespace());

        if let Some(c) = cs.next()
            && let Ok(r) = Rank::try_from(c)
            && let Some(c) = cs.next()
            && let Ok(s) = FlushingSuit::try_from(c)
            && cs.next().is_none()
        {
            return Ok(Self::new(r, s));
        }

        Err(ParseError::InvalidCard(s.into()))
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for IsomorphicCard {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for IsomorphicCard {
    fn deserialize<D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        use std::fmt;

        use serde::de;

        struct V;
        impl de::Visitor<'_> for V {
            type Value = IsomorphicCard;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
                f.write_str("an isomorphic card string like \"Ah\"")
            }
            fn visit_str<E: de::Error>(
                self,
                s: &str,
            ) -> Result<Self::Value, E> {
                s.parse().map_err(E::custom)
            }
        }
        deserializer.deserialize_str(V)
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_from_str() {
        assert_eq!(
            isocard!("Ax"),
            IsomorphicCard::new(Rank::RA, FlushingSuit::X)
        );
        assert!("An".parse::<IsomorphicCard>().is_ok());
        assert!("A".parse::<IsomorphicCard>().is_err());
        assert!("?x".parse::<IsomorphicCard>().is_err());
        assert!("A?".parse::<IsomorphicCard>().is_err());
        assert!("Axx".parse::<IsomorphicCard>().is_err());
    }

    #[test]
    fn test_display() {
        assert_eq!(isocard!("Ty").to_string(), "Ty");
    }

    #[test]
    fn test_lt() {
        assert!(isocard!("Kx").const_lt(isocard!("Ax")));
        assert!(isocard!("Ax").const_lt(isocard!("Ay")));
        assert!(!isocard!("Ay").const_lt(isocard!("Ax")));
    }
}

#[cfg(all(test, feature = "serde"))]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests_serde {
    use super::*;
    use crate::*;

    #[test]
    fn test_iso_card_ser_de() {
        let card = isocard!("Ax");
        assert_tokens(&card, &[Token::Str("Ax")]);
    }

    #[test]
    fn test_iso_card_invalid() {
        assert_de_tokens_error::<IsomorphicCard>(&[Token::Str("A?")], "A?");
    }

    #[test]
    fn test_iso_card_unexpected_type() {
        assert_de_tokens_error::<IsomorphicCard>(
            &[Token::Bool(true)],
            "invalid type: boolean `true`, expected an isomorphic card string like \"Ah\"",
        );
    }
}
