use super::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, Display)]
#[display("{category}")]
pub struct PQLFlopHandCategory {
    pub category: FlopHandCategory,
    pub ord: HandTypeOrd,
}

impl PQLFlopHandCategory {
    pub fn min(g: PQLGame) -> Self {
        Self {
            category: FlopHandCategory::MIN,
            ord: g.into(),
        }
    }

    pub fn max(g: PQLGame) -> Self {
        Self {
            category: FlopHandCategory::MAX,
            ord: g.into(),
        }
    }
}

impl PartialOrd for PQLFlopHandCategory {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.ord == other.ord {
            self.category
                .to_u8(self.ord)
                .partial_cmp(&other.category.to_u8(self.ord))
        } else {
            None
        }
    }
}

impl From<(FlopHandCategory, PQLGame)> for PQLFlopHandCategory {
    fn from(t: (FlopHandCategory, PQLGame)) -> Self {
        Self {
            category: t.0,
            ord: t.1.into(),
        }
    }
}

impl TryFrom<(&str, PQLGame)> for PQLFlopHandCategory {
    type Error = ParseError;

    fn try_from((s, g): (&str, PQLGame)) -> Result<Self, Self::Error> {
        Ok((s.parse()?, g).into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Display)]
pub enum FlopHandCategory {
    #[default]
    #[display("FLOPNOTHING")]
    Nothing,
    #[display("FLOPUNDERPAIR")]
    UnderPair,
    #[display("FLOPTHIRDPAIR")]
    ThirdPair,
    #[display("FLOPPOCKET23")]
    Pocket23,
    #[display("FLOPSECONDPAIR")]
    SecondPair,
    #[display("FLOPPOCKET12")]
    Pocket12,
    #[display("FLOPTOPPAIR")]
    TopPair,
    #[display("FLOPOVERPAIR")]
    Overpair,
    #[display("FLOPBOTTOMTWO")]
    BottomTwo,
    #[display("FLOPTOPANDBOTTOM")]
    TopAndBottom,
    #[display("FLOPTOPTWO")]
    TopTwo,
    #[display("FLOPTRIPS")]
    Trips,
    #[display("FLOPSET")]
    Set,
    #[display("FLOPSTRAIGHT")]
    Straight,
    #[display("FLOPFLUSH")]
    Flush,
    #[display("FLOPFULLHOUSE")]
    FullHouse,
    #[display("FLOPQUADS")]
    Quads,
    #[display("FLOPSTRAIGHTFLUSH")]
    StraightFlush,
}

impl FlopHandCategory {
    pub const MAX: Self = Self::StraightFlush;
    pub const MIN: Self = Self::Nothing;

    pub const ARR_ALL: [Self; 18] = [
        Self::Nothing,
        Self::UnderPair,
        Self::ThirdPair,
        Self::Pocket23,
        Self::SecondPair,
        Self::Pocket12,
        Self::TopPair,
        Self::Overpair,
        Self::BottomTwo,
        Self::TopAndBottom,
        Self::TopTwo,
        Self::Trips,
        Self::Set,
        Self::Straight,
        Self::Flush,
        Self::FullHouse,
        Self::Quads,
        Self::StraightFlush,
    ];

    const fn to_u8(self, ord: HandTypeOrd) -> u8 {
        match self {
            Self::Nothing => 0,
            Self::UnderPair => 1,
            Self::ThirdPair => 2,
            Self::Pocket23 => 3,
            Self::SecondPair => 4,
            Self::Pocket12 => 5,
            Self::TopPair => 6,
            Self::Overpair => 7,
            Self::BottomTwo => 8,
            Self::TopAndBottom => 9,
            Self::TopTwo => 10,
            Self::Trips => 11,
            Self::Set => 12,
            Self::Straight => 13,
            Self::Flush => match ord {
                HandTypeOrd::Standard => 14,
                HandTypeOrd::Shortdeck => 15,
            },
            Self::FullHouse => match ord {
                HandTypeOrd::Standard => 15,
                HandTypeOrd::Shortdeck => 14,
            },
            Self::Quads => 16,
            Self::StraightFlush => 17,
        }
    }
}

impl FromStr for FlopHandCategory {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().trim() {
            "flopnothing" => Ok(Self::Nothing),
            "flopunderpair" => Ok(Self::UnderPair),
            "flopthirdpair" => Ok(Self::ThirdPair),
            "floppocket23" => Ok(Self::Pocket23),
            "flopsecondpair" => Ok(Self::SecondPair),
            "floppocket12" => Ok(Self::Pocket12),
            "floptoppair" => Ok(Self::TopPair),
            "flopoverpair" => Ok(Self::Overpair),
            "flopbottomtwo" => Ok(Self::BottomTwo),
            "floptopandbottom" => Ok(Self::TopAndBottom),
            "floptoptwo" => Ok(Self::TopTwo),
            "floptrips" => Ok(Self::Trips),
            "flopset" => Ok(Self::Set),
            "flopstraight" => Ok(Self::Straight),
            "flopflush" => Ok(Self::Flush),
            "flopfullhouse" => Ok(Self::FullHouse),
            "flopquads" => Ok(Self::Quads),
            "flopstraightflush" => Ok(Self::StraightFlush),
            _ => Err(ParseError::InvalidFlopHandCategory(s.into())),
        }
    }
}

#[cfg_attr(coverage_nightly, coverage(off))]
#[cfg(test)]
mod tests {
    use FlopHandCategory::*;

    use super::*;

    impl Arbitrary for FlopHandCategory {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            *g.choose(&Self::ARR_ALL).unwrap()
        }
    }

    impl Arbitrary for PQLFlopHandCategory {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            Self {
                category: Arbitrary::arbitrary(g),
                ord: Arbitrary::arbitrary(g),
            }
        }
    }

    #[test]
    fn test_partial_cmp() {
        let l = PQLFlopHandCategory::from((Nothing, PQLGame::Holdem));
        let r = PQLFlopHandCategory::from((Nothing, PQLGame::ShortDeck));

        assert!(l.partial_cmp(&r).is_none());
    }

    #[test]
    fn test_from_str() {
        assert_eq!(Ok(Nothing), "flopNothing".parse());
        assert_eq!(Ok(UnderPair), "flopuNderpair".parse());
        assert_eq!(Ok(ThirdPair), "flopthIrdpair".parse());
        assert_eq!(Ok(Pocket23), "floppocKet23".parse());
        assert_eq!(Ok(SecondPair), "flopsecoNdpair".parse());
        assert_eq!(Ok(Pocket12), "floppockeT12".parse());
        assert_eq!(Ok(TopPair), "floptoppaiR".parse());
        assert_eq!(Ok(Overpair), "flopOverpair".parse());
        assert_eq!(Ok(BottomTwo), "flopbOttomtwo".parse());
        assert_eq!(Ok(TopAndBottom), "floptoPandbottom".parse());
        assert_eq!(Ok(TopTwo), "floptopTwo".parse());
        assert_eq!(Ok(Trips), "floptripS".parse());
        assert_eq!(Ok(Set), "flopSet".parse());
        assert_eq!(Ok(Straight), "flopsTraight".parse());
        assert_eq!(Ok(Flush), "flopflUsh".parse());
        assert_eq!(Ok(FullHouse), "flopfulLhouse".parse());
        assert_eq!(Ok(Quads), "flopquadS".parse());
        assert_eq!(Ok(StraightFlush), "flopStraightFlush".parse());

        assert_eq!(Ok(Nothing), " flopnothing ".parse(), "should trim");

        assert!("invalid".parse::<FlopHandCategory>().is_err());
    }
}
