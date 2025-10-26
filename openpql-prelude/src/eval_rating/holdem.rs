use super::{Card64, HandRating, Rank16, count_ranks, flush_ranks};

#[inline]
pub(super) const fn eval_quads(
    has4: Rank16,
    has1: Rank16,
) -> Option<HandRating> {
    if has4.is_empty() {
        None
    } else {
        Some(HandRating::new_quad(has4, has1.diff(has4)))
    }
}

#[inline]
pub(super) const fn eval_fullhouse(
    has3: Rank16,
    has2: Rank16,
) -> Option<HandRating> {
    if !has3.is_empty() && has2.count() > 1 {
        let hi = has3.retain_highest();

        Some(HandRating::new_fullhouse(hi, has2.diff(hi)))
    } else {
        None
    }
}

#[inline]
pub(super) const fn eval_trips(
    has3: Rank16,
    has1: Rank16,
) -> Option<HandRating> {
    if has3.is_empty() {
        None
    } else {
        let hi = has3.retain_highest();

        Some(HandRating::new_trips(hi, has1.diff(hi)))
    }
}

#[inline]
pub(super) const fn eval_twopair(
    has2: Rank16,
    has1: Rank16,
) -> Option<HandRating> {
    if has2.count() > 1 {
        let hi = has2.retain_highest2();

        Some(HandRating::new_twopair(hi, has1.diff(hi)))
    } else {
        None
    }
}

#[inline]
pub(super) const fn eval_pair(
    has2: Rank16,
    has1: Rank16,
) -> Option<HandRating> {
    if has2.is_empty() {
        None
    } else {
        let hi = has2.retain_highest();

        Some(HandRating::new_pair(hi, has1.diff(hi)))
    }
}

#[inline]
#[cfg_attr(coverage_nightly, coverage(off))]
pub(super) const fn mk_straight_ranking<const FLUSH: bool>(
    has1: Rank16,
) -> HandRating {
    if FLUSH {
        HandRating::new_straightflush(has1)
    } else {
        HandRating::new_straight(has1)
    }
}

#[inline]
const fn eval_straight_inner<const FLUSH: bool>(
    has1: Rank16,
) -> Option<HandRating> {
    if has1.0 & Rank16::STRAIGHT_TJQKA.0 == Rank16::STRAIGHT_TJQKA.0 {
        Some(mk_straight_ranking::<FLUSH>(Rank16::STRAIGHT_TJQKA))
    } else if has1.0 & Rank16::STRAIGHT_9TJQK.0 == Rank16::STRAIGHT_9TJQK.0 {
        Some(mk_straight_ranking::<FLUSH>(Rank16::STRAIGHT_9TJQK))
    } else if has1.0 & Rank16::STRAIGHT_89TJQ.0 == Rank16::STRAIGHT_89TJQ.0 {
        Some(mk_straight_ranking::<FLUSH>(Rank16::STRAIGHT_89TJQ))
    } else if has1.0 & Rank16::STRAIGHT_789TJ.0 == Rank16::STRAIGHT_789TJ.0 {
        Some(mk_straight_ranking::<FLUSH>(Rank16::STRAIGHT_789TJ))
    } else if has1.0 & Rank16::STRAIGHT_6789T.0 == Rank16::STRAIGHT_6789T.0 {
        Some(mk_straight_ranking::<FLUSH>(Rank16::STRAIGHT_6789T))
    } else if has1.0 & Rank16::STRAIGHT_56789.0 == Rank16::STRAIGHT_56789.0 {
        Some(mk_straight_ranking::<FLUSH>(Rank16::STRAIGHT_56789))
    } else if has1.0 & Rank16::STRAIGHT_45678.0 == Rank16::STRAIGHT_45678.0 {
        Some(mk_straight_ranking::<FLUSH>(Rank16::STRAIGHT_45678))
    } else if has1.0 & Rank16::STRAIGHT_34567.0 == Rank16::STRAIGHT_34567.0 {
        Some(mk_straight_ranking::<FLUSH>(Rank16::STRAIGHT_34567))
    } else if has1.0 & Rank16::STRAIGHT_23456.0 == Rank16::STRAIGHT_23456.0 {
        Some(mk_straight_ranking::<FLUSH>(Rank16::STRAIGHT_23456))
    } else if has1.0 & Rank16::STRAIGHT_A2345.0 == Rank16::STRAIGHT_A2345.0 {
        Some(mk_straight_ranking::<FLUSH>(Rank16::R5))
    } else {
        None
    }
}

#[inline]
pub(super) const fn eval_straight(has1: Rank16) -> Option<HandRating> {
    eval_straight_inner::<false>(has1)
}

#[inline]
pub(super) const fn eval_straightflush(has1: Rank16) -> Option<HandRating> {
    eval_straight_inner::<true>(has1)
}

#[inline]
pub const fn eval_holdem_flush(c64: Card64) -> Option<HandRating> {
    if let Some(ranks) = flush_ranks(c64) {
        if let Some(ranking) = eval_straightflush(ranks) {
            Some(ranking)
        } else {
            Some(HandRating::new_flush(ranks.retain_highest5()))
        }
    } else {
        None
    }
}

