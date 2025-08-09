use super::{
    FLUSH, FULLHOUSE, HIGHCARD, PAIR, QUADS, STRAIGHT, STRAIGHT_9TJQK,
    STRAIGHT_89TJQ, STRAIGHT_789TJ, STRAIGHT_6789T, STRAIGHT_23456,
    STRAIGHT_34567, STRAIGHT_45678, STRAIGHT_56789, STRAIGHT_A2345,
    STRAIGHT_TJQKA, TRIPS, TWOPAIR, U16_5, common::NONE_I16, get_card_count,
    mk_masks_comb2rank, mk_masks_rank, mk_masks_rank13, mk_masks_rankcomb2,
    mk_masks_rankcomb3, mk_masks_rankrank, mk_ranking, retain_leading_2_bits,
    retain_leading_3_bits, retain_leading_bit, to_straightflush,
};

pub const fn eval(player: u64, board: u64) -> i16 {
    let nf = eval_nf(player, board);
    let f = eval_f(player, board);

    if f > nf { f } else { nf }
}

pub const fn eval_nf(player: u64, board: u64) -> i16 {
    let (p1, p2, _, _) = get_card_count(player);
    let (b1, b2, b3, _) = get_card_count(board);

    let res = eval_quads(p2, p1, b3, b2, b1);
    if res != NONE_I16 {
        return res;
    }

    let res = eval_fullhouse(p2, p1, b3, b2, b1);
    if res != NONE_I16 {
        return res;
    }

    let res = eval_straight(p1, b1);
    if res != NONE_I16 {
        return res;
    }

    let res = eval_trips(p2, p1, b3, b2, b1);
    if res != NONE_I16 {
        return res;
    }

    let res = eval_twopair(p2, p1, b2, b1);
    if res != NONE_I16 {
        return res;
    }

    let res = eval_pair(p2, p1, b2, b1);
    if res != NONE_I16 {
        return res;
    }

    mk_ranking(
        HIGHCARD,
        mk_masks_rank13(
            0,
            retain_leading_3_bits(b1) | retain_leading_2_bits(p1),
        ),
    )
}

#[inline]
pub const fn eval_f(player: u64, board: u64) -> i16 {
    let (p, b) = flush_ranks_omaha(player, board);

    if p > 0 {
        let res = eval_straight(p, b);

        if res == NONE_I16 {
            mk_ranking(
                FLUSH,
                mk_masks_rank13(
                    0,
                    retain_leading_2_bits(p) | retain_leading_3_bits(b),
                ),
            )
        } else {
            to_straightflush(res)
        }
    } else {
        NONE_I16
    }
}

#[inline]
const fn flush_ranks_omaha(player: u64, board: u64) -> (u16, u16) {
    #[inline]
    const fn get_u16(bytes: [u8; 8], i: usize) -> u16 {
        u16::from_le_bytes([bytes[i], bytes[i + 1]])
    }
    let arr = board.to_le_bytes();

    let suit_idx = if arr[0].count_ones() + arr[1].count_ones() >= 3 {
        Some(0)
    } else if arr[2].count_ones() + arr[3].count_ones() >= 3 {
        Some(2)
    } else if arr[4].count_ones() + arr[5].count_ones() >= 3 {
        Some(4)
    } else if arr[6].count_ones() + arr[7].count_ones() >= 3 {
        Some(6)
    } else {
        None
    };

    if let Some(i) = suit_idx {
        let arr_player = player.to_le_bytes();

        let p = get_u16(arr_player, i);

        if p.count_ones() >= 2 {
            let b = get_u16(arr, i);
            return (p, b);
        }
    }

    (0, 0)
}

const fn max(l: (u16, u16), r: (u16, u16)) -> (u16, u16) {
    if l.0 > r.0 || (l.0 == r.0 && l.1 > r.1) {
        l
    } else {
        r
    }
}

// PAIR Case I: aax; yz
#[inline]
const fn eval_pair_1(p1: u16, b2: u16, b1: u16) -> (u16, u16) {
    let h = retain_leading_bit(b2);

    if h > 0 {
        (h, retain_leading_bit(b1 & !h) | retain_leading_2_bits(p1))
    } else {
        (0, 0)
    }
}

// PAIR Case II: axy; az
#[inline]
const fn eval_pair_2(p1: u16, b1: u16) -> (u16, u16) {
    let h = retain_leading_bit(p1 & b1);

    if h > 0 {
        (
            h,
            retain_leading_bit(p1 & !h) | retain_leading_2_bits(b1 & !h),
        )
    } else {
        (0, 0)
    }
}

// PAIR Case III: xyz; aa
#[inline]
const fn eval_pair_3(p2: u16, b1: u16) -> (u16, u16) {
    let h = retain_leading_bit(p2);

    if h > 0 {
        (h, retain_leading_3_bits(b1))
    } else {
        (0, 0)
    }
}

