use std::{
    fmt,
    hash::Hash,
    iter, mem,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Index, Not},
    slice,
    str::FromStr,
    vec::Vec,
};

use derive_more::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, Display, From, Into,
};
#[cfg(test)]
use quickcheck::{Arbitrary, TestResult};
#[cfg(test)]
use quickcheck_macros::*;

use super::{
    constants::{
        MASK16_RANKS, MASK64_2, MASK64_ALL, N_CARDS, N_RANKS, N_SUITS,
        OFFSET_C, OFFSET_D, OFFSET_H, OFFSET_S, OFFSET_SUIT, RANK_NAMES,
        SUIT_NAMES, U16_LEADING_ONE,
    },
    *,
};

mod board;
mod card;
mod card64;
mod card_idx;
mod hand_iter;
mod hand_n;
mod isomorphic;
mod rank;
mod rank16;
mod suit;
mod suit4;

pub use board::*;
pub use card::*;
pub use card_idx::*;
pub use card64::*;
pub use hand_iter::*;
pub use hand_n::*;
pub use isomorphic::*;
pub use rank::*;
pub use rank16::*;
pub use suit::*;
pub use suit4::*;

pub type Hand = [Card];

/// Card count type
pub type CardCount = u8;

/// Index of the turn card in a board array
pub const IDX_TURN: usize = 3;
/// Index of the river card in a board array
pub const IDX_RIVER: usize = 4;
/// Number of cards in a flop
pub const N_FLOP: usize = 3;
/// Number of cards in a flop + turn
pub const N_TURN: usize = 4;
/// Number of cards in a flop + turn + river
pub const N_RIVER: usize = 5;

/// Mask for shortdeck ranks in Rank16
pub const MASK16_RANKS_SHORT: u16 = 0b0001_1111_1111_0000;

/// Mask for spades in Card64
pub const MASK64_S: u64 = MASK16_RANKS as u64;
/// Mask for hearts in Card64
pub const MASK64_H: u64 = MASK64_S << OFFSET_H;
/// Mask for diamonds in Card64
pub const MASK64_D: u64 = MASK64_S << OFFSET_D;
/// Mask for clubs in Card64
pub const MASK64_C: u64 = MASK64_S << OFFSET_C;

/// Mask for all suits in Card64
pub const MASK64_ALL_SHORT: u64 = (MASK16_RANKS_SHORT as u64)
    | ((MASK16_RANKS_SHORT as u64) << OFFSET_H)
    | ((MASK16_RANKS_SHORT as u64) << OFFSET_D)
    | ((MASK16_RANKS_SHORT as u64) << OFFSET_C);

/// Number of combinations of two ranks
pub const COMB_TWO_RANKS_LEN: usize = 91;

/// Normalizes a Card64 value
#[inline]
const fn normalize_u64(c: u64) -> [u8; 8] {
    unsafe {
        let [s, h, d, c]: [u16; 4] = mem::transmute(c);

        let has4 = s & h & d & c;
        let has3 = s & h & d | s & h & c | s & d & c | h & d & c;
        let has2 = s & h | s & d | s & c | h & d | h & c | d & c;
        let has1 = s | h | d | c;

        mem::transmute([has1, has2, has3, has4])
    }
}
