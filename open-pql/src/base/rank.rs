use super::{Display, FromStr, Hash, N_RANKS, ParseError, mem};

/// Enum for Ranks
///
/// Represents the rank (value) of a playing card from 2 to Ace.
/// Ranks are ordered from lowest (2) to highest (Ace) for poker hand evaluation.
/// Each rank has a unique numeric value (0-12) and character representation.
///
/// # Examples
///
/// ```
/// use open_pql::{Rank, Rank::*};
///
/// let rank = RA; // Ace
/// assert_eq!(rank.to_string(), "A");
/// assert_eq!(rank as u8, 12);
///
/// let parsed: Rank = "K".parse().unwrap();
/// assert_eq!(parsed, RK);
/// ```
#[derive(
    Copy, Clone, PartialEq, Eq, Debug, Ord, PartialOrd, Hash, Display, Default,
)]
pub enum Rank {
    /// Deuce
    #[default]
    #[display("2")]
    R2 = 0,
    /// Three
    #[display("3")]
    R3,
    /// Four
    #[display("4")]
    R4,
    /// Five
    #[display("5")]
    R5,
    /// Six
    #[display("6")]
    R6,
    /// Seven
    #[display("7")]
    R7,
    /// Eight
    #[display("8")]
    R8,
    /// Nine
    #[display("9")]
    R9,
    /// Ten
    #[display("T")]
    RT,
    /// Jack
    #[display("J")]
    RJ,
    /// Queen
    #[display("Q")]
    RQ,
    /// King
    #[display("K")]
    RK,
    /// Ace
    #[display("A")]
    RA,
}

impl Rank {
    /// All possible ranks
    pub const ARR_ALL: [Self; N_RANKS as usize] = [
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

    /// All ranks used in short deck poker
    pub const ARR_ALL_SHORT: [Self; 9] = [
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

    /// Creates a rank from a u8 value (0-12)
    pub(crate) fn from_u8(v: u8) -> Self {
        debug_assert!(v < N_RANKS, "invalid rank: {v}");
        unsafe { mem::transmute(v) }
    }

    /// Creates a rank from a character, returning None for invalid characters
    ///
    /// # Examples
    ///
    /// ```
    /// use open_pql::{Rank, Rank::*};
    ///
    /// assert_eq!(Rank::from_char('A'), Some(RA));
    /// assert_eq!(Rank::from_char('K'), Some(RK));
    /// assert_eq!(Rank::from_char('2'), Some(R2));
    /// assert_eq!(Rank::from_char('X'), None);
    /// ```
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
}

impl From<Rank> for char {
    fn from(value: Rank) -> Self {
        match value {
            Rank::R2 => '2',
            Rank::R3 => '3',
            Rank::R4 => '4',
            Rank::R5 => '5',
            Rank::R6 => '6',
            Rank::R7 => '7',
            Rank::R8 => '8',
            Rank::R9 => '9',
            Rank::RT => 'T',
            Rank::RJ => 'J',
            Rank::RQ => 'Q',
            Rank::RK => 'K',
            Rank::RA => 'A',
        }
    }
}

impl TryFrom<char> for Rank {
    type Error = ParseError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '2' => Ok(Self::R2),
            '3' => Ok(Self::R3),
            '4' => Ok(Self::R4),
            '5' => Ok(Self::R5),
            '6' => Ok(Self::R6),
            '7' => Ok(Self::R7),
            '8' => Ok(Self::R8),
            '9' => Ok(Self::R9),
            'T' | 't' => Ok(Self::RT),
            'J' | 'j' => Ok(Self::RJ),
            'Q' | 'q' => Ok(Self::RQ),
            'K' | 'k' => Ok(Self::RK),
            'A' | 'a' => Ok(Self::RA),
            _ => Err(ParseError::InvalidRank(c.into())),
        }
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

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::*;

    impl Arbitrary for Rank {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            *g.choose(&Self::ARR_ALL).unwrap()
        }
    }

    #[test]
    fn test_consts() {
        assert_eq!(
            Rank::ARR_ALL,
            [
                Rank::R2,
                Rank::R3,
                Rank::R4,
                Rank::R5,
                Rank::R6,
                Rank::R7,
                Rank::R8,
                Rank::R9,
                Rank::RT,
                Rank::RJ,
                Rank::RQ,
                Rank::RK,
                Rank::RA,
            ]
        );
    }

    #[test]
    fn test_as_int() {
        assert_eq!(Rank::R2 as i8, 0);
        assert_eq!(Rank::R3 as i8, 1);
        assert_eq!(Rank::R4 as i8, 2);
        assert_eq!(Rank::R5 as i8, 3);
        assert_eq!(Rank::R6 as i8, 4);
        assert_eq!(Rank::R7 as i8, 5);
        assert_eq!(Rank::R8 as i8, 6);
        assert_eq!(Rank::R9 as i8, 7);
        assert_eq!(Rank::RT as i8, 8);
        assert_eq!(Rank::RJ as i8, 9);
        assert_eq!(Rank::RQ as i8, 10);
        assert_eq!(Rank::RK as i8, 11);
        assert_eq!(Rank::RA as i8, 12);
    }

