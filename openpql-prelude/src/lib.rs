#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#![cfg_attr(test, allow(clippy::needless_pass_by_value))]
#![cfg_attr(any(test, feature = "python"), allow(clippy::wildcard_imports))]

use std::{
    cmp, error::Error, fmt, hash::Hash, mem::transmute, ops, ops::Not,
    str::FromStr, vec::Vec,
};

use derive_more::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, Deref, Display,
    Index, Into,
};

mod buffer;
mod card;
mod error;
mod eval_flop;
mod eval_rating;
mod game;
#[cfg(feature = "python")]
mod python;
mod rating;
#[cfg(feature = "rand")]
mod rng;

pub use buffer::BufferWrite;
pub use card::{
    Board, Card, Card64, CardCount, CardIdx, CardIter, Flop, HandIter, HandN,
    Rank, Rank16, RankIdx, Suit, Suit4, SuitIdx, SuitMapping,
};
pub use error::ParseError;
use eval_flop::{eval_flop_holdem, eval_flop_omaha};
use eval_rating::{eval_holdem, eval_omaha, eval_shortdeck};
pub use game::{Game, Player, PlayerIdx, Street};
use rating::HandRatingView;
pub use rating::{FlopHandCategory, HandRating, HandType};
#[cfg(feature = "rand")]
pub use rng::CardGen;

type RatingInner = u16;
const N_STRAIGHT: usize = 10;
const N_STRAIGHT_SD: usize = 6;
const N_FLOP_CATEGORY: usize = 18;
const N_HANDTYPE: usize = 9;

#[cfg(any(test, feature = "quickcheck"))]
mod distinct;

#[cfg(any(test, feature = "quickcheck"))]
pub use {card::CardN, distinct::Distinct};

#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

#[cfg(test)]
use tests::*;

#[cfg(test)]
pub mod tests {
    pub use std::{hash::Hasher, str::FromStr};

    pub use derive_more::derive::{Index, Into};
    pub use itertools::Itertools;
    pub use quickcheck::{Arbitrary, TestResult};
    pub use regex::Regex;
    pub use rustc_hash::{FxHashMap, FxHashSet};

    pub use super::{
        CardN,
        rating::{
            HandType,
            tests::{mk_ranking_sd, mk_rating},
        },
    };
}
