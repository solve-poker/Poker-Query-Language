use std::{hash::Hash, str::FromStr};

use crate::{CardCount, ParseError, card::Idx};

/// Card rank from `R2` to `RA`.
#[cfg_attr(feature = "speedy", derive(speedy::Readable, speedy::Writable))] // LCOV_EXCL_LINE
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Copy, Clone, PartialEq, Eq, Debug, Ord, PartialOrd, Hash, derive_more::Display, Default,
)]
pub enum Rank {
    /// Two.
    #[default]
    #[cfg_attr(feature = "serde", serde(rename = "2"))]
    #[display("2")]
    R2 = 0,
    /// Three.
    #[cfg_attr(feature = "serde", serde(rename = "3"))]
    #[display("3")]
    R3,
    /// Four.
    #[cfg_attr(feature = "serde", serde(rename = "4"))]
    #[display("4")]
    R4,
    /// Five.
    #[cfg_attr(feature = "serde", serde(rename = "5"))]
    #[display("5")]
    R5,
    /// Six.
    #[cfg_attr(feature = "serde", serde(rename = "6"))]
    #[display("6")]
    R6,
    /// Seven.
    #[cfg_attr(feature = "serde", serde(rename = "7"))]
    #[display("7")]
    R7,
    /// Eight.
    #[cfg_attr(feature = "serde", serde(rename = "8"))]
    #[display("8")]
    R8,
    /// Nine.
    #[cfg_attr(feature = "serde", serde(rename = "9"))]
    #[display("9")]
    R9,
    /// Ten.
    #[cfg_attr(feature = "serde", serde(rename = "T"))]
    #[display("T")]
    RT,
    /// Jack.
    #[cfg_attr(feature = "serde", serde(rename = "J"))]
    #[display("J")]
    RJ,
    /// Queen.
    #[cfg_attr(feature = "serde", serde(rename = "Q"))]
    #[display("Q")]
    RQ,
    /// King.
    #[cfg_attr(feature = "serde", serde(rename = "K"))]
    #[display("K")]
    RK,
    /// Ace.
    #[cfg_attr(feature = "serde", serde(rename = "A"))]
    #[display("A")]
    RA,
}

impl Rank {
    /// Rank count in a standard deck.
    pub const N_RANKS: CardCount = 13;

    /// Rank count in a short deck.
    pub const N_RANKS_SD: CardCount = 9;

    /// Display character per rank in ascending order.
    pub const CHARS: [char; Self::N_RANKS as usize] = [
        '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
    ];

    const ARR_ALL: [Self; Self::N_RANKS as usize] = [
        Self::R2,
        Self::R3,
        Self::R4,
        Self::R5,
        Self::R6,
        Self::R7,
        Self::R8,
        Self::R9,
        Self::RT,
        Self::RJ,
        Self::RQ,
        Self::RK,
        Self::RA,
    ];

    const ARR_ALL_SD: [Self; Self::N_RANKS_SD as usize] = [
        Self::R6,
        Self::R7,
        Self::R8,
        Self::R9,
        Self::RT,
        Self::RJ,
        Self::RQ,
        Self::RK,
        Self::RA,
    ];

    /// Parses a rank from a character, returning `None` if invalid.
    #[inline]
    #[must_use]
    pub const fn from_char(c: char) -> Option<Self> {
        match c {
            '2' => Some(Self::R2),
            '3' => Some(Self::R3),
            '4' => Some(Self::R4),
            '5' => Some(Self::R5),
            '6' => Some(Self::R6),
            '7' => Some(Self::R7),
            '8' => Some(Self::R8),
            '9' => Some(Self::R9),
            't' | 'T' => Some(Self::RT),
            'j' | 'J' => Some(Self::RJ),
            'q' | 'Q' => Some(Self::RQ),
            'k' | 'K' => Some(Self::RK),
            'a' | 'A' => Some(Self::RA),
            _ => None,
        }
    }

    /// Returns the display character.
    #[inline]
    #[must_use]
    pub const fn to_char(self) -> char {
        Self::CHARS[self as usize]
    }

    /// Returns every rank, short-deck when `SD` is true.
    #[inline]
    #[must_use]
    pub const fn all<const SD: bool>() -> &'static [Self] {
        const {
            if SD {
                &Self::ARR_ALL_SD
            } else {
                &Self::ARR_ALL
            }
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

impl TryFrom<char> for Rank {
    type Error = ParseError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        Self::from_char(c).ok_or_else(|| ParseError::InvalidRank(c.into()))
    }
}

impl FromStr for Rank {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cs = s.chars().filter(|c| !c.is_whitespace());
        if let Some(c) = cs.next()
            && let Ok(r) = Self::try_from(c)
            && cs.next().is_none()
        {
            return Ok(r);
        }
        Err(ParseError::InvalidRank(s.into()))
    }
}

