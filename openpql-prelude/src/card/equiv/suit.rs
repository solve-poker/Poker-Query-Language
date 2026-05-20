//! Suit labels classified by flush relevance.

use std::str::FromStr;

use crate::ParseError;

#[derive(
    Clone,
    Copy,
    Default,
    Debug,
    derive_more::Display,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
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
    /// irrelevent
    #[default]
    #[display("n")]
    N,
}

impl FlushingSuit {
    /// Parses a flush-relevant suit label, case-insensitively. Returns `None`
    /// for any character other than `x`, `y`, `z`, or `n`.
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