/// Evaluates a Hold'em hand ranking excluding flush hands from 5-7 cards.
///
/// # Panics
/// Panics if the input is not a valid hand.
#[inline]
pub const fn eval_holdem_noflush(c64: Card64) -> HandRating {
    let [has1, has2, has3, has4] = count_ranks(c64);

    if let Some(ranking) = eval_quads(has4, has1) {
        return ranking;
    }

    if let Some(ranking) = eval_fullhouse(has3, has2) {
        return ranking;
    }

    if let Some(ranking) = eval_straight(has1) {
        return ranking;
    }

    if let Some(ranking) = eval_trips(has3, has1) {
        return ranking;
    }

    if let Some(ranking) = eval_twopair(has2, has1) {
        return ranking;
    }

    if let Some(ranking) = eval_pair(has2, has1) {
        return ranking;
    }

    HandRating::new_highcard(has1.retain_highest5())
}

/// Evaluates a Hold'em hand ranking from 5-7 cards.
///
/// # Panics
/// Panics if the input is not a valid hand.
#[inline]
pub const fn eval_holdem(c64: Card64) -> HandRating {
    if let Some(f) = eval_holdem_flush(c64) {
        f
    } else {
        eval_holdem_noflush(c64)
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use HandType::*;

    use super::*;
    use crate::*;

    fn assert_ranking(cs: &str, ht: HandType, hi: &str, lo: &str) {
        let l = eval_holdem(c64!(cs));
        let r = mk_rating(ht, hi, lo);
        assert_eq!(l, r, "{cs:?} {l:?} != {r:?}");
    }

    #[test]
    fn test_straight_flush() {
        assert_ranking("As Ks Qs Js Ts", StraightFlush, "A", "");
        assert_ranking("Kh Qh Jh Th 9h", StraightFlush, "K", "");
        assert_ranking("Qd Jd Td 9d 8d", StraightFlush, "Q", "");
        assert_ranking("Jc Tc 9c 8c 7c", StraightFlush, "J", "");
        assert_ranking("Td 9d 8d 7d 6d", StraightFlush, "T", "");
        assert_ranking("9s 8s 7s 6s 5s", StraightFlush, "9", "");
        assert_ranking("8h 7h 6h 5h 4h", StraightFlush, "8", "");
        assert_ranking("7d 6d 5d 4d 3d", StraightFlush, "7", "");
        assert_ranking("6c 2c 3c 4c 5c", StraightFlush, "6", "");
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

    #[test]
    fn test_order() {
        let data = [
            "As Ks Qs Js Ts 9s 8s", // StraightFlush A
            "Kh Qh Jh Th 9h 8h 7h", // StraightFlush K
            "Qd Jd Td 9d 8d 7d 6d", // StraightFlush Q
            "Jc Tc 9c 8c 7c 6c Ac", // StraightFlush J
            "Td 9d 8d 7d 6d Ad Kd", // StraightFlush T
            "Kh As Ah Ac Ad 6d 6c", // Quad A
            "7s 7h 7c 7d Ts 6d 6c", // Quad 7
            "7s 7h 7c 7d 6s 6d 6c", // Quad 7
            "Ks Kh Kd As Ah Ad Qs", // Fullhouse A
            "Ks Kh Kd As Ah Qd Qs", // Fullhouse K
            "Ks Kh Kd Qs Qh Qd As", // Fullhouse K
            "As 7s 8s 9s Js 6h 6d", // Flush A
            "Ks 7s 8s 9s Js 6s 6d", // Flush K
            "Js 9s 8s 7s 6s 6h 6d", // Flush J
            "As Kh Qd Jc Ts Ah Ad", // Straight A
            "Ts 9h 8d 7c 6s Ah Ad", // Straight T
            "Js 9h 8d 7c As Ah Ad", // Trips A
            "Ts 9h 8d 7c As Ah Ad", // Trips A
            "As Ah 8d Tc Ks 9s Th", // Twopair AT.K
            "Ts Th 6s 6h As Ah Qs", // Twopair AT.Q
            "Ts Th 7s 7h 6s 6h Ks", // Twopair T7
            "Ts Th 6s 6h As Kh Qs", // Twopair T6
            "As Ah 8d Tc Ks 9s Jh", // Pair A
            "Js Jh 8d 9c Ks As Th", // Pair J
            "6s 7h 8d 9c Js Qh Kd", // Highcard
        ];

        let mut v = data.to_vec();
        v.sort_by_key(|s| eval_holdem(c64!(s)));
        v.reverse();

        assert_eq!(&v, &data);
    }

    #[quickcheck]
    fn test_holdem7(cards: CardN<7>) -> TestResult {
        let res7 = eval_holdem(Card64::from(cards.clone()));

        let mut max5 = HandRating(0);

        for cs in cards.into_iter().combinations(5) {
            let res = eval_holdem(Card64::from(cs.as_slice()));

            if res > max5 {
                max5 = res;
            }
        }

        TestResult::from_bool(max5 == res7)
    }
}
