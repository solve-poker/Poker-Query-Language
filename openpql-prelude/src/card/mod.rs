//! Card, rank, suit, and board primitives.

#[allow(clippy::module_inception)]
mod card;
mod collection;
mod equiv;
mod idx;
mod iter;
mod rank;
mod set;
mod suit;

pub use card::Card;
pub use collection::{Board, Flop, HandN};
pub use equiv::{
    FlushingSuit, IsomorphicCard, IsomorphicFlop, IsomorphicRiver,
    IsomorphicTurn, SuitMap,
};
pub use idx::{CardIdx, RankIdx, SuitIdx};
pub use iter::{CardIter, HandIter};
pub use rank::Rank;
pub use set::{Card64, Rank16, Suit4};
pub use suit::Suit;

/// Card count type used throughout the crate.
pub type CardCount = u8;
/// Signed integer representation of a card, rank, or suit.
pub type Idx = i8;
type Suit4Inner = u8;
type Rank16Inner = u16;
type Card64Inner = u64;
