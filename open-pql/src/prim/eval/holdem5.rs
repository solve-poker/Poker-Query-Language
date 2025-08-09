use super::{
    FLUSH, FULLHOUSE, HIGHCARD, NONE_I16, PAIR, QUADS, STRAIGHT,
    STRAIGHT_A2345, TRIPS, TWOPAIR, U16_5, flush_ranks, get_card_count,
    mk_masks_comb2rank, mk_masks_rank, mk_masks_rank13, mk_masks_rankcomb2,
    mk_masks_rankcomb3, mk_masks_rankrank, mk_ranking, to_straightflush,
};

pub const fn eval(c: u64) -> i16 {
    let nf = eval_nf(c);
    let f = eval_f(c);

    if f > nf { f } else { nf }
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

    mk_ranking(HIGHCARD, mk_masks_rank13(0, has1))
}

#[inline]
pub const fn eval_f(c: u64) -> i16 {
    let ranks = flush_ranks(c);

    if ranks == u16::MIN {
        NONE_I16
    } else {
        let res = eval_straight(ranks);

        if res == NONE_I16 {
            mk_ranking(FLUSH, mk_masks_rank13(0, ranks))
        } else {
            to_straightflush(res)
        }
    }
}

#[inline]
pub(super) const fn eval_pair(has2: u16, has1: u16) -> i16 {
    if has2 != 0 {
        mk_ranking(PAIR, mk_masks_rankcomb3(has1 & !has2, has2))
    } else {
        NONE_I16
    }
}

#[inline]
pub(super) const fn eval_twopair(has2: u16, has1: u16) -> i16 {
    if has2.count_ones() > 1 {
        mk_ranking(TWOPAIR, mk_masks_comb2rank(has1 & !has2, has2))
    } else {
        NONE_I16
    }
}

#[inline]
pub(super) const fn eval_trips(has3: u16, has1: u16) -> i16 {
    if has3 != 0 {
        mk_ranking(TRIPS, mk_masks_rankcomb2(has1 & !has3, has3))
    } else {
        NONE_I16
    }
}

#[inline]
pub(super) const fn eval_fullhouse(has3: u16, has2: u16) -> i16 {
    if has3 != 0 {
        let l = has2 & !has3;

        if l != 0 {
            mk_ranking(FULLHOUSE, mk_masks_rankrank(l, has3))
        } else {
            NONE_I16
        }
    } else {
        NONE_I16
    }
}

#[inline]
pub(super) const fn eval_quads(has4: u16, has1: u16) -> i16 {
    if has4 != 0 {
        mk_ranking(QUADS, mk_masks_rankrank(has1 & !has4, has4))
    } else {
        NONE_I16
    }
}

#[inline]
pub(super) const fn eval_straight(ranks: u16) -> i16 {
    if ranks.count_ones() == 5 {
        let tail = ranks.trailing_zeros();
        let head = ranks.leading_zeros();

        if head + tail == 11 {
            return mk_ranking(STRAIGHT, mk_masks_rank(0, ranks));
        } else if STRAIGHT_A2345 == ranks {
            return mk_ranking(STRAIGHT, mk_masks_rank(0, U16_5));
        }
    }
    NONE_I16
}

#[cfg(test)]
mod tests {
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
        assert_ranking("As Ks Qs Js Ts", StraightFlush, "A", "");
        assert_ranking("Kh Qh Jh Th 9h", StraightFlush, "K", "");
        assert_ranking("Qd Jd Td 9d 8d", StraightFlush, "Q", "");
        assert_ranking("Jc Tc 9c 8c 7c", StraightFlush, "J", "");
        assert_ranking("Td 9d 8d 7d 6d", StraightFlush, "T", "");
        assert_ranking("Ad 5d 4d 3d 2d", StraightFlush, "5", "");
    }

    #[test]
    fn test_quad() {
        assert_ranking("Kh As Ah Ac Ad", Quads, "A", "K");
        assert_ranking("6s 6h 6c 6d Ts", Quads, "6", "T");
    }

    #[test]
    fn test_full_house() {
        assert_ranking("As Ah Ts Th Td", FullHouse, "T", "A");
        assert_ranking("Ts Th Td 9s 9h", FullHouse, "T", "9");
    }

    #[test]
    fn test_flush() {
        assert_ranking("6s 7s 8s 9s Js", Flush, "J6789", "");
        assert_ranking("6h 7h 8h 9h Jh", Flush, "J6789", "");
        assert_ranking("6d 7d 8d 9d Jd", Flush, "J6789", "");
        assert_ranking("6c 7c 8c 9c Jc", Flush, "J6789", "");
        assert_ranking("6s 7s 8s 9s Js", Flush, "6789J", "");
    }

    #[test]
    fn test_straight() {
        assert_ranking("As Kh Qd Jc Ts", Straight, "A", "");
        assert_ranking("Ks Qh Jd Tc 9s", Straight, "K", "");
        assert_ranking("Qs Jh Td 9c 8s", Straight, "Q", "");
        assert_ranking("Js Th 9d 8c 7s", Straight, "J", "");
        assert_ranking("Ts 9h 8d 7c 6s", Straight, "T", "");
        assert_ranking("9s 8h 7d 6c 5s", Straight, "9", "");
        assert_ranking("8s 7h 6d 5c 4s", Straight, "8", "");
        assert_ranking("7s 6h 5d 4c 3s", Straight, "7", "");
        assert_ranking("6d 2h 3d 4c 5s", Straight, "6", "");
        assert_ranking("Ad 2h 3d 4c 5s", Straight, "5", "");
    }

    #[test]
    fn test_trips() {
        assert_ranking("Ts Th Td As Kh", Trips, "T", "AK");
    }

    #[test]
    fn test_two_pairs() {
        assert_ranking("Ts Th 6s 6h Ks", TwoPair, "T6", "K");
        assert_ranking("Ts Th 6s 6h Jd", TwoPair, "T6", "J");
    }

    #[test]
    fn test_pair() {
        assert_ranking("Js Jh 8d 9c Ks", Pair, "J", "K98");
        assert_ranking("As Ah 8d Tc Ks", Pair, "A", "KT8");
    }

    #[test]
    fn test_high_card() {
        assert_ranking("8d 9c Js Qh Kd", HighCard, "KQJ98", "");
    }
}
