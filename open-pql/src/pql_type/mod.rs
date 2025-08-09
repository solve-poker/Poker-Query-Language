use super::{
    prim::math::combinatorics::{
        combination_of_2_index_to_ranks, combination_of_3_index_to_ranks,
    },
    *,
};

mod hand_rating_view;
mod hand_type_ordering;
mod pql_board_range;
mod pql_boolean;
mod pql_card;
mod pql_card_count;
mod pql_double;
mod pql_equity;
mod pql_flop_hand_category;
mod pql_fraction;
mod pql_game;
mod pql_hand_ranking;
mod pql_hand_type;
mod pql_hi_rating;
mod pql_integer;
mod pql_lo_rating;
mod pql_long;
mod pql_numeric;
mod pql_player;
mod pql_player_count;
mod pql_range;
mod pql_rank;
mod pql_rank_set;
mod pql_street;
mod pql_string;
mod pql_suit;
mod pql_suit_set;
mod rating_memory_layout;

pub use hand_rating_view::HandRatingView;
pub use hand_type_ordering::HandTypeOrd;
pub use pql_board_range::PQLBoardRange;
pub use pql_boolean::PQLBoolean;
pub use pql_card::PQLCard;
pub use pql_card_count::PQLCardCount;
pub use pql_double::PQLDouble;
pub use pql_equity::PQLEquity;
pub use pql_flop_hand_category::*;
pub use pql_fraction::PQLFraction;
pub use pql_game::PQLGame;
pub use pql_hand_type::*;
pub use pql_hi_rating::PQLHiRating;
pub use pql_integer::PQLInteger;
pub use pql_lo_rating::PQLLoRating;
pub use pql_long::PQLLong;
pub use pql_numeric::PQLNumeric;
pub use pql_player::PQLPlayer;
pub use pql_player_count::PQLPlayerCount;
pub use pql_range::PQLRange;
pub use pql_rank::PQLRank;
pub use pql_rank_set::PQLRankSet;
pub use pql_street::PQLStreet;
pub use pql_string::PQLString;
pub use pql_suit::PQLSuit;
pub use pql_suit_set::PQLSuitSet;
use rating_memory_layout::RatingMemoryLayout;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display)]
pub enum PQLType {
    BoardRange,
    Boolean,
    Card,
    CardCount,
    Double,
    Equity,
    FlopHandCategory,
    Fraction,
    HandRanking,
    HandType,
    HiRating,
    Integer,
    Long,
    LoRating,
    Numeric,
    Player,
    PlayerCount,
    Range,
    Rank,
    RankSet,
    Street,
    String,
}

#[cfg(test)]
mod tests {
    use PQLType::*;

    use super::*;

    impl PQLType {
        pub(crate) const fn is_num(self) -> bool {
            matches!(
                self,
                Self::Double
                    | Self::Equity
                    | Self::Long
                    | Self::Integer
                    | Self::CardCount
                    | Self::PlayerCount
                    | Self::Fraction
                    | Self::Numeric
            )
        }
    }

    impl Arbitrary for PQLType {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            #[allow(unused)]
            const fn completeness_check(e: PQLType) {
                match e {
                    BoardRange | Boolean | Card | CardCount | Double
                    | Equity | FlopHandCategory | Fraction | HandRanking
                    | HandType | HiRating | Integer | Long | LoRating
                    | Numeric | Player | PlayerCount | Range | Rank
                    | RankSet | Street | String => (),
                }
            }

            *g.choose(&[
                BoardRange,
                Boolean,
                Card,
                CardCount,
                Double,
                Equity,
                FlopHandCategory,
                Fraction,
                HandRanking,
                HandType,
                HiRating,
                Integer,
                Long,
                LoRating,
                Numeric,
                Player,
                PlayerCount,
                Range,
                Rank,
                RankSet,
                Street,
                String,
            ])
            .unwrap()
        }
    }
}