#[inline]
const fn eval_pair(p2: u16, p1: u16, b2: u16, b1: u16) -> i16 {
    let (h, l) = max(
        max(eval_pair_1(p1, b2, b1), eval_pair_2(p1, b1)),
        eval_pair_3(p2, b1),
    );

    if h > 0 {
        mk_ranking(PAIR, mk_masks_rankcomb3(l, h))
    } else {
        NONE_I16
    }
}

// TWOPAIR Case I: aak; bb
#[inline]
const fn eval_twopair_1(p2: u16, b2: u16, b1: u16) -> (u16, u16) {
    let h1 = retain_leading_bit(b2);
    let h2 = retain_leading_bit(p2);

    if h1 > 0 && h2 > 0 {
        (h1 | h2, retain_leading_bit(b1 & !h1))
    } else {
        (0, 0)
    }
}

// TWOPAIR Case II: aab; bk
#[inline]
const fn eval_twopair_2(p1: u16, b2: u16, b1: u16) -> (u16, u16) {
    let h1 = retain_leading_bit(b2);
    let h2 = retain_leading_bit(p1 & b1);

    if h1 > 0 && h2 > 0 {
        (h1 | h2, retain_leading_bit(p1 & !h2))
    } else {
        (0, 0)
    }
}

// TWOPAIR Case II: abk; ab
#[inline]
const fn eval_twopair_3(p1: u16, b1: u16) -> (u16, u16) {
    let h = retain_leading_2_bits(p1 & b1);

    if h.count_ones() == 2 {
        (h, retain_leading_bit(b1 & !h))
    } else {
        (0, 0)
    }
}

#[inline]
const fn eval_twopair(p2: u16, p1: u16, b2: u16, b1: u16) -> i16 {
    let (h, l) = max(
        max(eval_twopair_1(p2, b2, b1), eval_twopair_2(p1, b2, b1)),
        eval_twopair_3(p1, b1),
    );

    if h > 0 {
        mk_ranking(TWOPAIR, mk_masks_comb2rank(l, h))
    } else {
        NONE_I16
    }
}

// TRIPS Case I: aaa; xy
#[inline]
const fn eval_trips_1(p1: u16, b3: u16) -> (u16, u16) {
    let h = retain_leading_bit(b3);

    if h > 0 {
        (h, retain_leading_2_bits(p1))
    } else {
        (0, 0)
    }
}

// TRIPS Case II: aax; ay
#[inline]
const fn eval_trips_2(p1: u16, b2: u16, b1: u16) -> (u16, u16) {
    let h = retain_leading_bit(b2 & p1);

    if h > 0 {
        (h, retain_leading_bit(b1 & !h) | retain_leading_bit(p1 & !h))
    } else {
        (0, 0)
    }
}

// TRIPS Case III: axy; aa
#[inline]
const fn eval_trips_3(p2: u16, b1: u16) -> (u16, u16) {
    let h = retain_leading_bit(b1 & p2);

    if h > 0 {
        (h, retain_leading_2_bits(b1 & !h))
    } else {
        (0, 0)
    }
}

#[inline]
const fn eval_trips(p2: u16, p1: u16, b3: u16, b2: u16, b1: u16) -> i16 {
    let (h, l) = max(
        max(eval_trips_1(p1, b3), eval_trips_2(p1, b2, b1)),
        eval_trips_3(p2, b1),
    );

    if h > 0 {
        mk_ranking(TRIPS, mk_masks_rankcomb2(l, h))
    } else {
        NONE_I16
    }
}

#[inline]
const fn eval_straight_x(mask: u16, p1: u16, b1: u16) -> u16 {
    if mask & (p1 | b1) == mask
        && (mask & p1).count_ones() >= 2
        && (mask & b1).count_ones() >= 3
    {
        retain_leading_bit(mask)
    } else {
        u16::MIN
    }
}

