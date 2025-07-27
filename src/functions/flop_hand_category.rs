use super::*;
#[pqlfn(arg, rtn, eval)]
/// # Panics
/// high rank bits from ranking is always non-empty
pub fn flop_hand_category(
    hand: &Hand,
    (game, flop): (PQLGame, Flop),
) -> PQLFlopHandCategory {
    let board =
        (flop.0, flop.1, flop.2, Card::default(), Card::default()).into();

    let rating = hi_rating(hand, PQLStreet::Flop, (game, board));

    let (ht, _, high) = rating.to_hand_type_and_low_high_ranks(game);

    (
        match ht.ht {
            HandType::StraightFlush => FlopHandCategory::StraightFlush,
            HandType::Quads => FlopHandCategory::Quads,
            HandType::FullHouse => FlopHandCategory::FullHouse,
            HandType::Flush => FlopHandCategory::Flush,
            HandType::Straight => FlopHandCategory::Straight,
            HandType::Trips => {
                let trips_rank = max_rank(high).unwrap();

                if flop.count_by_rank(trips_rank) == 2 {
                    FlopHandCategory::Trips
                } else {
                    FlopHandCategory::Set
                }
            }
            HandType::TwoPair => {
                let (top, mid, _) = flop.sorted_ranks();

                match (high.contains_rank(top), high.contains_rank(mid)) {
                    (true, true) => FlopHandCategory::TopTwo,
                    (true, _) => FlopHandCategory::TopAndBottom,
                    (_, _) => FlopHandCategory::BottomTwo,
                }
            }
            HandType::Pair => {
                let pair_rank = max_rank(high).unwrap();

                let (top, mid, btm) = flop.sorted_ranks();

                if pair_rank > top {
                    FlopHandCategory::Overpair
                } else if pair_rank == top {
                    FlopHandCategory::TopPair
                } else if pair_rank > mid {
                    FlopHandCategory::Pocket12
                } else if pair_rank == mid {
                    FlopHandCategory::SecondPair
                } else if pair_rank > btm {
                    FlopHandCategory::Pocket23
                } else if pair_rank == btm {
                    FlopHandCategory::ThirdPair
                } else {
                    FlopHandCategory::UnderPair
                }
            }
            HandType::HighCard => FlopHandCategory::Nothing,
        },
        game,
    )
        .into()
}

#[cfg(test)]
mod tests {
    use FlopHandCategory::*;

    use super::*;
    use crate::*;

    fn assert_flop_cat(g: PQLGame, s: &str, c: FlopHandCategory) {
        let mut s = s.split('|');

        assert_eq!(
            flop_hand_category(
                cards!(s.next().unwrap()).as_ref(),
                (g, flop!(s.next().unwrap())),
            ),
            (c, g).into()
        );
    }

    #[test]
    fn test_flop_hand_category_holdem() {
        let g = PQLGame::Holdem;
        assert_flop_cat(g, "8s 9s | 7s 6s Ts", StraightFlush);
        assert_flop_cat(g, "8s 8h | 8d 8c Ts", Quads);
        assert_flop_cat(g, "7s 6s | 7h 6h 6c", FullHouse);
        assert_flop_cat(g, "8s 9s | 7s 6s 2s", Flush);
        assert_flop_cat(g, "8d 9s | 7s 6h Tc", Straight);
        assert_flop_cat(g, "7h 7d | 7s 6h Tc", Set);
        assert_flop_cat(g, "7d 8s | 7s 7h Tc", Trips);
        assert_flop_cat(g, "8s Ts | 7s 8h Tc", TopTwo);
        assert_flop_cat(g, "7c Ts | 7s 8h Tc", TopAndBottom);
        assert_flop_cat(g, "7c 8c | 7s 8h Tc", BottomTwo);
        assert_flop_cat(g, "Js Jh | 7s 8h Tc", Overpair);
        assert_flop_cat(g, "Ts Ah | 7s 8h Tc", TopPair);
        assert_flop_cat(g, "9s 9h | 7s 8h Tc", Pocket12);
        assert_flop_cat(g, "8s Ah | 7s 8h Tc", SecondPair);
        assert_flop_cat(g, "7s 7h | 6s 8h Tc", Pocket23);
        assert_flop_cat(g, "7h Ah | 7s 8h Tc", ThirdPair);
        assert_flop_cat(g, "4s 4h | 6s 8h Tc", UnderPair);
        assert_flop_cat(g, "4s 2h | 6s 8h Tc", Nothing);
    }

    #[test]
    fn test_flop_hand_category_omaha() {
        let g = PQLGame::Omaha;
        assert_flop_cat(g, "3d 6c As Ks | Qs Js Ts", StraightFlush);
        assert_flop_cat(g, "3d 6c As Ah | Ad Ac Ks", Quads);
        assert_flop_cat(g, "3d 6c As Ah | Ad Kc Ks", FullHouse);
        assert_flop_cat(g, "3d 6c As Ks | Qs Js 9s", Flush);
        assert_flop_cat(g, "3d 6c As Kh | Qd Jc Ts", Straight);
        assert_flop_cat(g, "3d 6c As Ah | Ad Kc Qs", Set);
        assert_flop_cat(g, "3d 6c As 2h | Ad Ac Qs", Trips);
        assert_flop_cat(g, "3d 6c Js Qh | Td Jc Qs", TopTwo);
        assert_flop_cat(g, "3d 6c Ts Qh | Td Jc Qs", TopAndBottom);
        assert_flop_cat(g, "3d 6c Js Th | Td Jc Qs", BottomTwo);
        assert_flop_cat(g, "3d 6c As Ah | Kd Qc Js", Overpair);
        assert_flop_cat(g, "3d 6c Ks 2h | Kd Qc Js", TopPair);
        assert_flop_cat(g, "3d 6c Qs Qh | Kd Tc 7s", Pocket12);
        assert_flop_cat(g, "3d 6c Ts 2h | Kd Tc 7s", SecondPair);
        assert_flop_cat(g, "3d 6c 9s 9h | Kd Tc 7s", Pocket23);
        assert_flop_cat(g, "3d 6c 7s 2h | Kd Tc 7s", ThirdPair);
        assert_flop_cat(g, "3d 6c 2s 2h | Kd Tc 7s", UnderPair);
        assert_flop_cat(g, "3d 6c As Kh | Qd Jc 9s", Nothing);
    }
}
