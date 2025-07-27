use super::{
    flush_ranks, get_card_count, mk_masks_comb2rank, mk_masks_rank,
    mk_masks_rank13, mk_masks_rankcomb2, mk_masks_rankcomb3, mk_masks_rankrank,
    mk_ranking, retain_leading_2_bits, retain_leading_5_bits,
    retain_leading_bit, to_straightflush, FLUSH, FULLHOUSE, HIGHCARD, NONE_I16,
    PAIR, QUADS, STRAIGHT, STRAIGHT_23456, STRAIGHT_34567, STRAIGHT_45678,
    STRAIGHT_56789, STRAIGHT_6789T, STRAIGHT_789TJ, STRAIGHT_89TJQ,
    STRAIGHT_9TJQK, STRAIGHT_A2345, STRAIGHT_TJQKA, TRIPS, TWOPAIR, U16_5,
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

    let res = eval_quads(has4, has1);
    if res != NONE_I16 {
        return res;
    }

    let res = eval_fullhouse(has3, has2);
    if res != NONE_I16 {
        return res;
    }

    let res = eval_straight(has1);
    if res != NONE_I16 {
        return res;
    }

    let res = eval_trips(has3, has1);
    if res != NONE_I16 {
        return res;
    }

    let res = eval_twopair(has2, has1);
    if res != NONE_I16 {
        return res;
    }

    let res = eval_pair(has2, has1);
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
        let res = eval_straight(ranks);

        if res == NONE_I16 {
            mk_ranking(FLUSH, mk_masks_rank13(0, retain_leading_5_bits(ranks)))
        } else {
            to_straightflush(res)
        }
    }
}

#[inline]
pub(super) const fn eval_pair(has2: u16, has1: u16) -> i16 {
    if has2 > 0 {
        let h = retain_leading_bit(has2);
        let l = has1 & !h;

        mk_ranking(PAIR, mk_masks_rankcomb3(l, h))
    } else {
        NONE_I16
    }
}

#[inline]
pub(super) const fn eval_twopair(has2: u16, has1: u16) -> i16 {
    if has2.count_ones() > 1 {
        let h = retain_leading_2_bits(has2);
        let l = has1 & !h;

        mk_ranking(TWOPAIR, mk_masks_comb2rank(l, h))
    } else {
        NONE_I16
    }
}
#[inline]
pub(super) const fn eval_trips(has3: u16, has1: u16) -> i16 {
    if has3 > 0 {
        let h = retain_leading_bit(has3);
        let l = has1 & !h;

        mk_ranking(TRIPS, mk_masks_rankcomb2(l, h))
    } else {
        NONE_I16
    }
}

#[inline]
pub(super) const fn eval_fullhouse(has3: u16, has2: u16) -> i16 {
    if has3 > 0 {
        let h = retain_leading_bit(has3);
        let l = has2 & !h;

        if l > 0 {
            mk_ranking(FULLHOUSE, mk_masks_rankrank(l, h))
        } else {
            NONE_I16
        }
    } else {
        NONE_I16
    }
}

#[inline]
pub(super) const fn eval_quads(has4: u16, has1: u16) -> i16 {
    if has4 > 0 {
        let h = has4;
        let l = has1 & !has4;

        mk_ranking(QUADS, mk_masks_rankrank(l, h))
    } else {
        NONE_I16
    }
}

#[inline]
pub(super) const fn eval_straight(ranks: u16) -> i16 {
    if STRAIGHT_TJQKA & ranks == STRAIGHT_TJQKA {
        mk_ranking(STRAIGHT, mk_masks_rank(0, STRAIGHT_TJQKA))
    } else if STRAIGHT_9TJQK & ranks == STRAIGHT_9TJQK {
        mk_ranking(STRAIGHT, mk_masks_rank(0, STRAIGHT_9TJQK))
    } else if STRAIGHT_89TJQ & ranks == STRAIGHT_89TJQ {
        mk_ranking(STRAIGHT, mk_masks_rank(0, STRAIGHT_89TJQ))
    } else if STRAIGHT_789TJ & ranks == STRAIGHT_789TJ {
        mk_ranking(STRAIGHT, mk_masks_rank(0, STRAIGHT_789TJ))
    } else if STRAIGHT_6789T & ranks == STRAIGHT_6789T {
        mk_ranking(STRAIGHT, mk_masks_rank(0, STRAIGHT_6789T))
    } else if STRAIGHT_56789 & ranks == STRAIGHT_56789 {
        mk_ranking(STRAIGHT, mk_masks_rank(0, STRAIGHT_56789))
    } else if STRAIGHT_45678 & ranks == STRAIGHT_45678 {
        mk_ranking(STRAIGHT, mk_masks_rank(0, STRAIGHT_45678))
    } else if STRAIGHT_34567 & ranks == STRAIGHT_34567 {
        mk_ranking(STRAIGHT, mk_masks_rank(0, STRAIGHT_34567))
    } else if STRAIGHT_23456 & ranks == STRAIGHT_23456 {
        mk_ranking(STRAIGHT, mk_masks_rank(0, STRAIGHT_23456))
    } else if STRAIGHT_A2345 & ranks == STRAIGHT_A2345 {
        mk_ranking(STRAIGHT, mk_masks_rank(0, U16_5))
    } else {
        NONE_I16
    }
}

