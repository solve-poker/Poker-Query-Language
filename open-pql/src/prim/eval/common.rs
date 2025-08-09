use std::mem::transmute;

use crate::prim;

pub const NONE_I16: i16 = i16::MIN;

pub const HIGHCARD: u8 = 0b100 << 5;
pub const PAIR: u8 = 0b101 << 5;
pub const TWOPAIR: u8 = 0b110 << 5;
pub const TRIPS: u8 = 0b111 << 5;
pub const STRAIGHT: u8 = 0b000 << 5;
pub const FLUSH: u8 = 0b001 << 5;
pub const FULLHOUSE: u8 = 0b010 << 5;
pub const QUADS: u8 = 0b011 << 5;
pub const STRAIGHTFLUSH: u8 = 0b011 << 5;
pub const FLUSH_SHORT: u8 = 0b010 << 5;
pub const FULLHOUSE_SHORT: u8 = 0b001 << 5;

pub const STRAIGHT_A789T: u16 = 0b1_0001_1110_0000;
pub const STRAIGHT_A2345: u16 = 0b1_0000_0000_1111;
pub const STRAIGHT_23456: u16 = 0b11111;
pub const STRAIGHT_34567: u16 = 0b11111 << 1;
pub const STRAIGHT_45678: u16 = 0b11111 << 2;
pub const STRAIGHT_56789: u16 = 0b11111 << 3;
pub const STRAIGHT_6789T: u16 = 0b11111 << 4;
pub const STRAIGHT_789TJ: u16 = 0b11111 << 5;
pub const STRAIGHT_89TJQ: u16 = 0b11111 << 6;
pub const STRAIGHT_9TJQK: u16 = 0b11111 << 7;
pub const STRAIGHT_TJQKA: u16 = 0b11111 << 8;

pub const ARR_STRAIGHT: [u16; 10] = [
    STRAIGHT_TJQKA,
    STRAIGHT_9TJQK,
    STRAIGHT_89TJQ,
    STRAIGHT_789TJ,
    STRAIGHT_6789T,
    STRAIGHT_56789,
    STRAIGHT_45678,
    STRAIGHT_34567,
    STRAIGHT_23456,
    STRAIGHT_A2345,
];

pub const ARR_STRAIGHT_SHORT: [u16; 5] = [
    STRAIGHT_TJQKA,
    STRAIGHT_9TJQK,
    STRAIGHT_89TJQ,
    STRAIGHT_789TJ,
    STRAIGHT_A789T,
];

pub const U16_T: u16 = 0b0_0001_0000_0000;
pub const U16_5: u16 = 0b0_0000_0000_1000;

#[inline]
pub const fn get_card_count(c: u64) -> (u16, u16, u16, u16) {
    unsafe { transmute(prim::normalize_u64(c)) }
}

#[inline]
pub const fn mk_ranking(kind: u8, (lo, hi): (u8, u8)) -> i16 {
    i16::from_le_bytes([lo, hi | kind])
}

#[inline]
pub const fn to_straightflush(ranking: i16) -> i16 {
    i16::from_le_bytes([0, STRAIGHTFLUSH]) | (0b0001_1111_1111_1111 & ranking)
}

pub const fn flush_ranks(c: u64) -> u16 {
    let arr = c.to_le_bytes();

    if arr[0].count_ones() + arr[1].count_ones() >= 5 {
        u16::from_le_bytes([arr[0], arr[1]])
    } else if arr[2].count_ones() + arr[3].count_ones() >= 5 {
        u16::from_le_bytes([arr[2], arr[3]])
    } else if arr[4].count_ones() + arr[5].count_ones() >= 5 {
        u16::from_le_bytes([arr[4], arr[5]])
    } else if arr[6].count_ones() + arr[7].count_ones() >= 5 {
        u16::from_le_bytes([arr[6], arr[7]])
    } else {
        u16::MIN
    }
}
