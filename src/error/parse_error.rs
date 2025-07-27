// Error type for `FromStr`
#[derive(Debug, Clone, PartialEq, Eq)]
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
    InvalidFunctionName(String),
}
