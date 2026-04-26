use std::{cmp, str::FromStr};

use crate::{N_FLOP_CATEGORY, ParseError};

/// Category of a hand relative to the flop.
#[cfg_attr(feature = "speedy", derive(speedy::Readable, speedy::Writable))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, derive_more::Display)]
pub enum FlopHandCategory {
    /// No made hand on the flop.
    #[default]
    #[display("FLOPNOTHING")]
    Nothing,
    /// Pocket pair below every board card.
    #[display("FLOPUNDERPAIR")]
    UnderPair,
    /// Pair using the lowest board card.
    #[display("FLOPTHIRDPAIR")]
    ThirdPair,
    /// Pocket pair between the second and third board card.
    #[display("FLOPPOCKET23")]
    Pocket23,
    /// Pair using the middle board card.
    #[display("FLOPSECONDPAIR")]
    SecondPair,
    /// Pocket pair between the top and second board card.
    #[display("FLOPPOCKET12")]
    Pocket12,
    /// Pair using the highest board card.
    #[display("FLOPTOPPAIR")]
    TopPair,
    /// Pocket pair above every board card.
    #[display("FLOPOVERPAIR")]
    Overpair,
    /// Two pair using the two lowest board cards.
    #[display("FLOPBOTTOMTWO")]
    BottomTwo,
    /// Two pair using the highest and lowest board cards.
    #[display("FLOPTOPANDBOTTOM")]
    TopAndBottom,
    /// Two pair using the two highest board cards.
    #[display("FLOPTOPTWO")]
    TopTwo,
    /// Three of a kind using two board cards.
    #[display("FLOPTRIPS")]
    Trips,
    /// Three of a kind using a pocket pair.
    #[display("FLOPSET")]
    Set,
    /// Straight on the flop.
    #[display("FLOPSTRAIGHT")]
    Straight,
    /// Flush on the flop.
    #[display("FLOPFLUSH")]
    Flush,
    /// Full house on the flop.
    #[display("FLOPFULLHOUSE")]
    FullHouse,
    /// Four of a kind on the flop.
    #[display("FLOPQUADS")]
    Quads,
    /// Straight flush on the flop.
    #[display("FLOPSTRAIGHTFLUSH")]
    StraightFlush,
}

type Idx = u8;

impl FlopHandCategory {
    /// Strongest variant.
    pub const MAX: Self = Self::StraightFlush;
    /// Weakest variant.
    pub const MIN: Self = Self::Nothing;

    /// Every variant in ascending order.
    pub const ARR_ALL: [Self; N_FLOP_CATEGORY] = [
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

    const fn to_idx<const SD: bool>(self) -> Idx {
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
            Self::Flush => 14 + (SD as Idx) * 2, // shortdeck: 16
            Self::FullHouse => 15,
            Self::Quads => 17,
            Self::StraightFlush => 18,
        }
    }

    /// Compares two categories under Hold'em (`SD = false`) or Short Deck (`SD = true`) ordering.
    #[must_use]
    pub fn compare<const SD: bool>(self, other: Self) -> cmp::Ordering {
        self.to_idx::<SD>().cmp(&other.to_idx::<SD>())
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

#[cfg(any(test, feature = "quickcheck"))]
#[cfg_attr(coverage_nightly, coverage(off))]
impl quickcheck::Arbitrary for FlopHandCategory {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        *g.choose(&Self::ARR_ALL).unwrap()
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        use FlopHandCategory as Cat;

        assert_eq!(Ok(Cat::Nothing), "flopNothing".parse());
        assert_eq!(Ok(Cat::UnderPair), "flopuNderpair".parse());
        assert_eq!(Ok(Cat::ThirdPair), "flopthIrdpair".parse());
        assert_eq!(Ok(Cat::Pocket23), "floppocKet23".parse());
        assert_eq!(Ok(Cat::SecondPair), "flopsecoNdpair".parse());
        assert_eq!(Ok(Cat::Pocket12), "floppockeT12".parse());
        assert_eq!(Ok(Cat::TopPair), "floptoppaiR".parse());
        assert_eq!(Ok(Cat::Overpair), "flopOverpair".parse());
        assert_eq!(Ok(Cat::BottomTwo), "flopbOttomtwo".parse());
        assert_eq!(Ok(Cat::TopAndBottom), "floptoPandbottom".parse());
        assert_eq!(Ok(Cat::TopTwo), "floptopTwo".parse());
        assert_eq!(Ok(Cat::Trips), "floptripS".parse());
        assert_eq!(Ok(Cat::Set), "flopSet".parse());
        assert_eq!(Ok(Cat::Straight), "flopsTraight".parse());
        assert_eq!(Ok(Cat::Flush), "flopflUsh".parse());
        assert_eq!(Ok(Cat::FullHouse), "flopfulLhouse".parse());
        assert_eq!(Ok(Cat::Quads), "flopquadS".parse());
        assert_eq!(Ok(Cat::StraightFlush), "flopStraightFlush".parse());

        assert_eq!(Ok(Cat::Nothing), " flopnothing ".parse(), "should trim");

        assert!("invalid".parse::<Cat>().is_err());
    }

    #[test]
    fn test_ord_holdem() {
        let mut sorted = FlopHandCategory::ARR_ALL.to_vec();
        sorted.sort_unstable_by(|l, r| l.compare::<false>(*r));

        assert_eq!(sorted, FlopHandCategory::ARR_ALL);
    }

    #[test]
    fn test_ord_shortdeck() {
        let mut sorted = FlopHandCategory::ARR_ALL.to_vec();
        sorted.sort_unstable_by(|l, r| l.compare::<true>(*r));

        let mut sorted_sd = FlopHandCategory::ARR_ALL.to_vec();
        let i_flush = sorted_sd
            .iter()
            .position(|c| *c == FlopHandCategory::Flush)
            .unwrap();
        sorted_sd.swap(i_flush, i_flush + 1);

        assert_eq!(
            sorted_sd[i_flush],
            FlopHandCategory::FullHouse,
            "Fullhouse should be where Flush was"
        );
        assert_eq!(sorted, sorted_sd);
    }
}
