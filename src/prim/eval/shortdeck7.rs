use super::{
    flush_ranks, get_card_count, holdem7, mk_masks_rank, mk_masks_rank13,
    mk_masks_rankrank, mk_ranking, retain_leading_5_bits, retain_leading_bit,
    to_straightflush, FLUSH_SHORT, FULLHOUSE_SHORT, HIGHCARD, NONE_I16,
    STRAIGHT, STRAIGHT_789TJ, STRAIGHT_89TJQ, STRAIGHT_9TJQK, STRAIGHT_A789T,
    STRAIGHT_TJQKA, U16_T,
};

pub const fn eval(c: u64) -> i16 {
    let nf = eval_nf(c);
    let f = eval_f(c);

    if f > nf {
        f
    } else {
        nf
    }
}

#[inline]
pub const fn eval_nf(c: u64) -> i16 {
    let (has1, has2, has3, has4) = get_card_count(c);

    let res = holdem7::eval_quads(has4, has1);
    if res != NONE_I16 {
        return res;
    }

    let res = eval_fullhouse7s(has3, has2);
    if res != NONE_I16 {
        return res;
    }

    let res = eval_straight7s(has1);
    if res != NONE_I16 {
        return res;
    }

    let res = holdem7::eval_trips(has3, has1);
    if res != NONE_I16 {
        return res;
    }

    let res = holdem7::eval_twopair(has2, has1);
    if res != NONE_I16 {
        return res;
    }

    let res = holdem7::eval_pair(has2, has1);
    if res != NONE_I16 {
        return res;
    }

    mk_ranking(HIGHCARD, mk_masks_rank13(0, retain_leading_5_bits(has1)))
}

#[inline]
pub const fn eval_f(c: u64) -> i16 {
    let ranks = flush_ranks(c);

    if ranks == u16::MIN {
        NONE_I16
    } else {
        let res = eval_straight7s(ranks);

        if res == NONE_I16 {
            mk_ranking(
                FLUSH_SHORT,
                mk_masks_rank13(0, retain_leading_5_bits(ranks)),
            )
        } else {
            to_straightflush(res)
        }
    }
}

#[inline]
pub(super) const fn eval_fullhouse7s(has3: u16, has2: u16) -> i16 {
    if has3 > 0 {
        let h = retain_leading_bit(has3);
        let l = has2 & !h;

        if l > 0 {
            mk_ranking(FULLHOUSE_SHORT, mk_masks_rankrank(l, h))
        } else {
            NONE_I16
        }
    } else {
        NONE_I16
    }
}

#[inline]
pub(super) const fn eval_straight7s(ranks: u16) -> i16 {
    if STRAIGHT_TJQKA & ranks == STRAIGHT_TJQKA {
        mk_ranking(STRAIGHT, mk_masks_rank(0, STRAIGHT_TJQKA))
    } else if STRAIGHT_9TJQK & ranks == STRAIGHT_9TJQK {
        mk_ranking(STRAIGHT, mk_masks_rank(0, STRAIGHT_9TJQK))
    } else if STRAIGHT_89TJQ & ranks == STRAIGHT_89TJQ {
        mk_ranking(STRAIGHT, mk_masks_rank(0, STRAIGHT_89TJQ))
    } else if STRAIGHT_789TJ & ranks == STRAIGHT_789TJ {
        mk_ranking(STRAIGHT, mk_masks_rank(0, STRAIGHT_789TJ))
    } else if STRAIGHT_A789T & ranks == STRAIGHT_A789T {
        mk_ranking(STRAIGHT, mk_masks_rank(0, U16_T))
    } else {
        NONE_I16
    }
}

#[cfg(test)]
mod tests {
    use eval::shortdeck5;
    use itertools::Itertools;

    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_shortdeck7(cards: CardN<7, true>) -> TestResult {
        let cards: Vec<_> = cards.into();

        let res7 = eval(Card64::from(cards.as_ref()).to_u64());

        let mut max5 = HandRatingInt::MIN;

        for cs in cards.into_iter().combinations(5) {
            let res = shortdeck5::eval(Card64::from(cs.as_ref()).to_u64());

            if res > max5 {
                max5 = res;
            }
        }

        TestResult::from_bool(max5 == res7)
    }
}
