use std::{error::Error, fmt};

use derive_more::Display;

use crate::tree::{Chip, PlayerIdx};

type Bet = Chip;
type ToCall = Chip;
type PlayerRemaining = Chip;

#[derive(Clone, PartialEq, Eq, Display)]
pub enum GameTreeError {
    #[display("BetAmountInvalid P{_0} {_1} facing: {_2} stack: {_3}")]
    BetAmountInvalid(PlayerIdx, Bet, ToCall, PlayerRemaining),
    #[display("BetExceedsStack P{_0} {_1} > {_2}")]
    BetExceedsStack(PlayerIdx, Bet, PlayerRemaining),
    #[display("RaiseTooSmall P{_0} bet: {_1}")]
    RaiseTooSmall(PlayerIdx, Bet),
    #[display("RaiseNotAllowed P{_0}")]
    RaiseNotAllowed(PlayerIdx),
    #[display("ActionNotAllowed")]
    ActionNotAllowed,
}

impl fmt::Debug for GameTreeError {
    #[cfg_attr(coverage_nightly, coverage(off))]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl Error for GameTreeError {}
