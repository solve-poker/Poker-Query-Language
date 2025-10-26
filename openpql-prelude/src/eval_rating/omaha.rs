use super::{Card64, HandRating, Rank16, count_ranks, flush_ranks_omaha};

/// Evaluates an Omaha hand ranking from player cards and board cards.
///
/// # Panics
/// Panics if the input is not a valid hand.
#[inline]
pub const fn eval_omaha(player: Card64, board: Card64) -> HandRating {
    let nf = eval_omaha_noflush(player, board);
    if let Some(f) = eval_omaha_flush(player, board)
        && f.0 > nf.0
    {
        return f;
    }

    nf
}

/// Evaluates an Omaha hand ranking excluding flush hands.
#[inline]
pub const fn eval_omaha_noflush(player: Card64, board: Card64) -> HandRating {
    let [p1, p2, _, _] = count_ranks(player);
    let [b1, b2, b3, _] = count_ranks(board);

    if let Some(ranking) = eval_quads(p2, p1, b3, b2, b1) {
        return ranking;
    }

    if let Some(ranking) = eval_fullhouse(p2, p1, b3, b2, b1) {
        return ranking;
    }

    if let Some(hi) = eval_straight(p1, b1) {
        return HandRating::new_straight(hi);
    }

    if let Some(ranking) = eval_trips(p2, p1, b3, b2, b1) {
        return ranking;
    }

    if let Some(ranking) = eval_twopair(p2, p1, b2, b1) {
        return ranking;
    }

    if let Some(ranking) = eval_pair(p2, p1, b2, b1) {
        return ranking;
    }

    HandRating::new_highcard(Rank16(
        b1.retain_highest3().0 | p1.retain_highest2().0,
    ))
}

/// Evaluates an Omaha flush hand.
#[inline]
pub const fn eval_omaha_flush(
    player: Card64,
    board: Card64,
) -> Option<HandRating> {
    if let Some((p, b)) = flush_ranks_omaha(player, board) {
        if let Some(hi) = eval_straight(p, b) {
            return Some(HandRating::new_straightflush(hi));
        }

        Some(HandRating::new_flush(Rank16(
            p.retain_highest2().0 | b.retain_highest3().0,
        )))
    } else {
        None
    }
}

#[inline]
const fn intersect(l: Rank16, r: Rank16) -> Option<Rank16> {
    let v = l.0 & r.0;
    if v != 0 { Some(Rank16(v)) } else { None }
}

#[inline]
const fn highest_from_each(l: Rank16, r: Rank16) -> Rank16 {
    Rank16(l.retain_highest().0 | r.retain_highest().0)
}

#[inline]
const fn max(
    lhs: Option<(Rank16, Rank16)>,
    rhs: Option<(Rank16, Rank16)>,
) -> Option<(Rank16, Rank16)> {
    const fn gt(lhs: (Rank16, Rank16), rhs: (Rank16, Rank16)) -> bool {
        lhs.0.0 > rhs.0.0 // || (lhs.0.0 == rhs.0.0 && lhs.1.0 > rhs.1.0) <-- 0 cases
    }

    match (lhs, rhs) {
        (Some(l), Some(r)) => {
            if gt(l, r) {
                Some(l)
            } else {
                Some(r)
            }
        }
        (Some(l), None) => Some(l),
        (None, Some(r)) => Some(r),
        (None, None) => None,
    }
}

// PAIR Case I: aax; yz
#[inline]
const fn eval_pair_1(
    p1: Rank16,
    b2: Rank16,
    b1: Rank16,
) -> Option<(Rank16, Rank16)> {
    if b2.is_empty() {
        None
    } else {
        let hi = b2.retain_highest();
        Some((hi, b1.diff(hi).retain_highest().or(p1.retain_highest2())))
    }
}

