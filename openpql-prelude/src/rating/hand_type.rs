use super::{Display, FromStr, HandRating, N_HANDTYPE, ParseError, cmp};

/// Represents the categorical type of a poker hand.
///
/// This enum classifies poker hands into their standard categories, ordered from
/// weakest to strongest. It is used to quickly identify what type of hand a player
/// has without considering the specific ranks involved.
///
/// # Ordering
/// The variants are ordered from weakest (`HighCard`) to strongest (`StraightFlush`),
/// matching standard poker hand rankings.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, Display)]
pub enum HandType {
    /// No matching cards (default/weakest hand type)
    #[default]
    #[display("HIGH_CARD")]
    HighCard,
    /// Two cards of the same rank
    #[display("PAIR")]
    Pair,
    /// Two different pairs
    #[display("TWO_PAIR")]
    TwoPair,
    /// Three cards of the same rank
    #[display("TRIPS")]
    Trips,
    /// Five cards in sequential rank
    #[display("STRAIGHT")]
    Straight,
    /// Five cards of the same suit
    #[display("FLUSH")]
    Flush,
    /// Three of a kind plus a pair
    #[display("FULL_HOUSE")]
    FullHouse,
    /// Four cards of the same rank
    #[display("QUADS")]
    Quads,
    /// Five cards in sequential rank, all of the same suit
    #[display("STRAIGHT_FLUSH")]
    StraightFlush,
}

type Idx = u8;

impl HandType {
    pub const MAX: Self = Self::StraightFlush;
    pub const MIN: Self = Self::HighCard;

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

    pub fn compare<const SD: bool>(self, other: Self) -> cmp::Ordering {
        self.to_idx::<SD>().cmp(&other.to_idx::<SD>())
    }
}

const MASK_KIND: u16 = 0b1110_0000_0000_0000;
const MASK_LO: u16 = 0b0000_0000_1111_1111;
const N_FLUSH_SET_BITS: u32 = 7;

impl From<HandRating> for HandType {
    /// Extracts the categorical hand type from a `HandRanking`.
    ///
    /// This implementation decodes the bit-packed `HandRanking` to determine
    /// the hand type. It uses bit masking to identify the hand category from
    /// the upper bits and additional bit checks to distinguish between hands
    /// that share the same mask (e.g., Flush vs `FullHouse`, Quads vs `StraightFlush`).
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
