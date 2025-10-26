use super::{HandRating, HandType, Rank16};

/// Hand Ranking Wrapper
/// helper struct to view details of a thin `HandRating`
#[derive(Clone, Debug)]
pub struct HandRatingView {
    pub hand_type: HandType,
    pub high: Rank16,
    pub low: Rank16,
}

impl From<(HandType, Rank16)> for HandRatingView {
    fn from((hand_type, high): (HandType, Rank16)) -> Self {
        Self {
            hand_type,
            high,
            low: Rank16::default(),
        }
    }
}

impl From<(HandType, (Rank16, Rank16))> for HandRatingView {
    fn from((hand_type, (high, low)): (HandType, (Rank16, Rank16))) -> Self {
        Self {
            hand_type,
            high,
            low,
        }
    }
}

impl From<HandRating> for HandRatingView {
    fn from(rating: HandRating) -> Self {
        let ht = HandType::from(rating);

        match ht {
            HandType::HighCard => (ht, rating.parse_highcard()).into(),
            HandType::Pair => (ht, rating.parse_pair()).into(),
            HandType::TwoPair => (ht, rating.parse_twopair()).into(),
            HandType::Trips => (ht, rating.parse_trips()).into(),
            HandType::Straight => (ht, rating.parse_straight()).into(),
            HandType::Flush => (ht, rating.parse_flush()).into(),
            HandType::FullHouse => (ht, rating.parse_fullhouse()).into(),
            HandType::Quads => (ht, rating.parse_quad()).into(),
            HandType::StraightFlush => {
                (ht, rating.parse_straightflush()).into()
            }
        }
    }
}
