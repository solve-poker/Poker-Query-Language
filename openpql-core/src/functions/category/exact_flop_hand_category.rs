use crate::{PQLBoard, PQLCard, PQLFlopHandCategory, PQLGame, functions::flop_hand_category};

/// Returns `true` when the hand's flop category matches `category` exactly.
pub fn exact_flop_hand_category(
    game: PQLGame,
    hand: &[PQLCard],
    board: PQLBoard,
    category: PQLFlopHandCategory,
) -> bool {
    flop_hand_category(game, hand, board) == category
}

#[cfg(test)]
mod tests {
    use openpql_prelude::CardN;
    use quickcheck::TestResult;
    use quickcheck_macros::quickcheck;

    use super::*;

    #[quickcheck]
    fn test_exact_flop_hand_category(game: PQLGame, cards: CardN<10>) -> TestResult {
        let n = game.player_cards_len() as usize;
        let hand = &cards.as_ref()[..n];
        let board = PQLBoard::from(&cards.as_ref()[n..n + 5]);

        let category = flop_hand_category(game, hand, board);

        for cat in PQLFlopHandCategory::ARR_ALL {
            assert_eq!(
                exact_flop_hand_category(game, hand, board, cat),
                cat == category,
            );
        }

        TestResult::passed()
    }
}
