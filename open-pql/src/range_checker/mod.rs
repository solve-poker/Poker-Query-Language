use crate::{
    Card, Card64, LocInfo, PQLCardCount, Rank16, Suit4,
    constants::{MASK64_2, MASK64_S, OFFSET_SUIT},
    prim::math::combinatorics::range_cond_indices,
    range_parser::{Error, ast, parse as parse_range},
};

mod cache;
mod checker;
mod constrain;
mod constrain_rank;
mod constrain_suit;
mod expr;
mod leaf;

pub use cache::CachedChecker;
use checker::Checker;
use constrain::Constrain;
use constrain_rank::ConstrainRank;
use constrain_suit::ConstrainSuit;
use expr::Expr;
use itertools::Itertools;
use leaf::{Deps, Leaf};
use rustc_hash::FxHashMap;

type Idx = u8;
