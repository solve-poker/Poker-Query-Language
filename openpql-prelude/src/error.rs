use std::error::Error;

// Error type for `FromStr`
/// Parse failure produced by a `FromStr` implementation.
#[cfg_attr(feature = "speedy", derive(speedy::Readable, speedy::Writable))]
#[derive(Debug, Clone, PartialEq, Eq, derive_more::Display)]
pub enum ParseError {
    /// Input is not a valid rank.
    InvalidRank(String),
    /// Input is not a valid suit.
    InvalidSuit(String),
    /// Input is not a valid card.
    InvalidCard(String),
    /// Input is not a valid hand type.
    InvalidHandType(String),
    /// Input is not a valid flop hand category.
    InvalidFlopHandCategory(String),
    /// Input is not a valid street.
    InvalidStreet(String),
    /// Input is not a valid game.
    InvalidGame(String),
    /// Input is not a valid hand.
    InvalidHand(String),
    /// Input is not a valid player.
    InvalidPlayer(String),
}

impl Error for ParseError {}
