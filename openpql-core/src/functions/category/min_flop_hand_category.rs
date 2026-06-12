use std::cmp::Ordering;

use crate::{PQLBoard, PQLCard, PQLFlopHandCategory, PQLGame, functions::flop_hand_category};

/// Returns `true` when the hand's flop category is at least `category`.
pub fn min_flop_hand_category(
    game: PQLGame,
    hand: &[PQLCard],
    board: PQLBoard,
    category: PQLFlopHandCategory,
) -> bool {
    let computed = flop_hand_category(game, hand, board);

    let order = if game.is_shortdeck() {
        computed.compare::<true>(category)
    } else {
        computed.compare::<false>(category)
    };

    !matches!(order, Ordering::Less)
}

#[cfg(test)]
mod tests {
    use openpql_prelude::CardN;
    use quickcheck::TestResult;
    use quickcheck_macros::quickcheck;

    use super::*;

    #[quickcheck]
    fn test_min_flop_hand_category(game: PQLGame, cards: CardN<10>) -> TestResult {
        let n = game.player_cards_len() as usize;
        let hand = &cards.as_ref()[..n];
        let board = PQLBoard::from(&cards.as_ref()[n..n + 5]);

        let category = flop_hand_category(game, hand, board);
        let compare = if game.is_shortdeck() {
            PQLFlopHandCategory::compare::<true>
        } else {
            PQLFlopHandCategory::compare::<false>
        };

        for cat in PQLFlopHandCategory::ARR_ALL {
            assert_eq!(
                min_flop_hand_category(game, hand, board, cat),
                !matches!(compare(category, cat), Ordering::Less),
            );
        }

        TestResult::passed()
    }
}