#[inline]
const fn eval_straight(p1: u16, b1: u16) -> i16 {
    let r = eval_straight_x(STRAIGHT_TJQKA, p1, b1);
    if r > u16::MIN {
        return mk_ranking(STRAIGHT, mk_masks_rank(0, r));
    }

    let r = eval_straight_x(STRAIGHT_9TJQK, p1, b1);
    if r > u16::MIN {
        return mk_ranking(STRAIGHT, mk_masks_rank(0, r));
    }

    let r = eval_straight_x(STRAIGHT_89TJQ, p1, b1);
    if r > u16::MIN {
        return mk_ranking(STRAIGHT, mk_masks_rank(0, r));
    }

    let r = eval_straight_x(STRAIGHT_789TJ, p1, b1);
    if r > u16::MIN {
        return mk_ranking(STRAIGHT, mk_masks_rank(0, r));
    }

    let r = eval_straight_x(STRAIGHT_6789T, p1, b1);
    if r > u16::MIN {
        return mk_ranking(STRAIGHT, mk_masks_rank(0, r));
    }

    let r = eval_straight_x(STRAIGHT_56789, p1, b1);
    if r > u16::MIN {
        return mk_ranking(STRAIGHT, mk_masks_rank(0, r));
    }

    let r = eval_straight_x(STRAIGHT_45678, p1, b1);
    if r > u16::MIN {
        return mk_ranking(STRAIGHT, mk_masks_rank(0, r));
    }

    let r = eval_straight_x(STRAIGHT_34567, p1, b1);
    if r > u16::MIN {
        return mk_ranking(STRAIGHT, mk_masks_rank(0, r));
    }

    let r = eval_straight_x(STRAIGHT_23456, p1, b1);
    if r > u16::MIN {
        return mk_ranking(STRAIGHT, mk_masks_rank(0, r));
    }

    let r = eval_straight_x(STRAIGHT_A2345, p1, b1);
    if r > u16::MIN {
        return mk_ranking(STRAIGHT, mk_masks_rank(0, U16_5));
    }

    NONE_I16
}

// FULLHOUSE Case I: aaa; bb
#[inline]
const fn eval_fullhouse_1(p2: u16, b3: u16) -> (u16, u16) {
    let h = retain_leading_bit(b3);

    if h > 0 {
        let l = retain_leading_bit(p2);

        if l > 0 {
            return (h, l);
        }
    }
    (0, 0)
}

// FULLHOUSE Case II: abb; aa
#[inline]
const fn eval_fullhouse_2(p2: u16, b2: u16, b1: u16) -> (u16, u16) {
    let h = retain_leading_bit(p2 & b1);

    if h > 0 {
        let l = retain_leading_bit(b2);

        if l > 0 {
            return (h, l);
        }
    }
    (0, 0)
}

// FULLHOUSE Case III: abb; ab
#[inline]
const fn eval_fullhouse_3(p1: u16, b2: u16, b1: u16) -> (u16, u16) {
    let h = retain_leading_bit(b2 & p1);

    if h > 0 {
        let l = retain_leading_bit(b1 & p1 & !h);

        if l > 0 {
            return (h, l);
        }
    }
    (0, 0)
}

#[inline]
const fn eval_fullhouse(p2: u16, p1: u16, b3: u16, b2: u16, b1: u16) -> i16 {
    let (h, l) = max(
        max(eval_fullhouse_1(p2, b3), eval_fullhouse_2(p2, b2, b1)),
        eval_fullhouse_3(p1, b2, b1),
    );

    if h > 0 {
        mk_ranking(FULLHOUSE, mk_masks_rankrank(l, h))
    } else {
        NONE_I16
    }
}

// QUADS Case I: aak; aa
#[inline]
const fn eval_quads_1(p2: u16, b2: u16, b1: u16) -> (u16, u16) {
    let h = retain_leading_bit(p2 & b2);
    if h > 0 {
        (h, retain_leading_bit(b1 & !h))
    } else {
        (0, 0)
    }
}

// QUADS Case II: aaa; ak
#[inline]
const fn eval_quads_2(p1: u16, b3: u16) -> (u16, u16) {
    let h = retain_leading_bit(p1 & b3);
    if h > 0 {
        (h, retain_leading_bit(p1 & !h))
    } else {
        (0, 0)
    }
}

#[inline]
const fn eval_quads(p2: u16, p1: u16, b3: u16, b2: u16, b1: u16) -> i16 {
    let (h, l) = max(eval_quads_1(p2, b2, b1), eval_quads_2(p1, b3));

    if h > 0 {
        mk_ranking(QUADS, mk_masks_rankrank(l, h))
    } else {
        NONE_I16
    }
}

#[cfg(test)]
mod tests {
    use eval::holdem5;
    use itertools::Itertools;

    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_omaha9(cards: CardN<9>) -> TestResult {
        let (hand, board): (CardN<4>, CardN<5>) = cards.into();

        let res9 = eval(
            Card64::from(hand.as_ref()).to_u64(),
            Card64::from(board.as_ref()).to_u64(),
        );

        let mut max5 = HandRatingInt::MIN;

        for h in hand.into_iter().combinations(2) {
            for b in board.clone().into_iter().combinations(3) {
                let mut cs = Card64::empty();

                cs.set(h[0]);
                cs.set(h[1]);
                cs.set(b[0]);
                cs.set(b[1]);
                cs.set(b[2]);

                let res = holdem5::eval(cs.to_u64());

                if res > max5 {
                    max5 = res;
                }
            }
        }

        TestResult::from_bool(max5 == res9)
    }
}
