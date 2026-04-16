use std::cmp::Ordering;

use crate::{PQLBoard, PQLCard, PQLGame, PQLHandType, functions::hand_type};

pub fn min_hand_type(
    game: PQLGame,
    hand: &[PQLCard],
    board: PQLBoard,
    ht: PQLHandType,
) -> bool {
    let computed = hand_type(game, hand, board);

    let order = if game.is_shortdeck() {
        computed.compare::<true>(ht)
    } else {
        computed.compare::<false>(ht)
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
    fn test_min_hand_type(game: PQLGame, cards: CardN<9>) -> TestResult {
        let n = game.player_cards_len() as usize;
        let hand = &cards.as_ref()[..n];
        let board = PQLBoard::from(&cards.as_ref()[n..n + 5]);

        let exact = hand_type(game, hand, board);
        let compare = if game.is_shortdeck() {
            PQLHandType::compare::<true>
        } else {
            PQLHandType::compare::<false>
        };

        for ht in PQLHandType::ARR_ALL {
            assert_eq!(
                min_hand_type(game, hand, board, ht),
                !matches!(compare(exact, ht), Ordering::Less),
            );
        }

        TestResult::passed()
    }
}
