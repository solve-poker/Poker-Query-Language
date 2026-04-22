use std::{error::Error, fmt};

use derive_more::Display;

use crate::tree::{Chip, PlayerIdx};

type Bet = Chip;
type ToCall = Chip;
type PlayerRemaining = Chip;

/// Parse failure for tree types.
#[derive(Clone, PartialEq, Eq, Display, Debug)]
pub enum TreeParseError {
    /// Input is not a valid action.
    #[display("InvalidAction: {_0}")]
    InvalidAction(String),
    /// Input is not a valid history.
    #[display("InvalidHistory: {_0}")]
    InvalidHistory(String),
}

impl Error for TreeParseError {}

/// Rule violation detected while validating a game-tree action.
#[derive(Clone, PartialEq, Eq, Display)]
pub enum GameTreeError {
    /// Bet does not match the amount required to call.
    #[display("BetAmountInvalid P{_0} {_1} facing: {_2} stack: {_3}")]
    BetAmountInvalid(PlayerIdx, Bet, ToCall, PlayerRemaining),
    /// Bet is larger than the player's remaining stack.
    #[display("BetExceedsStack P{_0} {_1} > {_2}")]
    BetExceedsStack(PlayerIdx, Bet, PlayerRemaining),
    /// Raise is below the minimum legal raise.
    #[display("RaiseTooSmall P{_0} bet: {_1}")]
    RaiseTooSmall(PlayerIdx, Bet),
    /// Raise is not permitted in the current state.
    #[display("RaiseNotAllowed P{_0}")]
    RaiseNotAllowed(PlayerIdx),
    /// Action is not legal in the current state.
    #[display("ActionNotAllowed")]
    ActionNotAllowed,
}

impl fmt::Debug for GameTreeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl Error for GameTreeError {}
