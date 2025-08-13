pub const N_CARDS: u8 = 52;
pub const N_RANKS: u8 = 13;
pub const N_SUITS: u8 = 4;
pub const RANK_NAMES: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];
pub const SUIT_NAMES: [char; 4] = ['s', 'h', 'd', 'c'];

pub const OFFSET_SUIT: i8 = 16;
pub const OFFSET_S: i8 = 0;
pub const OFFSET_H: i8 = 16;
pub const OFFSET_D: i8 = 16 * 2;
pub const OFFSET_C: i8 = 16 * 3;

pub const MASK16_RANKS: u16 = 0b0001_1111_1111_1111;
pub const U16_LEADING_ONE: u16 = 0b1000_0000_0000_0000;

pub const MASK64_2: u64 =
    1 << OFFSET_S | 1 << OFFSET_H | 1 << OFFSET_D | 1 << OFFSET_C;

pub const MASK64_ALL: u64 = MASK64_S | MASK64_H | MASK64_D | MASK64_C;

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