    #[test]
    fn test_from_char() {
        assert_eq!(Ok(Rank::R2), '2'.try_into());
        assert_eq!(Ok(Rank::R3), '3'.try_into());
        assert_eq!(Ok(Rank::R4), '4'.try_into());
        assert_eq!(Ok(Rank::R5), '5'.try_into());
        assert_eq!(Ok(Rank::R6), '6'.try_into());
        assert_eq!(Ok(Rank::R7), '7'.try_into());
        assert_eq!(Ok(Rank::R8), '8'.try_into());
        assert_eq!(Ok(Rank::R9), '9'.try_into());

        assert_eq!(Ok(Rank::RT), 'T'.try_into());
        assert_eq!(Ok(Rank::RJ), 'J'.try_into());
        assert_eq!(Ok(Rank::RQ), 'Q'.try_into());
        assert_eq!(Ok(Rank::RK), 'K'.try_into());
        assert_eq!(Ok(Rank::RA), 'A'.try_into());

        assert_eq!(Ok(Rank::RT), 't'.try_into());
        assert_eq!(Ok(Rank::RJ), 'j'.try_into());
        assert_eq!(Ok(Rank::RQ), 'q'.try_into());
        assert_eq!(Ok(Rank::RK), 'k'.try_into());
        assert_eq!(Ok(Rank::RA), 'a'.try_into());

        assert_eq!(
            Err(ParseError::InvalidRank("?".into())),
            Rank::try_from('?')
        );
    }

    #[test]
    fn test_from_char_option() {
        assert_eq!(Some(Rank::R2), Rank::from_char('2'));
        assert_eq!(Some(Rank::R3), Rank::from_char('3'));
        assert_eq!(Some(Rank::R4), Rank::from_char('4'));
        assert_eq!(Some(Rank::R5), Rank::from_char('5'));
        assert_eq!(Some(Rank::R6), Rank::from_char('6'));
        assert_eq!(Some(Rank::R7), Rank::from_char('7'));
        assert_eq!(Some(Rank::R8), Rank::from_char('8'));
        assert_eq!(Some(Rank::R9), Rank::from_char('9'));

        assert_eq!(Some(Rank::RT), Rank::from_char('T'));
        assert_eq!(Some(Rank::RJ), Rank::from_char('J'));
        assert_eq!(Some(Rank::RQ), Rank::from_char('Q'));
        assert_eq!(Some(Rank::RK), Rank::from_char('K'));
        assert_eq!(Some(Rank::RA), Rank::from_char('A'));

        assert_eq!(Some(Rank::RT), Rank::from_char('t'));
        assert_eq!(Some(Rank::RJ), Rank::from_char('j'));
        assert_eq!(Some(Rank::RQ), Rank::from_char('q'));
        assert_eq!(Some(Rank::RK), Rank::from_char('k'));
        assert_eq!(Some(Rank::RA), Rank::from_char('a'));

        assert_eq!(None, Rank::from_char('?'));
        assert_eq!(None, Rank::from_char('1'));
        assert_eq!(None, Rank::from_char('X'));
    }

    #[test]
    fn test_from_str() {
        assert_eq!(Ok(Rank::R2), " 2 ".parse());
        assert_eq!(
            Err(ParseError::InvalidRank("23".into())),
            "23".parse::<Rank>()
        );
        assert!("".parse::<Rank>().is_err());
        assert!("?".parse::<Rank>().is_err());
    }

    #[test]
    fn test_to_string() {
        assert_eq!("2", &Rank::R2.to_string());
        assert_eq!("3", &Rank::R3.to_string());
        assert_eq!("4", &Rank::R4.to_string());
        assert_eq!("5", &Rank::R5.to_string());
        assert_eq!("6", &Rank::R6.to_string());
        assert_eq!("7", &Rank::R7.to_string());
        assert_eq!("8", &Rank::R8.to_string());
        assert_eq!("9", &Rank::R9.to_string());
        assert_eq!("T", &Rank::RT.to_string());
        assert_eq!("J", &Rank::RJ.to_string());
        assert_eq!("Q", &Rank::RQ.to_string());
        assert_eq!("K", &Rank::RK.to_string());
        assert_eq!("A", &Rank::RA.to_string());
    }

    #[test]
    fn test_to_char() {
        let cs = "23456789TJQKA";
        for (i, &r) in Rank::ARR_ALL.iter().enumerate() {
            assert_eq!(cs.chars().nth(i).unwrap(), char::from(r));
        }
    }

    #[should_panic(expected = "invalid rank")]
    #[test]
    fn test_from_u8() {
        Rank::from_u8(14);
    }
}
