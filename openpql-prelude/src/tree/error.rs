use std::{error::Error, fmt};

use crate::tree::{Chip, PlayerIdx};

type Bet = Chip;
type ToCall = Chip;
type PlayerRemaining = Chip;

/// Parse failure for tree types.
#[derive(Clone, PartialEq, Eq, derive_more::Display, Debug)]
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
#[derive(Clone, PartialEq, Eq, derive_more::Display)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug_matches_display() {
        let cases = [
            GameTreeError::BetAmountInvalid(0, 1, 2, 3),
            GameTreeError::BetExceedsStack(1, 100, 50),
            GameTreeError::RaiseTooSmall(2, 5),
            GameTreeError::RaiseNotAllowed(3),
            GameTreeError::ActionNotAllowed,
        ];
        for e in cases {
            assert_eq!(format!("{e:?}"), e.to_string());
        }
    }

    #[test]
    fn test_tree_parse_error_display() {
        assert_eq!(
            TreeParseError::InvalidAction("x".into()).to_string(),
            "InvalidAction: x"
        );
        assert_eq!(
            TreeParseError::InvalidHistory("y".into()).to_string(),
            "InvalidHistory: y"
        );
    }
}
