use super::*;

/// Enum for Ranks
#[derive(
    Copy, Clone, PartialEq, Eq, Debug, Ord, PartialOrd, Hash, Display, Default,
)]
pub enum Rank {
    /// Duece
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
    /// [ R2, R3, R4, R5, R6, R7, R8, R9, RT, RJ, RQ, RK, RA ]
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

        if let Some(c) = cs.next() {
            if let Ok(r) = Self::try_from(c) {
                if cs.next().is_none() {
                    return Ok(r);
                }
            }
        }

        Err(ParseError::InvalidRank(s.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
