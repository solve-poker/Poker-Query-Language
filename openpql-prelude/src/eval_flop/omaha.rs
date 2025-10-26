use super::{Board, Card64, FlopHandCategory, cmp, eval_flop_holdem};

// TODO: refactor
pub fn eval_flop_omaha(player: Card64, board: Board) -> FlopHandCategory {
    let player_cards: Vec<_> = player.iter().collect();

    let c0 = player_cards[0];
    let c1 = player_cards[1];
    let c2 = player_cards[2];
    let c3 = player_cards[3];

    let mut max = FlopHandCategory::default();

    for hand in [[c0, c1], [c0, c2], [c0, c3], [c1, c2], [c1, c3], [c2, c3]] {
        let cur = eval_flop_holdem(hand.into_iter().collect(), board);

        if cur.compare::<false>(max) == cmp::Ordering::Greater {
            max = cur;
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use FlopHandCategory::*;

    use super::*;
    use crate::*;

    fn assert_flop_cat(s: &str, expected: FlopHandCategory) {
        let mut s = s.split('|');
        let p = c64!(s.next().unwrap());
        let b = board!(s.next().unwrap());

        assert_eq!(eval_flop_omaha(p, b), expected);
    }

    #[test]
    fn test_flop_hand_category_omaha() {
        assert_flop_cat("3d 6c As Ks | Qs Js Ts", StraightFlush);
        assert_flop_cat("3d 6c As Ah | Ad Ac Ks", Quads);
        assert_flop_cat("3d 6c As Ah | Ad Kc Ks", FullHouse);
        assert_flop_cat("3d 6c As Ks | Qs Js 9s", Flush);
        assert_flop_cat("3d 6c As Kh | Qd Jc Ts", Straight);
        assert_flop_cat("3d 6c As Ah | Ad Kc Qs", Set);
        assert_flop_cat("3d 6c As 2h | Ad Ac Qs", Trips);
        assert_flop_cat("3d 6c Js Qh | Td Jc Qs", TopTwo);
        assert_flop_cat("3d 6c Ts Qh | Td Jc Qs", TopAndBottom);
        assert_flop_cat("3d 6c Js Th | Td Jc Qs", BottomTwo);
        assert_flop_cat("3d 6c As Ah | Kd Qc Js", Overpair);
        assert_flop_cat("3d 6c Ks 2h | Kd Qc Js", TopPair);
        assert_flop_cat("3d 6c Qs Qh | Kd Tc 7s", Pocket12);
        assert_flop_cat("3d 6c Ts 2h | Kd Tc 7s", SecondPair);
        assert_flop_cat("3d 6c 9s 9h | Kd Tc 7s", Pocket23);
        assert_flop_cat("3d 6c 7h 2h | Kd Tc 7s", ThirdPair);
        assert_flop_cat("3d 6c 2s 2h | Kd Tc 7s", UnderPair);
        assert_flop_cat("3d 6c As Kh | Qd Jc 9s", Nothing);
    }
}
