use super::*;
#[pqlfn(arg, rtn, eval)]
pub fn min_hi_rating(
    hand: &Hand,
    street: PQLStreet,
    rating: PQLHiRating,
    (game, board): (PQLGame, Board),
) -> PQLBoolean {
    hi_rating(hand, street, (game, board)) >= rating
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_min_hi_rating(hbg: HandBoardGame) -> TestResult {
        let HandBoardGame {
            street,
            game,
            hand,
            board,
            ..
        } = hbg;

        let rating = hi_rating(&hand, street, (game, board));

        let add1 = PQLHiRating::new(rating.to_i16() + 1);
        let sub1 = PQLHiRating::new(rating.to_i16() - 1);

        TestResult::from_bool(
            !min_hi_rating(&hand, street, add1, (game, board))
                && min_hi_rating(&hand, street, rating, (game, board))
                && min_hi_rating(&hand, street, sub1, (game, board)),
        )
    }
}
