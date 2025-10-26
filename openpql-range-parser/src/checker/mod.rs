type IdxVec<const N: usize> = SmallVec<[Idx; N]>;
type RankDiff = i8;

use super::{
    Array, Card, Card64, Error, From, Idx, Itertools, LazyLock, LocInfo,
    PhantomData, Rank, Rank16, SmallVec, Suit, Suit4, ast,
    ast::{
        List, ListElem, RangeCard, RankVar, Span, SpanElem, SuitVar, Term,
        TermElem,
    },
    ops, parse_expr,
};

pub type BoardRangeChecker<const SD: bool> = Checker<5, true, SD>;
pub type RangeChecker<const N: usize, const SD: bool> = Checker<N, false, SD>;

#[allow(clippy::module_inception)]
mod checker;
mod constrain;
mod constrain_rank;
mod constrain_suit;
mod expr;
mod leaf;
mod perm_indices;
mod var_condition;
mod var_condition_rank;
mod var_condition_suit;

pub use checker::Checker;
use constrain::Constrain;
use constrain_rank::ConstrainRank;
use constrain_suit::ConstrainSuit;
use expr::Expr;
use leaf::{Deps, Leaf};
use perm_indices::range_cond_indices;
use var_condition::VarCondition;
use var_condition_rank::VarConditionRank;
use var_condition_suit::VarConditionSuit;
