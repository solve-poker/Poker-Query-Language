//! Card, rank, suit, and board representations for poker.
//!
//! This module provides core poker primitives including:
//! - Individual cards (`Card`) with rank and suit
//! - Efficient card set operations (`Card64`)
//! - Board state management (`Board`)
//! - Fixed-size hands (`HandN<N>`)
//! - Rank and suit manipulation utilities

use super::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, Deref, Display, FromStr, Hash,
    Index, Into, N_STRAIGHT, N_STRAIGHT_SD, Not, error::ParseError, fmt, ops,
};

mod board;
#[allow(clippy::module_inception)]
mod card;
mod card64;
mod card_idx;
mod card_iter;
mod flop;
mod hand_iter;
mod hand_n;
mod isomorphic;
mod rank;
mod rank16;
mod rank_idx;
mod suit;
mod suit4;
mod suit_idx;
mod suit_mapping;

pub use board::*;
pub use card::*;
pub use card_idx::*;
pub use card_iter::*;
pub use card64::*;
pub use flop::*;
pub use hand_iter::*;
pub use hand_n::*;
pub use rank::*;
pub use rank_idx::*;
pub use rank16::*;
pub use suit::*;
pub use suit_idx::*;
pub use suit_mapping::*;
pub use suit4::*;

pub type CardCount = u8;
type Idx = i8;
type Suit4Inner = u8;
type Rank16Inner = u16;
type Card64Inner = u64;

#[cfg(any(test, feature = "quickcheck"))]
mod card_n;
#[cfg(feature = "rayon")]
mod hand_par_iter;

#[cfg(any(test, feature = "quickcheck"))]
pub use card_n::CardN;
