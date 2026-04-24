use crate::{PQLBoard, PQLCard, PQLGame, PQLHandType, functions::hand_type};

/// Returns `true` when the hand's made hand type matches `ht` exactly.
pub fn exact_hand_type(
    game: PQLGame,
    hand: &[PQLCard],
    board: PQLBoard,
    ht: PQLHandType,
) -> bool {
    hand_type(game, hand, board) == ht
}

#[cfg(test)]
mod tests {
    use openpql_prelude::CardN;
    use quickcheck::TestResult;
    use quickcheck_macros::quickcheck;

    use super::*;

    #[quickcheck]
    fn test_exact_hand_type(game: PQLGame, cards: CardN<9>) -> TestResult {
        let n = game.player_cards_len() as usize;
        let hand = &cards.as_ref()[..n];
        let board = PQLBoard::from(&cards.as_ref()[n..n + 5]);

        let ht = hand_type(game, hand, board);

        for kind in PQLHandType::ARR_ALL {
            assert_eq!(exact_hand_type(game, hand, board, kind), kind == ht,);
        }

        TestResult::passed()
    }
}