#[cfg(any(test, feature = "quickcheck"))]
impl quickcheck::Arbitrary for Rank {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        *g.choose(&Self::ARR_ALL).unwrap()
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;

    #[quickcheck]
    fn test_const_cmp(a: Rank, b: Rank) {
        assert_eq!(a < b, a.const_lt(b));
        assert_eq!(a == b, a.const_eq(b));
    }

    #[quickcheck]
    fn test_all(rank: Rank) {
        if rank >= Rank::R6 {
            assert!(Rank::all::<true>().contains(&rank));
        }

        assert!(Rank::all::<false>().contains(&rank));
    }

    #[test]
    fn test_as_int() {
        assert_eq!(Rank::R2 as Idx, 0);
        assert_eq!(Rank::R3 as Idx, 1);
        assert_eq!(Rank::R4 as Idx, 2);
        assert_eq!(Rank::R5 as Idx, 3);
        assert_eq!(Rank::R6 as Idx, 4);
        assert_eq!(Rank::R7 as Idx, 5);
        assert_eq!(Rank::R8 as Idx, 6);
        assert_eq!(Rank::R9 as Idx, 7);
        assert_eq!(Rank::RT as Idx, 8);
        assert_eq!(Rank::RJ as Idx, 9);
        assert_eq!(Rank::RQ as Idx, 10);
        assert_eq!(Rank::RK as Idx, 11);
        assert_eq!(Rank::RA as Idx, 12);
    }

    #[test]
    fn test_from_char() {
        assert_eq!('2'.try_into(), Ok(Rank::R2));
        assert_eq!('3'.try_into(), Ok(Rank::R3));
        assert_eq!('4'.try_into(), Ok(Rank::R4));
        assert_eq!('5'.try_into(), Ok(Rank::R5));
        assert_eq!('6'.try_into(), Ok(Rank::R6));
        assert_eq!('7'.try_into(), Ok(Rank::R7));
        assert_eq!('8'.try_into(), Ok(Rank::R8));
        assert_eq!('9'.try_into(), Ok(Rank::R9));

        assert_eq!('T'.try_into(), Ok(Rank::RT));
        assert_eq!('J'.try_into(), Ok(Rank::RJ));
        assert_eq!('Q'.try_into(), Ok(Rank::RQ));
        assert_eq!('K'.try_into(), Ok(Rank::RK));
        assert_eq!('A'.try_into(), Ok(Rank::RA));

        assert_eq!('t'.try_into(), Ok(Rank::RT));
        assert_eq!('j'.try_into(), Ok(Rank::RJ));
        assert_eq!('q'.try_into(), Ok(Rank::RQ));
        assert_eq!('k'.try_into(), Ok(Rank::RK));
        assert_eq!('a'.try_into(), Ok(Rank::RA));

        assert_eq!(
            Rank::try_from('?'),
            Err(ParseError::InvalidRank("?".into())),
        );
    }

    #[test]
    fn test_from_str() {
        assert_eq!(" 2 ".parse(), Ok(Rank::R2));
        assert_eq!(
            "23".parse::<Rank>(),
            Err(ParseError::InvalidRank("23".into())),
        );
        assert!("".parse::<Rank>().is_err());
        assert!("?".parse::<Rank>().is_err());
    }

    #[test]
    fn test_to_string() {
        assert_eq!(
            Rank::ARR_ALL
                .iter()
                .map(Rank::to_string)
                .collect::<String>(),
            "23456789TJQKA",
        );
    }

    #[test]
    fn test_to_char() {
        assert_eq!(
            Rank::ARR_ALL.map(Rank::to_char).iter().collect::<String>(),
            "23456789TJQKA",
        );
    }
}

#[cfg(all(test, feature = "serde"))]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests_serde {
    use super::*;
    use crate::*;

    fn assert_rank(rank: Rank, s: &'static str) {
        assert_tokens(
            &rank.compact(),
            &[Token::UnitVariant {
                name: "Rank",
                variant: s,
            }],
        );
    }

    #[quickcheck]
    fn test_rank_ser_de() {
        assert_rank(Rank::R2, "2");
        assert_rank(Rank::R3, "3");
        assert_rank(Rank::R4, "4");
        assert_rank(Rank::R5, "5");
        assert_rank(Rank::R6, "6");
        assert_rank(Rank::R7, "7");
        assert_rank(Rank::R8, "8");
        assert_rank(Rank::R9, "9");
        assert_rank(Rank::RT, "T");
        assert_rank(Rank::RJ, "J");
        assert_rank(Rank::RQ, "Q");
        assert_rank(Rank::RK, "K");
        assert_rank(Rank::RA, "A");
    }
}
