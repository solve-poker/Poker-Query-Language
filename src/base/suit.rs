use super::*;

/// Enum for Suits
#[derive(Copy, Clone, PartialEq, Eq, Debug, Ord, PartialOrd, Hash, Display)]
pub enum Suit {
    /// <span class="stab emoji">♠️</span> Spade
    #[display("s")]
    S = 0,
    /// <span class="stab emoji">♥️</span> Heart
    #[display("h")]
    H,
    /// <span class="stab emoji">♦️</span> Diamond
    #[display("d")]
    D,
    /// <span class="stab emoji">♣️</span> Club
    #[display("c")]
    C,
}

impl Suit {
    /// [ S, H, D, C ]
    pub const ARR_ALL: [Self; N_SUITS as usize] =
        [Self::S, Self::H, Self::D, Self::C];
}

impl TryFrom<char> for Suit {
    type Error = ParseError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'S' | 's' => Ok(Self::S),
            'H' | 'h' => Ok(Self::H),
            'D' | 'd' => Ok(Self::D),
            'C' | 'c' => Ok(Self::C),

            _ => Err(ParseError::InvalidSuit(c.into())),
        }
    }
}

impl FromStr for Suit {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cs = s.chars().filter(|c| !c.is_whitespace());

        if let Some(c) = cs.next() {
            if let Ok(s) = Self::try_from(c) {
                if cs.next().is_none() {
                    return Ok(s);
                }
            }
        }

        Err(ParseError::InvalidSuit(s.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Arbitrary for Suit {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            *g.choose(&Self::ARR_ALL).unwrap()
        }
    }

    #[test]
    fn test_consts() {
        assert_eq!(Suit::ARR_ALL, [Suit::S, Suit::H, Suit::D, Suit::C]);
    }

    #[test]
    fn test_as_int() {
        assert_eq!(Suit::S as i8, 0);
        assert_eq!(Suit::H as i8, 1);
        assert_eq!(Suit::D as i8, 2);
        assert_eq!(Suit::C as i8, 3);
    }

    #[test]
    fn test_from_char() {
        assert_eq!(Ok(Suit::S), 's'.try_into());
        assert_eq!(Ok(Suit::H), 'h'.try_into());
        assert_eq!(Ok(Suit::D), 'd'.try_into());
        assert_eq!(Ok(Suit::C), 'c'.try_into());

        assert_eq!(Ok(Suit::S), 'S'.try_into());
        assert_eq!(Ok(Suit::H), 'H'.try_into());
        assert_eq!(Ok(Suit::D), 'D'.try_into());
        assert_eq!(Ok(Suit::C), 'C'.try_into());

        assert_eq!(
            Err(ParseError::InvalidSuit("?".into())),
            Suit::try_from('?')
        );
    }

    #[test]
    fn test_from_str() {
        assert_eq!(Ok(Suit::S), " s ".parse());
        assert_eq!(
            Err(ParseError::InvalidSuit("sS".into())),
            "sS".parse::<Suit>()
        );
        assert!("".parse::<Suit>().is_err());
        assert!("?".parse::<Suit>().is_err());
    }

    #[test]
    fn test_to_string() {
        assert_eq!("s", &Suit::S.to_string());
        assert_eq!("h", &Suit::H.to_string());
        assert_eq!("d", &Suit::D.to_string());
        assert_eq!("c", &Suit::C.to_string());
    }
}
