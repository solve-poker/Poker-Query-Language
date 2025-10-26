use super::{
    Display, FromStr, N_FLOP_CATEGORY, N_HANDTYPE, ParseError, Rank16,
    RatingInner, cmp, fmt,
};

mod flop_hand_category;
mod hand_rating;
mod hand_rating_view;
mod hand_type;
mod idx_three_ranks;
mod idx_two_ranks;

pub use flop_hand_category::*;
pub use hand_rating::HandRating;
pub use hand_rating_view::*;
pub use hand_type::HandType;
use idx_three_ranks::IdxThreeRanks;
use idx_two_ranks::IdxTwoRanks;

#[cfg(test)]
pub mod tests {
    pub use crate::*;

    pub fn mk_rating(ht: HandType, hi: &str, lo: &str) -> HandRating {
        let hi = r16!(hi);
        let lo = r16!(lo);

        match ht {
            HandType::HighCard => HandRating::new_highcard(hi),
            HandType::Pair => HandRating::new_pair(hi, lo),
            HandType::TwoPair => HandRating::new_twopair(hi, lo),
            HandType::Trips => HandRating::new_trips(hi, lo),
            HandType::Straight => HandRating::new_straight(hi),
            HandType::Flush => HandRating::new_flush(hi),
            HandType::FullHouse => HandRating::new_fullhouse(hi, lo),
            HandType::Quads => HandRating::new_quad(hi, lo),
            HandType::StraightFlush => HandRating::new_straightflush(hi),
        }
    }

    pub fn mk_ranking_sd(ht: HandType, hi: &str, lo: &str) -> HandRating {
        match ht {
            HandType::Flush => HandRating::new_flush_sd(r16!(hi)),
            HandType::FullHouse => {
                HandRating::new_fullhouse_sd(r16!(hi), r16!(lo))
            }
            _ => mk_rating(ht, hi, lo),
        }
    }
}