// PAIR Case II: axy; az
#[inline]
const fn eval_pair_2(p1: Rank16, b1: Rank16) -> Option<(Rank16, Rank16)> {
    match intersect(p1, b1) {
        Some(hi) => Some((
            hi,
            b1.diff(hi)
                .retain_highest2()
                .or(p1.diff(hi).retain_highest()),
        )),
        None => None,
    }
}

// PAIR Case III: xyz; aa
#[inline]
const fn eval_pair_3(p2: Rank16, b1: Rank16) -> Option<(Rank16, Rank16)> {
    if p2.is_empty() {
        None
    } else {
        Some((p2, b1.retain_highest3()))
    }
}

#[inline]
const fn eval_pair(
    p2: Rank16,
    p1: Rank16,
    b2: Rank16,
    b1: Rank16,
) -> Option<HandRating> {
    if let Some((h, l)) = max(
        max(eval_pair_1(p1, b2, b1), eval_pair_2(p1, b1)),
        eval_pair_3(p2, b1),
    ) {
        Some(HandRating::new_pair(h, l))
    } else {
        None
    }
}

// TWOPAIR Case I: aak; bb
#[inline]
const fn eval_twopair_1(
    p2: Rank16,
    b2: Rank16,
    b1: Rank16,
) -> Option<(Rank16, Rank16)> {
    if b2.is_empty() || p2.is_empty() {
        None
    } else {
        let hi = highest_from_each(p2, b2);

        Some((hi, b1.diff(hi)))
    }
}

// TWOPAIR Case II: aab; bk
#[inline]
const fn eval_twopair_2(
    p1: Rank16,
    b2: Rank16,
    b1: Rank16,
) -> Option<(Rank16, Rank16)> {
    if let Some(pair) = intersect(b1, p1)
        && !b2.is_empty()
    {
        let hi = highest_from_each(pair, b2);

        return Some((hi, p1.diff(hi)));
    }

    None
}

// TWOPAIR Case III: abk; ab
#[inline]
const fn eval_twopair_3(p1: Rank16, b1: Rank16) -> Option<(Rank16, Rank16)> {
    if let Some(pair) = intersect(p1, b1)
        && pair.count() > 1
    {
        let hi = pair.retain_highest2();
        return Some((hi, b1.diff(hi)));
    }

    None
}

#[inline]
const fn eval_twopair(
    p2: Rank16,
    p1: Rank16,
    b2: Rank16,
    b1: Rank16,
) -> Option<HandRating> {
    if let Some((h, l)) = max(
        max(eval_twopair_1(p2, b2, b1), eval_twopair_2(p1, b2, b1)),
        eval_twopair_3(p1, b1),
    ) {
        Some(HandRating::new_twopair(h, l))
    } else {
        None
    }
}

// TRIPS Case I: aaa; xy
#[inline]
const fn eval_trips_1(p1: Rank16, b3: Rank16) -> Option<(Rank16, Rank16)> {
    if b3.is_empty() {
        None
    } else {
        Some((b3, p1.retain_highest2()))
    }
}

// TRIPS Case II: aax; ay
#[inline]
const fn eval_trips_2(
    p1: Rank16,
    b2: Rank16,
    b1: Rank16,
) -> Option<(Rank16, Rank16)> {
    match intersect(b2, p1) {
        Some(hi) => Some((hi, highest_from_each(b1.diff(hi), p1.diff(hi)))),
        None => None,
    }
}

// TRIPS Case III: axy; aa
#[inline]
const fn eval_trips_3(p2: Rank16, b1: Rank16) -> Option<(Rank16, Rank16)> {
    match intersect(p2, b1) {
        Some(trips) => {
            let hi = trips.retain_highest();

            Some((hi, b1.diff(hi).retain_highest2()))
        }
        None => None,
    }
}

#[inline]
const fn eval_trips(
    p2: Rank16,
    p1: Rank16,
    b3: Rank16,
    b2: Rank16,
    b1: Rank16,
) -> Option<HandRating> {
    if let Some((h, l)) = max(
        max(eval_trips_1(p1, b3), eval_trips_2(p1, b2, b1)),
        eval_trips_3(p2, b1),
    ) {
        Some(HandRating::new_trips(h, l))
    } else {
        None
    }
}

