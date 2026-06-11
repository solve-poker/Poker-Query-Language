use crate::{PQLBoard, PQLCard, PQLCardSet, PQLGame, PQLHiRating};

/// Returns the high-hand rating of the hand given the board.
pub fn hi_rating(game: PQLGame, hand: &[PQLCard], board: PQLBoard) -> PQLHiRating {
    game.eval_rating(PQLCardSet::from(hand), PQLCardSet::from(board))
}

#[cfg(test)]
mod tests {
    use openpql_prelude::{board, cards};

    use super::*;

    #[test]
    fn test_hi_rating_cases() {
        let r1 = hi_rating(PQLGame::Holdem, &cards!("As Ah"), board!("Ad 7c 2d Jh 9s"));
        let r2 = hi_rating(PQLGame::Holdem, &cards!("Ks Kh"), board!("Ad 7c 2d Jh 9s"));

        assert!(r1 > r2);
    }
}
