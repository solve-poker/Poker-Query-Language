use crate::{PQLBoard, PQLCard, PQLGame, PQLHiRating, functions::hi_rating};

/// Returns `true` when the hand's high-hand rating is at least `rating`.
pub fn min_hi_rating(
    game: PQLGame,
    hand: &[PQLCard],
    board: PQLBoard,
    rating: PQLHiRating,
) -> bool {
    hi_rating(game, hand, board) >= rating
}

#[cfg(test)]
mod tests {
    use openpql_prelude::CardN;
    use quickcheck::TestResult;
    use quickcheck_macros::quickcheck;

    use super::*;

    #[quickcheck]
    fn test_min_hi_rating(game: PQLGame, cards: CardN<13>) -> TestResult {
        let n = game.player_cards_len() as usize;
        let hand = &cards.as_ref()[..n];
        let other = &cards.as_ref()[n..2 * n];
        let board = PQLBoard::from(&cards.as_ref()[2 * n..2 * n + 5]);

        let rating = hi_rating(game, other, board);
        TestResult::from_bool(
            min_hi_rating(game, hand, board, rating) == (hi_rating(game, hand, board) >= rating),
        )
    }
}
