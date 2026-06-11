use openpql_core::functions as core;

use super::*;

mod best_hi_rating;
mod board_in_range;
mod board_ranks;
mod board_suit_count;
mod context;
mod duplicated_board_ranks;
mod duplicated_hand_ranks;
mod exact_flop_hand_category;
mod exact_hand_type;
mod flop_hand_category;
mod flushing_board;
mod fractional_river_equity;
mod hand_board_intersections;
mod hand_ranks;
mod hand_type;
mod has_second_board_rank;
mod has_top_board_rank;
mod hi_rating;
mod hvhequity;
mod in_range;
mod intersecting_hand_ranks;
mod max_hi_rating;
mod max_rank;
mod min_flop_hand_category;
mod min_hand_type;
mod min_hi_rating;
mod min_hvhequity;
mod min_rank;
mod monotone_board;
mod nonintersecting_hand_ranks;
mod nth_rank;
mod nut_hi;
mod nut_hi_for_hand_type;
mod nut_hi_outs;
mod overpair;
mod paired_board;
mod pocket_pair;
mod rainbow_board;
mod rank_count;
mod rate_hi_hand;
mod river_card;
mod river_equity;
mod scoops;
mod straight_board;
mod ties_hi;
mod to_card;
mod to_rank;
mod turn_card;
mod twotone_board;
mod winning_hand_type;
mod wins_hi;

pub use best_hi_rating::*;
pub use board_in_range::*;
pub use board_ranks::*;
pub use board_suit_count::*;
#[cfg(test)]
pub use context::tests::TestPQLFnContext;
pub use context::*;
pub use duplicated_board_ranks::*;
pub use duplicated_hand_ranks::*;
pub use exact_flop_hand_category::*;
pub use exact_hand_type::*;
pub use flop_hand_category::*;
pub use flushing_board::*;
pub use fractional_river_equity::*;
pub use hand_board_intersections::*;
pub use hand_ranks::*;
pub use hand_type::*;
pub use has_second_board_rank::*;
pub use has_top_board_rank::*;
pub use hi_rating::*;
pub use hvhequity::*;
pub use in_range::*;
pub use intersecting_hand_ranks::*;
pub use max_hi_rating::*;
pub use max_rank::*;
pub use min_flop_hand_category::*;
pub use min_hand_type::*;
pub use min_hi_rating::*;
pub use min_hvhequity::*;
pub use min_rank::*;
pub use monotone_board::*;
pub use nonintersecting_hand_ranks::*;
pub use nth_rank::*;
pub use nut_hi::*;
pub use nut_hi_for_hand_type::*;
pub use nut_hi_outs::*;
pub use overpair::*;
pub use paired_board::*;
pub use pocket_pair::*;
pub use rainbow_board::*;
pub use rank_count::*;
pub use rate_hi_hand::*;
pub use river_card::*;
pub use river_equity::*;
pub use scoops::*;
pub use straight_board::*;
pub use ties_hi::*;
pub use to_card::*;
pub use to_rank::*;
pub use turn_card::*;
pub use twotone_board::*;
pub use winning_hand_type::*;
pub use wins_hi::*;

pub trait PQLFn: fmt::Debug + Sync {
    fn arg_types(&self) -> Vec<PQLType>;
    fn rtn_type(&self) -> PQLType;
    fn execute(
        &self,
        ctx: &mut VmExecContext,
    ) -> Result<VmStackValue, PQLErrorKind>;
}

impl FromStr for &dyn PQLFn {
    type Err = PQLErrorKind;

    pqlfn_fromstr!(Err(PQLErrorKind::UnrecognizedFunction));
}
