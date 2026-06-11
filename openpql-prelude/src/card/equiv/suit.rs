//! Suit labels classified by flush relevance.

use std::str::FromStr;

use crate::{Idx, ParseError};

#[cfg_attr(feature = "speedy", derive(speedy::Readable, speedy::Writable))] // LCOV_EXCL_LINE
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[derive(
    Clone, Copy, Default, Debug, derive_more::Display, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
/// A suit label relabeled by flush relevance: `X`/`Y`/`Z` for candidates, `N` for irrelevant.
pub enum FlushingSuit {
    /// flush candidate 1
    #[display("x")]
    X = 0,
    /// flush candidate 2
    #[display("y")]
    Y,
    /// flush candidate 3
    #[display("z")]
    Z,
    /// irrelevant
    #[default]
    #[display("n")]
    N,
}

impl FlushingSuit {
    /// Parses a flush-relevant suit label case-insensitively, or `None` if invalid.
    #[inline]
    #[must_use]
    pub(crate) const fn from_char(suit: char) -> Option<Self> {
        match suit.to_ascii_lowercase() {
            'x' => Some(Self::X),
            'y' => Some(Self::Y),
            'z' => Some(Self::Z),
            'n' => Some(Self::N),
            _ => None,
        }
    }

    /// Const-context equality, equivalent to [`PartialEq::eq`].
    #[inline]
    #[must_use]
    pub const fn const_eq(self, other: Self) -> bool {
        self as Idx == other as Idx
    }

    /// Const-context less-than, equivalent to [`PartialOrd::lt`].
    #[inline]
    #[must_use]
    pub const fn const_lt(self, other: Self) -> bool {
        (self as Idx) < other as Idx
    }
}

impl TryFrom<char> for FlushingSuit {
    type Error = ParseError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        Self::from_char(c).ok_or_else(|| ParseError::InvalidSuit(c.into()))
    }
}

impl FromStr for FlushingSuit {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cs = s.chars().filter(|c| !c.is_whitespace());
        if let Some(c) = cs.next()
            && let Ok(s) = Self::try_from(c)
            && cs.next().is_none()
        {
            return Ok(s);
        }
        Err(ParseError::InvalidSuit(s.into()))
    }
}

#[cfg(any(test, feature = "quickcheck"))]
impl quickcheck::Arbitrary for FlushingSuit {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        *g.choose(&[Self::X, Self::Y, Self::Z, Self::N]).unwrap()
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;

    #[quickcheck]
    fn test_const_cmp(a: FlushingSuit, b: FlushingSuit) {
        assert_eq!(a < b, a.const_lt(b));
        assert_eq!(a == b, a.const_eq(b));
    }

    #[test]
    fn test_from_char() {
        assert_eq!(FlushingSuit::from_char('X'), Some(FlushingSuit::X));
        assert_eq!(FlushingSuit::from_char('y'), Some(FlushingSuit::Y));
        assert_eq!(FlushingSuit::from_char('Z'), Some(FlushingSuit::Z));
        assert_eq!(FlushingSuit::from_char('n'), Some(FlushingSuit::N));
        assert_eq!(FlushingSuit::from_char('?'), None);
    }

    #[test]
    fn test_try_from() {
        assert_eq!(FlushingSuit::try_from('x'), Ok(FlushingSuit::X));
        assert_eq!(
            FlushingSuit::try_from('?'),
            Err(ParseError::InvalidSuit("?".into())),
        );
    }

    #[test]
    fn test_from_str() {
        assert_eq!(" y ".parse(), Ok(FlushingSuit::Y));
        assert!("xy".parse::<FlushingSuit>().is_err());
        assert!("".parse::<FlushingSuit>().is_err());
        assert!("?".parse::<FlushingSuit>().is_err());
    }

    #[test]
    fn test_display() {
        assert_eq!(FlushingSuit::X.to_string(), "x");
        assert_eq!(FlushingSuit::Y.to_string(), "y");
        assert_eq!(FlushingSuit::Z.to_string(), "z");
        assert_eq!(FlushingSuit::N.to_string(), "n");
    }
}

#[cfg(all(test, feature = "serde"))]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests_serde {
    use super::*;
    use crate::*;

    fn assert_flushing_suit(suit: FlushingSuit, s: &'static str) {
        assert_tokens(
            &suit.compact(),
            &[Token::UnitVariant {
                name: "FlushingSuit",
                variant: s,
            }],
        );
    }

    #[quickcheck]
    fn test_flushing_suit_ser_de() {
        assert_flushing_suit(FlushingSuit::X, "x");
        assert_flushing_suit(FlushingSuit::Y, "y");
        assert_flushing_suit(FlushingSuit::Z, "z");
        assert_flushing_suit(FlushingSuit::N, "n");
    }
}
