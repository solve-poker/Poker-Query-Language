//! Cards relabeled to flush-relevant suits for isomorphic board forms.

use std::str::FromStr;

use crate::{FlushingSuit, Idx, ParseError, Rank};

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

#[derive(Clone, Copy, Debug, derive_more::Display, PartialEq, Eq, Hash)]
#[display("{rank}{suit}")]
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
    #[inline]
    pub(crate) const fn lt(self, other: Self) -> bool {
        let rank_l = self.rank as Idx;
        let rank_r = other.rank as Idx;
        if rank_l == rank_r {
            (self.suit as Idx) < (other.suit as Idx)
        } else {
            rank_l < rank_r
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
