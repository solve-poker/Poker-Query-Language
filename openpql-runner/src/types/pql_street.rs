use super::*;

#[derive(
    Debug, Clone, PartialEq, Eq, Copy, PartialOrd, Ord, Default, Display,
)]
pub enum PQLStreet {
    #[default]
    Flop = 1,
    Turn,
    River,
}

impl PQLStreet {
    pub fn board_card_count(self) -> PQLCardCount {
        prelude::Street::board_card_count(self.into())
    }
}

impl From<PQLStreet> for prelude::Street {
    fn from(v: PQLStreet) -> Self {
        match v {
            PQLStreet::Flop => Self::Flop,
            PQLStreet::Turn => Self::Turn,
            PQLStreet::River => Self::River,
        }
    }
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

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
pub mod tests {
    use super::*;
    use crate::*;

    impl quickcheck::Arbitrary for PQLStreet {
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
