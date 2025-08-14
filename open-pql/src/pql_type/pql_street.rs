// TODO: remove later
#![allow(clippy::fallible_impl_from)]

use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Copy, PartialOrd, Ord, Default)]
pub enum PQLStreet {
    #[default]
    Flop = 1,
    Turn,
    River,
}

impl FromStr for PQLStreet {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().trim() {
            "flop" => Ok(Self::Flop),
            "turn" => Ok(Self::Turn),
            "river" => Ok(Self::River),

            _ => Err(ParseError::InvalidStreet(s.into())),
        }
    }
}

impl From<(Board, PQLStreet)> for Card64 {
    fn from((board, street): (Board, PQLStreet)) -> Self {
        let mut c64 = Self::from(board.flop.unwrap().as_slice());

        if street >= PQLStreet::Turn {
            c64.set(board.turn.unwrap());
        }

        if street >= PQLStreet::River {
            c64.set(board.river.unwrap());
        }

        c64
    }
}

impl TryFrom<&str> for PQLStreet {
    type Error = ParseError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    impl Arbitrary for PQLStreet {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            #[allow(unused)]
            const fn completeness_check(e: PQLStreet) {
                match e {
                    PQLStreet::Flop | PQLStreet::Turn | PQLStreet::River => (),
                }
            }
            *g.choose(&[Self::Flop, Self::Turn, Self::River]).unwrap()
        }
    }

    #[test]
    fn test_from_str() {
        assert_eq!(Ok(PQLStreet::Flop), "Flop".parse());
        assert_eq!(Ok(PQLStreet::Turn), "tUrn".parse());
        assert_eq!(Ok(PQLStreet::River), "riVer".parse());

        assert_eq!(Ok(PQLStreet::Flop), " flop ".parse(), "should trim");

        assert!("invalid".parse::<PQLStreet>().is_err());
    }
}
