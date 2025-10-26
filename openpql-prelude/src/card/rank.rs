use super::{CardCount, Display, FromStr, Hash, Idx, ParseError};
#[cfg(feature = "python")]
use crate::python::*;

/// Card rank representation.
///
/// Represents card ranks from 2 to Ace, with parsing support and conversion utilities.
#[cfg_attr(feature = "python", pyclass(eq, ord, str, frozen, hash))]
#[derive(
    Copy, Clone, PartialEq, Eq, Debug, Ord, PartialOrd, Hash, Display, Default,
)]
pub enum Rank {
    #[default]
    #[display("2")]
    R2 = 0,
    #[display("3")]
    R3,
    #[display("4")]
    R4,
    #[display("5")]
    R5,
    #[display("6")]
    R6,
    #[display("7")]
    R7,
    #[display("8")]
    R8,
    #[display("9")]
    R9,
    #[display("T")]
    RT,
    #[display("J")]
    RJ,
    #[display("Q")]
    RQ,
    #[display("K")]
    RK,
    #[display("A")]
    RA,
}

impl Rank {
    /// Number of ranks in a standard deck
    pub const N_RANKS: CardCount = 13;

    /// Number of ranks in a short deck
    pub const N_RANKS_SD: CardCount = 9;

    /// Character representations for ranks
    pub const CHARS: [char; Self::N_RANKS as usize] = [
        '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
    ];

    /// Array of all 13 ranks.
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

    /// Array of all 9 ranks in a short deck (6+).
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

    /// Converts a character to a rank, returning `None` if invalid.
    #[inline]
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

    #[inline]
    pub const fn to_char(self) -> char {
        Self::CHARS[self as usize]
    }

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
    pub(crate) const fn eq(self, other: Self) -> bool {
        self as Idx == other as Idx
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

#[cfg(feature = "python")]
#[pymethods]
impl Rank {
    #[staticmethod]
    fn from_str(s: &str) -> PyResult<Self> {
        Ok(s.parse::<Self>()?)
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
