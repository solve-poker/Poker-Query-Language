use super::{
    Board, Card64, FlopHandCategory, HandRatingView, HandType, eval_holdem,
};

// TODO: refactor later
pub fn eval_flop_holdem(player: Card64, board: Board) -> FlopHandCategory {
    board.flop.map_or(FlopHandCategory::Nothing, |flop| {
        let flop64 = Card64::from(flop);
        let rating = eval_holdem(player | flop64);
        let HandRatingView {
            hand_type: ht,
            high,
            low: _,
        } = rating.into();

        match ht {
            HandType::StraightFlush => FlopHandCategory::StraightFlush,
            HandType::Quads => FlopHandCategory::Quads,
            HandType::FullHouse => FlopHandCategory::FullHouse,
            HandType::Flush => FlopHandCategory::Flush,
            HandType::Straight => FlopHandCategory::Straight,
            HandType::Trips => {
                let trips_rank = high.max_rank().unwrap();

                if flop64.count_by_rank(trips_rank) == 2 {
                    FlopHandCategory::Trips
                } else {
                    FlopHandCategory::Set
                }
            }
            HandType::TwoPair => {
                let [_, mid, top] = flop.0.map(|card| card.rank);

                match (high.contains_rank(top), high.contains_rank(mid)) {
                    (true, true) => FlopHandCategory::TopTwo,
                    (true, _) => FlopHandCategory::TopAndBottom,
                    (_, _) => FlopHandCategory::BottomTwo,
                }
            }
            HandType::Pair => {
                let pair_rank = high.max_rank().unwrap();

                let [btm, mid, top] = flop.0.map(|card| card.rank);

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
        }
    })
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

        assert_eq!(eval_flop_holdem(p, b), expected);
    }

    #[test]
    fn test_flop_hand_category_holdem() {
        assert_flop_cat("8s 9s | 7s 6s Ts", StraightFlush);
        assert_flop_cat("8s 8h | 8d 8c Ts", Quads);
        assert_flop_cat("7s 6s | 7h 6h 6c", FullHouse);
        assert_flop_cat("8s 9s | 7s 6s 2s", Flush);
        assert_flop_cat("8d 9s | 7s 6h Tc", Straight);
        assert_flop_cat("7h 7d | 7s 6h Tc", Set);
        assert_flop_cat("7d 8s | 7s 7h Tc", Trips);
        assert_flop_cat("8s Ts | 7s 8h Tc", TopTwo);
        assert_flop_cat("7c Ts | 7s 8h Tc", TopAndBottom);
        assert_flop_cat("7c 8c | 7s 8h Tc", BottomTwo);
        assert_flop_cat("Js Jh | 7s 8h Tc", Overpair);
        assert_flop_cat("Ts Ah | 7s 8h Tc", TopPair);
        assert_flop_cat("9s 9h | 7s 8h Tc", Pocket12);
        assert_flop_cat("8s Ah | 7s 8h Tc", SecondPair);
        assert_flop_cat("7s 7h | 6s 8h Tc", Pocket23);
        assert_flop_cat("7h Ah | 7s 8h Tc", ThirdPair);
        assert_flop_cat("4s 4h | 6s 8h Tc", UnderPair);
        assert_flop_cat("4s 2h | 6s 8h Tc", Nothing);
    }
}