#[inline]
const fn eval_straight_x(
    mask: Rank16,
    p1: Rank16,
    b1: Rank16,
) -> Option<Rank16> {
    if (mask.0 & (p1.0 | b1.0)) == mask.0
        && (mask.0 & p1.0).count_ones() >= 2
        && (mask.0 & b1.0).count_ones() >= 3
    {
        Some(mask.retain_highest())
    } else {
        None
    }
}

const fn eval_straight(p1: Rank16, b1: Rank16) -> Option<Rank16> {
    if let Some(r) = eval_straight_x(Rank16::STRAIGHT_TJQKA, p1, b1) {
        return Some(r);
    }

    if let Some(r) = eval_straight_x(Rank16::STRAIGHT_9TJQK, p1, b1) {
        return Some(r);
    }

    if let Some(r) = eval_straight_x(Rank16::STRAIGHT_89TJQ, p1, b1) {
        return Some(r);
    }

    if let Some(r) = eval_straight_x(Rank16::STRAIGHT_789TJ, p1, b1) {
        return Some(r);
    }

    if let Some(r) = eval_straight_x(Rank16::STRAIGHT_6789T, p1, b1) {
        return Some(r);
    }

    if let Some(r) = eval_straight_x(Rank16::STRAIGHT_56789, p1, b1) {
        return Some(r);
    }

    if let Some(r) = eval_straight_x(Rank16::STRAIGHT_45678, p1, b1) {
        return Some(r);
    }

    if let Some(r) = eval_straight_x(Rank16::STRAIGHT_34567, p1, b1) {
        return Some(r);
    }

    if let Some(r) = eval_straight_x(Rank16::STRAIGHT_23456, p1, b1) {
        return Some(r);
    }

    if eval_straight_x(Rank16::STRAIGHT_A2345, p1, b1).is_some() {
        return Some(Rank16::R5);
    }

    None
}

// FULLHOUSE Case I: aaa; bb
#[inline]
const fn eval_fullhouse_1(p2: Rank16, b3: Rank16) -> Option<(Rank16, Rank16)> {
    if b3.is_empty() || p2.is_empty() {
        None
    } else {
        Some((b3, p2))
    }
}

// FULLHOUSE Case II: abb; aa
#[inline]
const fn eval_fullhouse_2(
    p2: Rank16,
    b2: Rank16,
    b1: Rank16,
) -> Option<(Rank16, Rank16)> {
    if let Some(trips) = intersect(p2, b1) {
        let hi = trips.retain_highest();
        let lo = b2.diff(hi);

        if !lo.is_empty() {
            return Some((hi, lo));
        }
    }

    None
}

// FULLHOUSE Case III: abb; ab
#[inline]
const fn eval_fullhouse_3(
    p1: Rank16,
    b2: Rank16,
    b1: Rank16,
) -> Option<(Rank16, Rank16)> {
    if let Some(trips) = intersect(b2, p1) {
        let hi = trips.retain_highest();

        if let Some(lo) = intersect(b1.diff(hi), p1) {
            return Some((hi, lo));
        }
    }

    None
}

#[inline]
const fn eval_fullhouse(
    p2: Rank16,
    p1: Rank16,
    b3: Rank16,
    b2: Rank16,
    b1: Rank16,
) -> Option<HandRating> {
    if let Some((h, l)) = max(
        max(eval_fullhouse_1(p2, b3), eval_fullhouse_2(p2, b2, b1)),
        eval_fullhouse_3(p1, b2, b1),
    ) {
        Some(HandRating::new_fullhouse(h, l))
    } else {
        None
    }
}