#[cfg(test)]
mod tests {
    use eval::holdem5;
    use itertools::Itertools;

    use super::*;
    use crate::{HandType::*, *};

    fn assert_ranking(cs: &str, ht: HandType, hi: &str, lo: &str) {
        let g = PQLGame::Holdem;

        let rating = PQLHiRating::new(eval(c64!(cs).to_u64()));
        let l = HandRatingView::from((g, rating));
        let r = HandRatingView::from((g, ht, r16!(hi), r16!(lo)));

        assert_eq!(l, r);
    }

    #[test]
    fn test_straight_flush() {
        assert_ranking("Ah Ad As Ks Qs Js Ts", StraightFlush, "A", "");
        assert_ranking("As Ad Kh Qh Jh Th 9h", StraightFlush, "K", "");
        assert_ranking("As Ah Qd Jd Td 9d 8d", StraightFlush, "Q", "");
        assert_ranking("As Ah Jc Tc 9c 8c 7c", StraightFlush, "J", "");
        assert_ranking("As Ah Td 9d 8d 7d 6d", StraightFlush, "T", "");
        assert_ranking("As Ah Ad 5d 4d 3d 2d", StraightFlush, "5", "");
    }

    #[test]
    fn test_quad() {
        assert_ranking("Kh Kd As Ah Ac Ad Ks", Quads, "A", "K");
        assert_ranking("7h 8d 6s 6h 6c 6d Ts", Quads, "6", "T");
    }

    #[test]
    fn test_full_house() {
        assert_ranking("As Ah Ts Th Td 9s 9h", FullHouse, "T", "A");
        assert_ranking("9d As Ts Th Td 9s 9h", FullHouse, "T", "9");
    }

    #[test]
    fn test_flush() {
        assert_ranking("Ah Ad 6s 7s 8s 9s Js", Flush, "J6789", "");
        assert_ranking("As Ac 6h 7h 8h 9h Jh", Flush, "J6789", "");
        assert_ranking("As Ac 6d 7d 8d 9d Jd", Flush, "J6789", "");
        assert_ranking("As Ah 6c 7c 8c 9c Jc", Flush, "J6789", "");
        assert_ranking("Ks Kh 6s 7s 8s 9s Js", Flush, "J789K", "");
        assert_ranking("Ks Qs 6s 7s 8s 9s Js", Flush, "J89KQ", "");
    }

    #[test]
    fn test_straight() {
        assert_ranking("9s 8h As Kh Qd Jc Ts", Straight, "A", "");
        assert_ranking("8s 7h Ks Qh Jd Tc 9s", Straight, "K", "");
        assert_ranking("7s 6h Qs Jh Td 9c 8s", Straight, "Q", "");
        assert_ranking("6s Ah Js Th 9d 8c 7s", Straight, "J", "");
        assert_ranking("As Ah Ts 9h 8d 7c 6s", Straight, "T", "");
        assert_ranking("As Ah 9s 8h 7d 6c 5s", Straight, "9", "");
        assert_ranking("As Ah 8s 7h 6d 5c 4s", Straight, "8", "");
        assert_ranking("As Ah 7s 6h 5d 4c 3s", Straight, "7", "");
        assert_ranking("As Ah 6d 2h 3d 4c 5s", Straight, "6", "");
        assert_ranking("As Ah Ad 2h 3d 4c 5s", Straight, "5", "");
    }

    #[test]
    fn test_trips() {
        assert_ranking("Qd 6c Ts Th Td As Kh", Trips, "T", "AK");
    }

    #[test]
    fn test_two_pairs() {
        assert_ranking("Ts Th 6s 6h Ks Qh Jd", TwoPair, "T6", "K");
        assert_ranking("Ts Th 6s 6h 7s 7h Jd", TwoPair, "T7", "J");
    }

    #[test]
    fn test_pair() {
        assert_ranking("Js Jh 6s 7h 8d 9c Ks", Pair, "J", "K98");
        assert_ranking("As Ah 6s 7h 8d Tc Ks", Pair, "A", "KT8");
    }

    #[test]
    fn test_high_card() {
        assert_ranking("6s 7h 8d 9c Js Qh Kd", HighCard, "KQJ98", "");
    }

    #[quickcheck]
    fn test_holdem7(cards: CardN<7>) -> TestResult {
        let res7 = eval(Card64::from(cards.clone()).to_u64());

        let mut max5 = HandRatingInt::MIN;

        for cs in cards.into_iter().combinations(5) {
            let res = holdem5::eval(Card64::from(cs.as_ref()).to_u64());

            if res > max5 {
                max5 = res;
            }
        }

        TestResult::from_bool(max5 == res7)
    }
}
