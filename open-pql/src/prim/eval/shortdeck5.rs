use super::{
    FLUSH_SHORT, FULLHOUSE_SHORT, HIGHCARD, NONE_I16, STRAIGHT, STRAIGHT_9TJQK,
    STRAIGHT_89TJQ, STRAIGHT_789TJ, STRAIGHT_A789T, STRAIGHT_TJQKA, U16_T,
    flush_ranks, get_card_count, holdem5, mk_masks_rank, mk_masks_rank13,
    mk_masks_rankrank, mk_ranking, to_straightflush,
};

pub const fn eval(c: u64) -> i16 {
    let nf = eval_nf(c);
    let f = eval_f(c);

    if f > nf { f } else { nf }
}

#[inline]
pub const fn eval_nf(c: u64) -> i16 {
    let (has1, has2, has3, has4) = get_card_count(c);

    let res = holdem5::eval_quads(has4, has1);
    if res != NONE_I16 {
        return res;
    }

    let res = eval_fullhouse5s(has3, has2);
    if res != NONE_I16 {
        return res;
    }

    let res = eval_straight5s(has1);
    if res != NONE_I16 {
        return res;
    }

    let res = holdem5::eval_trips(has3, has1);
    if res != NONE_I16 {
        return res;
    }

    let res = holdem5::eval_twopair(has2, has1);
    if res != NONE_I16 {
        return res;
    }

    let res = holdem5::eval_pair(has2, has1);
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
        let res = eval_straight5s(ranks);

        if res == NONE_I16 {
            mk_ranking(FLUSH_SHORT, mk_masks_rank13(0, ranks))
        } else {
            to_straightflush(res)
        }
    }
}

#[inline]
pub(super) const fn eval_fullhouse5s(has3: u16, has2: u16) -> i16 {
    if has3 != 0 {
        let l = has2 & !has3;

        if l != 0 {
            mk_ranking(FULLHOUSE_SHORT, mk_masks_rankrank(l, has3))
        } else {
            NONE_I16
        }
    } else {
        NONE_I16
    }
}

#[inline]
pub(super) const fn eval_straight5s(ranks: u16) -> i16 {
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
    use super::*;
    use crate::{HandType::*, *};

    fn assert_ranking(cs: &str, ht: HandType, hi: &str, lo: &str) {
        let g = PQLGame::ShortDeck;

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
        assert_ranking("Td 9d 8d 7d Ad", StraightFlush, "T", "");
    }

    #[test]
    fn test_quad() {
        assert_ranking("Kh As Ah Ac Ad", Quads, "A", "K");
        assert_ranking("7s 7h 7c 7d Ts", Quads, "7", "T");
    }

    #[test]
    fn test_full_house() {
        assert_ranking("As Ah Ts Th Td", FullHouse, "T", "A");
        assert_ranking("Ts Th Td 9s 9h", FullHouse, "T", "9");
    }

    #[test]
    fn test_flush() {
        assert_ranking("As 7s 8s 9s Js", Flush, "AJ789", "");
        assert_ranking("Ah 7h 8h 9h Jh", Flush, "AJ789", "");
        assert_ranking("Ad 7d 8d 9d Jd", Flush, "AJ789", "");
        assert_ranking("Ac 7c 8c 9c Jc", Flush, "AJ789", "");
        assert_ranking("As 7s 8s 9s Js", Flush, "A789J", "");
    }

    #[test]
    fn test_straight() {
        assert_ranking("As Kh Qd Jc Ts", Straight, "A", "");
        assert_ranking("Ks Qh Jd Tc 9s", Straight, "K", "");
        assert_ranking("Qs Jh Td 9c 8s", Straight, "Q", "");
        assert_ranking("Js Th 9d 8c 7s", Straight, "J", "");
        assert_ranking("Ts 9h 8d 7c As", Straight, "T", "");
    }

    #[test]
    fn test_trips() {
        assert_ranking("Ts Th Td As Kh", Trips, "T", "AK");
    }

    #[test]
    fn test_two_pairs() {
        assert_ranking("Ts Th 7s 7h Ks", TwoPair, "T7", "K");
        assert_ranking("Ts Th 7s 7h Jd", TwoPair, "T7", "J");
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
