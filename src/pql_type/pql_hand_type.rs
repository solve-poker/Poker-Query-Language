use super::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, From, Display)]
#[display("{ht}")]
pub struct PQLHandType {
    pub ht: HandType,
    pub ord: HandTypeOrd,
}

impl PQLHandType {
    pub fn min(g: PQLGame) -> Self {
        Self {
            ht: HandType::MIN,
            ord: g.into(),
        }
    }

    pub fn max(g: PQLGame) -> Self {
        Self {
            ht: HandType::MAX,
            ord: g.into(),
        }
    }
}

impl PartialOrd for PQLHandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.ord == other.ord {
            self.ht
                .to_u8(self.ord)
                .partial_cmp(&other.ht.to_u8(self.ord))
        } else {
            None
        }
    }
}

impl From<(HandType, PQLGame)> for PQLHandType {
    fn from(t: (HandType, PQLGame)) -> Self {
        Self {
            ht: t.0,
            ord: t.1.into(),
        }
    }
}

impl TryFrom<(&str, PQLGame)> for PQLHandType {
    type Error = ParseError;

    fn try_from((s, g): (&str, PQLGame)) -> Result<Self, Self::Error> {
        Ok((s.parse()?, g).into())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, Display)]
pub enum HandType {
    #[default]
    #[display("HIGH_CARD")]
    HighCard,
    #[display("PAIR")]
    Pair,
    #[display("TWO_PAIR")]
    TwoPair,
    #[display("TRIPS")]
    Trips,
    #[display("STRAIGHT")]
    Straight,
    #[display("FLUSH")]
    Flush,
    #[display("FULL_HOUSE")]
    FullHouse,
    #[display("QUADS")]
    Quads,
    #[display("STRAIGHT_FLUSH")]
    StraightFlush,
}

impl HandType {
    pub const MAX: Self = Self::StraightFlush;
    pub const MIN: Self = Self::HighCard;

    pub const ARR_ALL: [Self; 10] = [
        Self::HighCard,
        Self::Pair,
        Self::TwoPair,
        Self::Trips,
        Self::Straight,
        Self::Flush,
        Self::FullHouse,
        Self::Quads,
        Self::StraightFlush,
        Self::StraightFlush,
    ];

    pub(super) const fn to_layout(self) -> RatingMemoryLayout {
        match self {
            Self::Straight | Self::StraightFlush => RatingMemoryLayout::Rank,
            Self::FullHouse | Self::Quads => RatingMemoryLayout::RankRank,
            Self::Trips => RatingMemoryLayout::RankComb2,
            Self::TwoPair => RatingMemoryLayout::Comb2Rank,
            Self::Pair => RatingMemoryLayout::RankComb3,
            Self::HighCard | Self::Flush => RatingMemoryLayout::Rank13,
        }
    }
}

impl HandType {
    const fn to_u8(self, ord: HandTypeOrd) -> u8 {
        match self {
            Self::HighCard => 0,
            Self::Pair => 1,
            Self::TwoPair => 2,
            Self::Trips => 3,
            Self::Straight => 4,
            Self::Flush => match ord {
                HandTypeOrd::Standard => 5,
                HandTypeOrd::Shortdeck => 6,
            },
            Self::FullHouse => match ord {
                HandTypeOrd::Standard => 6,
                HandTypeOrd::Shortdeck => 5,
            },
            Self::Quads => 7,
            Self::StraightFlush => 8,
        }
    }
}

impl FromStr for HandType {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().trim() {
            "highcard" => Ok(Self::HighCard),
            "pair" => Ok(Self::Pair),
            "twopair" => Ok(Self::TwoPair),
            "trips" => Ok(Self::Trips),
            "straight" => Ok(Self::Straight),
            "flush" => Ok(Self::Flush),
            "fullhouse" => Ok(Self::FullHouse),
            "quads" => Ok(Self::Quads),
            "straightflush" | "royalflush" => Ok(Self::StraightFlush),
            _ => Err(ParseError::InvalidHandType(s.into())),
        }
    }
}

impl From<(PQLGame, PQLHiRating)> for HandType {
    fn from((game, rating): (PQLGame, PQLHiRating)) -> Self {
        let ord: HandTypeOrd = game.into();

        ord.i16_to_hand_type(rating.to_i16())
    }
}

#[cfg(test)]
mod tests {
    use self::{HandType::*, RatingMemoryLayout as ML};
    use super::*;

    impl Arbitrary for HandType {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            *g.choose(&Self::ARR_ALL).unwrap()
        }
    }

    impl Arbitrary for PQLHandType {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            Self {
                ht: Arbitrary::arbitrary(g),
                ord: Arbitrary::arbitrary(g),
            }
        }
    }

    #[test]
    fn test_partial_cmp() {
        let l = PQLHandType::from((Flush, PQLGame::Holdem));
        let r = PQLHandType::from((Flush, PQLGame::ShortDeck));

        assert!(l.partial_cmp(&r).is_none());
    }

    #[test]
    fn test_to_layout() {
        assert_eq!(HighCard.to_layout(), ML::Rank13);
        assert_eq!(Pair.to_layout(), ML::RankComb3);
        assert_eq!(TwoPair.to_layout(), ML::Comb2Rank);
        assert_eq!(Trips.to_layout(), ML::RankComb2);
        assert_eq!(Straight.to_layout(), ML::Rank);
        assert_eq!(Flush.to_layout(), ML::Rank13);
        assert_eq!(FullHouse.to_layout(), ML::RankRank);
        assert_eq!(Quads.to_layout(), ML::RankRank);
        assert_eq!(StraightFlush.to_layout(), ML::Rank);
        assert_eq!(StraightFlush.to_layout(), ML::Rank);
    }

    #[test]
    fn test_from_str() {
        assert_eq!(Ok(HighCard), "highcard".parse());
        assert_eq!(Ok(Pair), "pair".parse());
        assert_eq!(Ok(TwoPair), "twopair".parse());
        assert_eq!(Ok(Trips), "trips".parse());
        assert_eq!(Ok(Straight), "straight".parse());
        assert_eq!(Ok(Flush), "flush".parse());
        assert_eq!(Ok(FullHouse), "fullhouse".parse());
        assert_eq!(Ok(Quads), "quads".parse());
        assert_eq!(Ok(StraightFlush), "straightflush".parse());
        assert_eq!(Ok(StraightFlush), "royalflush".parse());

        assert_eq!(Ok(HighCard), "  HighCARD  ".parse());
        assert!("invalid".parse::<HandType>().is_err());
    }

    #[test]
    fn test_from_game_rating() {
        let game = PQLGame::Holdem;
        let ht = FullHouse;
        let hi = r16!("A");
        let lo = r16!("K");
        let view: HandRatingView = (game, ht, hi, lo).into();

        assert!(ht == (game, view.rating).into());
    }
}
