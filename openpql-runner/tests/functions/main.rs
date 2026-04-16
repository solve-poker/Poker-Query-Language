//! Integration tests for individual PQL functions.
//!
//! One file per PQL function. Each submodule hits its function through
//! `PQLRunner::run` with fully-specified boards so results are
//! deterministic.
//!
//! **Rules** (see `../common/mod.rs` for the full list):
//!
//! 1. One selector per `select` statement. Runner bug: multi-selector
//!    queries only give the first selector its N trials.
//! 2. Use `assert_count_all` / `assert_count_none` for boolean functions;
//!    `run_count`, `run_trials`, `run_ok` for everything else.
//! 3. Fully specify boards so results don't depend on trial count
//!    (debug: 100, release: 60000).

#[path = "../common/mod.rs"]
mod common;

mod best_hi_rating;
mod board_in_range;
mod board_ranks;
mod board_suit_count;
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