// QUADS Case I: aak; aa
#[inline]
const fn eval_quads_1(
    p2: Rank16,
    b2: Rank16,
    b1: Rank16,
) -> Option<(Rank16, Rank16)> {
    match intersect(p2, b2) {
        Some(quad) => {
            let hi = quad.retain_highest();

            Some((hi, b1.diff(hi)))
        }
        None => None,
    }
}

// QUADS Case II: aaa; ak
#[inline]
const fn eval_quads_2(p1: Rank16, b3: Rank16) -> Option<(Rank16, Rank16)> {
    match intersect(p1, b3) {
        Some(hi) => Some((hi, p1.diff(hi))),
        None => None,
    }
}

#[inline]
const fn eval_quads(
    p2: Rank16,
    p1: Rank16,
    b3: Rank16,
    b2: Rank16,
    b1: Rank16,
) -> Option<HandRating> {
    match max(eval_quads_1(p2, b2, b1), eval_quads_2(p1, b3)) {
        Some((hi, lo)) => Some(HandRating::new_quad(hi, lo)),
        None => None,
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use HandType::*;

    use super::*;
    use crate::*;

    fn assert_ranking(p: &str, b: &str, ht: HandType, hi: &str, lo: &str) {
        let l = eval_omaha(c64!(p), c64!(b));
        let r = mk_rating(ht, hi, lo);
        assert_eq!(l, r, "{p} {b} {l:?} != {r:?}");
    }

    #[test]
    fn test_straightflush() {
        assert_ranking("As Ks Qh Jh", "Qs Js Ts Ah Kh", StraightFlush, "A", "");
        assert_ranking("Kh Qh Jd Td", "Jh Th 9h Kd Qd", StraightFlush, "K", "");
        assert_ranking("Qd Jd Tc 9c", "Td 9d 8d Qc Jc", StraightFlush, "Q", "");
        assert_ranking("Jc Tc 9s 8s", "9c 8c 7c Js Ts", StraightFlush, "J", "");
        assert_ranking("Ts 9s 8h 7h", "8s 7s 6s Th 9h", StraightFlush, "T", "");
        assert_ranking("9h 8h 7d 6d", "7h 6h 5h 9d 8d", StraightFlush, "9", "");
        assert_ranking("8d 7d 6c 5c", "6d 5d 4d 8c 7c", StraightFlush, "8", "");
        assert_ranking("7c 6c 5s 4s", "5c 4c 3c 7s 6s", StraightFlush, "7", "");
        assert_ranking("6s 2s 3h 4h", "3s 4s 5s 6h 2h", StraightFlush, "6", "");
        assert_ranking("Ah 2h 3d 4d", "3h 4h 5h Ad 2d", StraightFlush, "5", "");
    }

    #[test]
    fn test_quads() {
        assert_ranking("Ac Ks Kh Qs", "As Ah Ad Ks Kh", Quads, "A", "K");
        assert_ranking("Ac Ad Kc Qs", "Ks Kh Kd As Ah", Quads, "A", "K");
    }

    #[test]
    fn test_fullhouse() {
        assert_ranking("As Ah 2s 2h", "Ac Ks Kh Kd 2c", FullHouse, "A", "K"); // AA22 AKKK2
        assert_ranking("Ks Kh 2s 2h", "Kc As Ah Ad 2c", FullHouse, "A", "K"); // KK22 KAAA2
        assert_ranking("2s 2h Ks Kh", "2c As Ah Ad Kc", FullHouse, "A", "K"); // 22KK 2AAAK
        assert_ranking("2s 2h As Ah", "2c Ks Kh Kd Ac", FullHouse, "A", "K"); // 22AA 2KKKA

        assert_ranking("As Ks 2s 2h", "Kh Kc 2d Ah Ac", FullHouse, "A", "K"); // AK22 KK2AA
        assert_ranking("As 2s Ks Kh", "2h 2c Kd Ah Ac", FullHouse, "A", "K"); // A2KK 22KAA
        assert_ranking("Ks As 2s 2h", "Ah Ac 2d Kh Kc", FullHouse, "A", "K"); // KA22 AA2KK
        assert_ranking("Ks 2s As Ah", "2h 2c Ad Kh Kc", FullHouse, "A", "K"); // K2AA 22AKK
        assert_ranking("2s As Ks Kh", "Ah Ac Kd 2h 2c", FullHouse, "A", "K"); // 2AKK AAK22
        assert_ranking("2s Ks As Ah", "Kh Kc Ad 2h 2c", FullHouse, "A", "K"); // 2KAA KKA22
    }

    #[test]
    fn test_omaha() {
        let data = [
            ("Js Ts 9s 8s", "As Ks Qs Th Jh", StraightFlush, "A", ""),
            ("As 5s 9s 8s", "2s 3s 4s Th Jh", StraightFlush, "5", ""),
            ("As Ah Ks Kh", "Ac Ad Kc Kd Qs", Quads, "A", "K"),
            ("Qs Qh Qc 7s", "7h 7c 7d Qd As", Quads, "7", "Q"),
            ("Ac Kc 7s 7h", "7c Ks kh As Ah", FullHouse, "A", "K"),
            ("7s 7h 8s 8h", "7c 6s 6h As Ah", FullHouse, "7", "A"),
            ("Qs Qh 8s 8h", "7h 7c 7d As Ah", FullHouse, "7", "Q"),
            ("As Ks 2s 3s", "Js Ts 9s 2h 3h", Flush, "AKJT9", ""),
            ("Js Th 9d 8c", "As Kh Qd Tc Jc", Straight, "A", ""),
            ("As 5h 9d 8c", "2s 3h 4d Tc Jc", Straight, "5", ""),
            ("Ac Ks Kh 2s", "7c Ts Th As Ah", Trips, "A", "KT"),
            ("Ks Qh 8s 8h", "Kh Qc 8c 2s 3h", Trips, "8", "KQ"),
            ("Ks Qh 8s 9h", "7h 7c 7d As Ah", Trips, "7", "KQ"),
            ("Ks Ah 8d 8c", "7s 7h Qs 2h 3h", TwoPair, "78", "Q"),
            ("8s Qh 2s 3s", "7h 7c 8d As Kh", TwoPair, "78", "Q"),
            ("7s 8s As Ks", "7h 8h Qs 2h 3h", TwoPair, "78", "Q"),
            ("7s 7h 4c 5d", "As Kh Qc Jd Ts", Pair, "7", "AKQ"),
            ("3h 4c 5d 7h", "7s As Kh Qc 2s", Pair, "7", "AK5"),
            ("2s 3h 4c 5d", "7s 7h As Kh Qc", Pair, "7", "A45"),
            ("2s 6s 7h 8c", "As Kh Qc Jd Ts", HighCard, "AKQ78", ""),
        ];

        let ratings = data.map(|(_, _, ht, hi, lo)| mk_rating(ht, hi, lo));
        let mut res = data.map(|(p, b, _, _, _)| eval_omaha(c64!(p), c64!(b)));

        assert_eq!(res, ratings);

        res.sort_unstable();
        res.reverse();
        assert_eq!(res, ratings);
    }

    #[quickcheck]
    fn test_omaha9(cards: CardN<9>) {
        let (hand, board): (CardN<4>, CardN<5>) = cards.into();

        let res9 = eval_omaha(
            Card64::from(hand.as_slice()),
            Card64::from(board.as_slice()),
        );

        let mut max5 = HandRating::default();

        for h in hand.as_slice().iter().combinations(2) {
            for b in board.clone().into_iter().combinations(3) {
                let mut cs = Card64::default();

                cs.set(*h[0]);
                cs.set(*h[1]);
                cs.set(b[0]);
                cs.set(b[1]);
                cs.set(b[2]);

                let res = eval_holdem(cs);

                if res > max5 {
                    max5 = res;
                }
            }
        }

        assert_eq!(max5, res9, "{hand:?} {board:?} {max5} != {res9}");
    }
}
