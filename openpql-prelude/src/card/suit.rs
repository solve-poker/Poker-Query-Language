use super::{CardCount, Display, FromStr, Hash, Idx, ParseError};

/// Card suit representation.
///
/// Represents the four card suits (spades, hearts, diamonds, clubs),
/// with parsing support and conversion utilities.
#[derive(
    Copy, Clone, PartialEq, Eq, Debug, Ord, PartialOrd, Hash, Display, Default,
)]
pub enum Suit {
    #[display("s")]
    #[default]
    S = 0,
    #[display("h")]
    H,
    #[display("d")]
    D,
    #[display("c")]
    C,
}

impl Suit {
    /// Number of suits in a deck
    pub const N_SUITS: CardCount = 4;

    /// Array of all 4 suits.
    pub const ARR_ALL: [Self; Self::N_SUITS as usize] =
        [Self::S, Self::H, Self::D, Self::C];

    /// Character representations for suits
    pub const CHARS: [char; Self::N_SUITS as usize] = ['s', 'h', 'd', 'c'];
    /// Converts a character to a suit, returning `None` if invalid.
    #[inline]
    pub const fn from_char(c: char) -> Option<Self> {
        match c {
            'S' | 's' => Some(Self::S),
            'H' | 'h' => Some(Self::H),
            'D' | 'd' => Some(Self::D),
            'C' | 'c' => Some(Self::C),
            _ => None,
        }
    }

    #[inline]
    pub const fn to_char(self) -> char {
        Self::CHARS[self as usize]
    }

    #[inline]
    pub(crate) const fn eq(self, other: Self) -> bool {
        self as Idx == other as Idx
    }
}

impl TryFrom<char> for Suit {
    type Error = ParseError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        Self::from_char(c).ok_or_else(|| ParseError::InvalidSuit(c.into()))
    }
}

impl FromStr for Suit {
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
impl quickcheck::Arbitrary for Suit {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        *g.choose(&Self::ARR_ALL).unwrap()
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;

    #[test]
    fn test_all() {
        assert_eq!(Suit::ARR_ALL, [Suit::S, Suit::H, Suit::D, Suit::C]);
    }

    #[test]
    fn test_as_int() {
        assert_eq!(Suit::S as Idx, 0);
        assert_eq!(Suit::H as Idx, 1);
        assert_eq!(Suit::D as Idx, 2);
        assert_eq!(Suit::C as Idx, 3);
    }

    #[test]
    fn test_from_char() {
        assert_eq!('s'.try_into(), Ok(Suit::S));
        assert_eq!('h'.try_into(), Ok(Suit::H));
        assert_eq!('d'.try_into(), Ok(Suit::D));
        assert_eq!('c'.try_into(), Ok(Suit::C));

        assert_eq!('S'.try_into(), Ok(Suit::S));
        assert_eq!('H'.try_into(), Ok(Suit::H));
        assert_eq!('D'.try_into(), Ok(Suit::D));
        assert_eq!('C'.try_into(), Ok(Suit::C));

        assert_eq!(
            Suit::try_from('?'),
            Err(ParseError::InvalidSuit("?".into())),
        );
    }

    #[test]
    fn test_from_str() {
        assert_eq!(" s ".parse(), Ok(Suit::S));
        assert_eq!(
            "sS".parse::<Suit>(),
            Err(ParseError::InvalidSuit("sS".into())),
        );
        assert!("".parse::<Suit>().is_err());
        assert!("?".parse::<Suit>().is_err());
    }

    #[test]
    fn test_to_string() {
        assert_eq!(&Suit::S.to_string(), "s");
        assert_eq!(&Suit::H.to_string(), "h");
        assert_eq!(&Suit::D.to_string(), "d");
        assert_eq!(&Suit::C.to_string(), "c");
    }

    #[test]
    fn test_to_char() {
        assert_eq!(Suit::S.to_char(), 's');
        assert_eq!(Suit::H.to_char(), 'h');
        assert_eq!(Suit::D.to_char(), 'd');
        assert_eq!(Suit::C.to_char(), 'c');
    }
}
