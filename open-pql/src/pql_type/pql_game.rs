use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PQLGame {
    #[default]
    Holdem,
    Omaha,
    ShortDeck,
}

impl PQLGame {
    pub const fn n_cards(&self) -> PQLCardCount {
        match self {
            Self::Holdem | Self::ShortDeck => 2,
            Self::Omaha => 4,
        }
    }
}

impl FromStr for PQLGame {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().trim() {
            "holdem" => Ok(Self::Holdem),
            "omaha" => Ok(Self::Omaha),
            "shortdeck" => Ok(Self::ShortDeck),
            _ => Err(ParseError::InvalidGame(s.into())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Arbitrary for PQLGame {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            #[allow(unused)]
            const fn completeness_check(e: PQLGame) {
                match e {
                    PQLGame::Holdem | PQLGame::Omaha | PQLGame::ShortDeck => (),
                }
            }

            *g.choose(&[Self::Holdem, Self::Omaha, Self::ShortDeck])
                .unwrap()
        }
    }

    #[test]
    fn test_n_cards() {
        assert_eq!(2, PQLGame::Holdem.n_cards());
        assert_eq!(4, PQLGame::Omaha.n_cards());
        assert_eq!(2, PQLGame::ShortDeck.n_cards());
    }

    #[test]
    fn test_from_str() {
        assert_eq!(Ok(PQLGame::Holdem), " HoldEM ".parse());

        assert_eq!(Ok(PQLGame::Omaha), "omaha".parse());
        assert_eq!(Ok(PQLGame::ShortDeck), "shortdeck".parse());

        assert_eq!(
            Err(ParseError::InvalidGame("unknown".into())),
            "unknown".parse::<PQLGame>()
        );
    }
}
