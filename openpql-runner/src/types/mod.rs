use openpql_prelude as prelude;
// Range Values:
pub use pql_board_range::PQLBoardRange;
pub use pql_range::PQLRange;

use super::*;

// Primitives
pub type PQLInteger = PQLLong;
pub type PQLCardCount = prelude::CardCount;
pub type PQLDouble = f64;
pub type PQLEquity = PQLDouble;
pub type PQLLong = i64;
pub type PQLPlayerCount = PQLCardCount;
pub type PQLBoolean = bool;
pub type PQLString = String;
pub use pql_fraction::PQLFraction;
pub use pql_numeric::PQLNumeric;

// Card Values:
pub type DeadCards = PQLCardSet;
pub type PQLBoard = prelude::Board;
pub type PQLCard = prelude::Card;
pub type PQLCardSet = prelude::Card64;
pub type PQLFlop = prelude::Flop;
pub type PQLRank = prelude::Rank;
pub type PQLRankSet = prelude::Rank16;
pub type PQLSuit = prelude::Suit;
pub type PQLSuitSet = prelude::Suit4;

// Category Values:
pub type PQLFlopHandCategory = prelude::FlopHandCategory;
pub type PQLHiRating = prelude::HandRating;
pub type PQLLoRating = PQLHiRating;
pub struct PQLHandRanking {}
pub type PQLHandType = prelude::HandType;

// Game Values:
pub type PQLPlayer = prelude::Player;
pub use pql_street::PQLStreet;
pub type PQLGame = prelude::Game;

mod pql_board_range;
mod pql_fraction;
mod pql_numeric;
mod pql_range;
mod pql_street;
mod pql_type;

pub use pql_type::*;
