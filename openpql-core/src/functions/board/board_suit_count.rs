use crate::{PQLBoard, PQLCardCount, PQLCardSet, PQLSuitSet};

/// Counts the number of distinct suits present on the board.
pub fn board_suit_count(board: PQLBoard) -> PQLCardCount {
    PQLSuitSet::from(PQLCardSet::from(board)).count()
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use super::*;

    #[quickcheck]
    fn test_board_suit_count(board: PQLBoard) {
        let expected = {
            let mut suits = PQLSuitSet::default();
            for card in board.iter() {
                suits.set(card.suit);
            }
            suits.count()
        };

        assert_eq!(board_suit_count(board), expected);
    }
}
