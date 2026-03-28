use super::{Display, Error};

// Error type for `FromStr`
#[cfg_attr(feature = "speedy", derive(speedy::Readable, speedy::Writable))]
#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum ParseError {
    InvalidRank(String),
    InvalidSuit(String),
    InvalidCard(String),
    InvalidHandType(String),
    InvalidFlopHandCategory(String),
    InvalidStreet(String),
    InvalidGame(String),
    InvalidHand(String),
    InvalidPlayer(String),
}

impl Error for ParseError {}
