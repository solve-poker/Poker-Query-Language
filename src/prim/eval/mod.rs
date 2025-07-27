use super::{
    integer::{
        retain_leading_2_bits, retain_leading_3_bits, retain_leading_5_bits,
        retain_leading_bit,
    },
    rank::{
        mk_masks_comb2rank, mk_masks_rank, mk_masks_rank13, mk_masks_rankcomb2,
        mk_masks_rankcomb3, mk_masks_rankrank,
    },
};

mod common;
pub mod holdem5;
pub mod holdem7;
pub mod omaha9;
pub mod shortdeck5;
pub mod shortdeck7;

pub(crate) use common::{
    flush_ranks, mk_ranking, to_straightflush, FLUSH, FLUSH_SHORT, FULLHOUSE,
    FULLHOUSE_SHORT, HIGHCARD, NONE_I16, PAIR, QUADS, STRAIGHT, STRAIGHT_23456,
    STRAIGHT_34567, STRAIGHT_45678, STRAIGHT_56789, STRAIGHT_6789T,
    STRAIGHT_789TJ, STRAIGHT_89TJQ, STRAIGHT_9TJQK, STRAIGHT_TJQKA, TRIPS,
    TWOPAIR, U16_5, U16_T,
};
pub use common::{
    get_card_count, ARR_STRAIGHT, ARR_STRAIGHT_SHORT, STRAIGHT_A2345,
    STRAIGHT_A789T,
};

pub const fn decompose_omaha(cards: u64) -> (u64, u64, u64, u64, u64, u64) {
    let z1 = cards.trailing_zeros();
    let z2 = (cards ^ 1 << z1).trailing_zeros();
    let z3 = (cards ^ 1 << z1 ^ 1 << z2).trailing_zeros();
    let z4 = (cards ^ 1 << z1 ^ 1 << z2 ^ 1 << z3).trailing_zeros();

    (
        1 << z1 | 1 << z2,
        1 << z1 | 1 << z3,
        1 << z1 | 1 << z4,
        1 << z2 | 1 << z3,
        1 << z2 | 1 << z4,
        1 << z3 | 1 << z4,
    )
}
