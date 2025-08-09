use crate::{
    constants::{MASK64_2, MASK64_S, OFFSET_SUIT},
    prim::math::combinatorics::range_cond_indices,
    range_parser::{ast, parse as parse_range, Error},
    Card, Card64, LocInfo, PQLCardCount, Rank16, Suit4,
};

mod checker;
mod constrain;
mod constrain_rank;
mod constrain_suit;
mod expr;
mod leaf;

use checker::Checker;
use constrain::Constrain;
use constrain_rank::ConstrainRank;
use constrain_suit::ConstrainSuit;
use expr::Expr;
use itertools::Itertools;
use leaf::{Deps, Leaf};
use rustc_hash::FxHashMap;

type Idx = u8;
