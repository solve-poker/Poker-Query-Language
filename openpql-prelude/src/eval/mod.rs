pub mod flop;
pub mod index;
pub mod rating;

mod payoff;

use std::mem::transmute;

pub use payoff::calculate_payoffs;

use crate::{Card64, Rank16};

#[inline]
pub const fn count_ranks(c: Card64) -> [Rank16; 4] {
    // transmute is faster than calling to_le_bytes 4 times.
    unsafe {
        let [s, h, d, c]: [u16; 4] = transmute(c);

        let has4 = s & h & d & c;
        let has3 = s & h & d | s & h & c | s & d & c | h & d & c;

        let has2 = s & h | s & d | s & c | h & d | h & c | d & c;
        let has1 = s | h | d | c;

        transmute([has1, has2, has3, has4])
    }
}
