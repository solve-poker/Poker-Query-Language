use crate::{PQLBoard, PQLCard, PQLCardSet, PQLFlopHandCategory, PQLGame};

/// Classifies a hand's relationship to the flop (top pair, overpair, ...).
pub fn flop_hand_category(game: PQLGame, hand: &[PQLCard], board: PQLBoard) -> PQLFlopHandCategory {
    game.eval_flop_category(PQLCardSet::from(hand), board)
}

#[cfg(test)]
mod tests {
    use openpql_prelude::{board, cards};

    use super::*;

    #[test]
    fn test_flop_hand_category_cases() {
        assert_eq!(
            flop_hand_category(PQLGame::Holdem, &cards!("7c 8c"), board!("7s 8h Tc"),),
            PQLFlopHandCategory::BottomTwo,
        );
        assert_eq!(
            flop_hand_category(PQLGame::Omaha, &cards!("7c 8c 2s 3s"), board!("7s 8h Tc"),),
            PQLFlopHandCategory::BottomTwo,
        );
    }
}
