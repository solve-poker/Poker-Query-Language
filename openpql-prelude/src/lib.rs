//! Card primitives, hand evaluation, and game-tree types for Hold'em and
//! Short Deck poker.

#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#![cfg_attr(test, allow(clippy::needless_pass_by_value))]

mod card;
mod error;
mod eval;
mod game;
mod rating;
#[cfg(feature = "rand")]
mod rng;
/// Game-tree types: actions, history, and tree-building helpers.
pub mod tree;

pub use card::{
    Board, Card, Card64, CardCount, CardIdx, CardIter, Flop, FlushingSuit,
    HandIter, HandN, Idx, IsomorphicCard, IsomorphicFlop, IsomorphicRiver,
    IsomorphicTurn, Rank, Rank16, RankIdx, Suit, Suit4, SuitIdx, SuitMap,
};
pub use error::ParseError;
pub use eval::calculate_payoffs;
pub use game::{Game, Player, PlayerIdx, Street};
pub use rating::{FlopHandCategory, HandRating, HandType};
#[cfg(feature = "rand")]
pub use rng::CardGen;

type RatingInner = u16;
const N_STRAIGHT: usize = 10;
const N_STRAIGHT_SD: usize = 6;
const N_FLOP_CATEGORY: usize = 18;
const N_HANDTYPE: usize = 9;

#[cfg(any(test, feature = "quickcheck"))]
mod testing;

#[cfg(any(test, feature = "quickcheck"))]
pub use testing::{CardN, Distinct};

#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

#[cfg(test)]
use tests::*;

#[cfg_attr(coverage_nightly, coverage(off))]
#[cfg(test)]
pub mod tests {
    pub use std::{hash::Hasher, str::FromStr};

    pub use itertools::Itertools;
    pub use quickcheck::{Arbitrary, TestResult};
    pub use regex::Regex;
    pub use rustc_hash::{FxHashMap, FxHashSet};
    #[cfg(feature = "serde")]
    pub use serde_test::{
        Compact, Configure, Token, assert_de_tokens_error, assert_tokens,
    };
    #[cfg(feature = "serde")]
    pub use serde_utils::{to_i, to_s};

    pub use super::{
        CardN,
        rating::{
            HandType,
            tests::{mk_ranking_sd, mk_rating},
        },
    };

    pub mod serde_utils {
        use super::super::{Card, CardIdx};

        #[must_use]
        pub fn to_s(card: Card) -> &'static str {
            Box::leak(card.to_string().into_boxed_str())
        }

        #[must_use]
        pub fn to_i(card: Card) -> u8 {
            CardIdx::from(card).0.cast_unsigned()
        }
    }
}
