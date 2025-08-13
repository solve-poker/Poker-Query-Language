use std::{
    fmt,
    hash::Hash,
    iter, mem,
    ops::{Index, Not},
    slice,
    str::FromStr,
    vec::Vec,
};

use derive_more::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, Display, From, Into,
};

use super::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, PQLCardCount, ParseError,
    constants::{
        IDX_RIVER, IDX_TURN, MASK16_RANKS, MASK64_2, MASK64_ALL, N_CARDS,
        N_FLOP, N_RANKS, N_SUITS, OFFSET_C, OFFSET_D, OFFSET_H, OFFSET_S,
        OFFSET_SUIT, RANK_NAMES, SUIT_NAMES, U16_LEADING_ONE,
    },
    prim,
};

mod board;
mod card;
mod card64;
mod card_iter;
mod hand_iter;
mod hand_n;
mod isomorphic;
mod rank;
mod rank16;
mod rank16_iter;
mod rank_idx;
mod suit;
mod suit4;
mod suit_idx;
mod suit_mapping;

pub use board::*;
pub use card::*;
pub use card_iter::*;
pub use card64::*;
pub use hand_iter::*;
pub use hand_n::*;
pub use isomorphic::*;
pub use rank::*;
pub use rank_idx::*;
pub use rank16::*;
pub use rank16_iter::*;
pub use suit::*;
pub use suit_idx::*;
pub use suit_mapping::*;
pub use suit4::*;

pub type Hand = [Card];

/// Card count type
pub type CardCount = u8;
