use std::{cmp, str::FromStr};

use crate::{HandRating, N_HANDTYPE, ParseError};

/// Category of a poker hand, from `HighCard` to `StraightFlush`.
#[cfg_attr(feature = "speedy", derive(speedy::Readable, speedy::Writable))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, derive_more::Display)]
pub enum HandType {
    /// High card.
    #[default]
    #[display("HIGH_CARD")]
    HighCard,
    /// One pair.
    #[display("PAIR")]
    Pair,
    /// Two pair.
    #[display("TWO_PAIR")]
    TwoPair,
    /// Three of a kind.
    #[display("TRIPS")]
    Trips,
    /// Straight.
    #[display("STRAIGHT")]
    Straight,
    /// Flush.
    #[display("FLUSH")]
    Flush,
    /// Full house.
    #[display("FULL_HOUSE")]
    FullHouse,
    /// Four of a kind.
    #[display("QUADS")]
    Quads,
    /// Straight flush.
    #[display("STRAIGHT_FLUSH")]
    StraightFlush,
}

type Idx = u8;

impl HandType {
    /// Strongest variant.
    pub const MAX: Self = Self::StraightFlush;
    /// Weakest variant.
    pub const MIN: Self = Self::HighCard;

    /// Every variant in ascending order.
    pub const ARR_ALL: [Self; N_HANDTYPE] = [
        Self::HighCard,
        Self::Pair,
        Self::TwoPair,
        Self::Trips,
        Self::Straight,
        Self::Flush,
        Self::FullHouse,
        Self::Quads,
        Self::StraightFlush,
    ];

    const fn to_idx<const SD: bool>(self) -> Idx {
        match self {
            Self::HighCard => 0,
            Self::Pair => 1,
            Self::TwoPair => 2,
            Self::Trips => 3,
            Self::Straight => 4,
            Self::Flush => 5 + (SD as Idx) * 2, // shortdeck: 7
            Self::FullHouse => 6,
            Self::Quads => 8,
            Self::StraightFlush => 9,
        }
    }

    /// Compares two hand types under Hold'em (`SD = false`) or Short Deck (`SD = true`) ordering.
    #[must_use]
    pub fn compare<const SD: bool>(self, other: Self) -> cmp::Ordering {
        self.to_idx::<SD>().cmp(&other.to_idx::<SD>())
    }
}

const MASK_KIND: u16 = 0b1110_0000_0000_0000;
const MASK_LO: u16 = 0b0000_0000_1111_1111;
const N_FLUSH_SET_BITS: u32 = 7;

impl From<HandRating> for HandType {
    fn from(hand_ranking: HandRating) -> Self {
        match hand_ranking.0 & MASK_KIND {
            HandRating::MASK_QUADS => {
                if hand_ranking.0 & MASK_LO == 0 {
                    Self::StraightFlush
                } else {
                    Self::Quads
                }
            }
            HandRating::MASK_FULLHOUSE | HandRating::MASK_FLUSH => {
                if hand_ranking.0.count_ones() == N_FLUSH_SET_BITS {
                    Self::Flush
                } else {
                    Self::FullHouse
                }
            }
            HandRating::MASK_STRAIGHT => Self::Straight,
            HandRating::MASK_TRIPS => Self::Trips,
            HandRating::MASK_TWOPAIR => Self::TwoPair,
            HandRating::MASK_PAIR => Self::Pair,
            _ => Self::HighCard,
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
            "straightflush" => Ok(Self::StraightFlush),
            _ => Err(ParseError::InvalidHandType(s.into())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ord_holdem() {
        let mut sorted = HandType::ARR_ALL.to_vec();
        sorted.sort_unstable_by(|l, r| l.compare::<false>(*r));

        assert_eq!(sorted, HandType::ARR_ALL);
    }

    #[test]
    fn test_ord_shortdeck() {
        let mut sorted = HandType::ARR_ALL.to_vec();
        sorted.sort_unstable_by(|l, r| l.compare::<true>(*r));

        assert_eq!(
            sorted,
            [
                HandType::HighCard,
                HandType::Pair,
                HandType::TwoPair,
                HandType::Trips,
                HandType::Straight,
                HandType::FullHouse,
                HandType::Flush, // <--- flush is stronger
                HandType::Quads,
                HandType::StraightFlush,
            ]
        );
    }

    #[test]
    fn test_from_str() {
        fn assert_str(s: &str, expected: HandType) {
            assert_eq!(s.parse(), Ok(expected));
        }

        assert_str("highcard     ", HandType::HighCard);
        assert_str("pair         ", HandType::Pair);
        assert_str("twopair      ", HandType::TwoPair);
        assert_str("trips        ", HandType::Trips);
        assert_str("straight     ", HandType::Straight);
        assert_str("fullhouse    ", HandType::FullHouse);
        assert_str("flush        ", HandType::Flush);
        assert_str("quads        ", HandType::Quads);
        assert_str("straightflush", HandType::StraightFlush);

        assert_eq!(
            "invalid".parse::<HandType>(),
            Err(ParseError::InvalidHandType("invalid".to_string()))
        );
    }
}
