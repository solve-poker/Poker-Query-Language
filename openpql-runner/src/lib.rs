#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#![cfg_attr(test, allow(clippy::needless_pass_by_value))]
#![cfg_attr(test, allow(clippy::missing_panics_doc))]
#![allow(clippy::wildcard_imports)]
#![allow(unused_imports)]

use std::{
    any::Any,
    borrow::Borrow,
    cmp, convert, fmt, io, mem,
    num::{ParseFloatError, ParseIntError},
    ops, ptr,
    rc::Rc,
    str::FromStr,
};

use bitflags::bitflags;
use derive_more::{Display, Into, TryInto};
use openpql_macro::*;
pub use openpql_pql_parser::parse_pql;
use openpql_pql_parser::{Error as SyntaxError, *};
use openpql_prelude::{CardGen, HandN, ParseError, PlayerIdx};
use openpql_range_parser::{
    BoardRangeChecker, Error as RangeError, RangeChecker,
};
use runner_output::*;

mod error;
mod output_aggregator;
mod runner_output;
// TODO: remove
#[cfg_attr(coverage_nightly, coverage(off))]
mod functions;
mod helper_loc;
mod runner;
mod types;
mod util;
mod vm;

pub use error::*;
use functions::*;
use helper_loc::*;
use output_aggregator::*;
pub use runner::*;
#[cfg(test)]
pub use tests::*;
pub use types::*;
use util::*;
use vm::{
    Vm, VmBinOpCmp, VmCache, VmExecContext, VmProgram, VmSampledData,
    VmStackValue,
};

type HeapIdx = usize;
type FractionInner = u8;
type RangeSrc = String;
type FnCheckRange = Box<dyn Fn(&[PQLCard]) -> bool>;

fn parse_cards(text: &str) -> Option<PQLCardSet> {
    let mut res = PQLCardSet::default();
    let mut iter = text.chars().filter(|c| !c.is_whitespace());

    while let Some(rank) = iter.next() {
        let suit = iter.next()?;

        dbg!(suit);
        res.set(PQLCard::new(
            PQLRank::from_char(rank)?,
            PQLSuit::from_char(suit)?,
        ));
    }

    Some(res)
}

#[cfg(test)]
pub mod tests {
    pub use std::fmt::Write;

    pub use itertools::Itertools;
    use openpql_pql_parser::*;
    pub use openpql_prelude::{CardN, c64, card, cards, r16};
    pub use quickcheck::{Arbitrary, TestResult};
    pub use quickcheck_macros::quickcheck;
    pub use rand::{SeedableRng, prelude::*, rngs};

    pub use super::{
        PQLBoardRange, PQLGame, PQLHiRating, PQLRange,
        functions::{PQLFnContext, TestPQLFnContext, rate_hi_hand},
    };
    use super::{PQLCard, PQLCardCount, PQLError, PQLErrorKind, PQLRank};

    pub fn count_suits(cs: &[PQLCard]) -> PQLCardCount {
        cs.iter().map(|c| c.suit).unique().count().to_le_bytes()[0]
    }

    pub fn get_ranks(cs: &[PQLCard]) -> Vec<PQLRank> {
        cs.iter().map(|c| c.rank).collect()
    }

    pub fn mk_ranges(
        game: PQLGame,
        player: &[&str],
        board: &str,
    ) -> (Vec<PQLRange>, PQLBoardRange) {
        let player_ranges = player
            .iter()
            .map(|&s| (game, s).try_into().unwrap())
            .collect();

        let board_range = (game, board).try_into().unwrap();

        (player_ranges, board_range)
    }

    pub fn mk_rating(text: &str) -> PQLHiRating {
        rate_hi_hand(&PQLFnContext::default(), &text.to_string()).unwrap()
    }
}
