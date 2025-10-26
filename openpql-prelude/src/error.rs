use super::{Display, Error};
#[cfg(feature = "python")]
use crate::python::*;

// Error type for `FromStr`
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

#[cfg(feature = "python")]
impl From<ParseError> for PyErr {
    fn from(err: ParseError) -> Self {
        Self::new::<PyValueError, _>(err.to_string())
    }
}
