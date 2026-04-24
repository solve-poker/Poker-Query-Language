use crate::{
    PQLBoard, PQLCard,
    functions::{board_ranks, hand_ranks},
};

/// Returns `true` when the hand contains the highest rank on the board.
pub fn has_top_board_rank(hand: &[PQLCard], board: PQLBoard) -> bool {
    board_ranks(board)
        .max_rank()
        .is_some_and(|top| hand_ranks(hand).contains_rank(top))
}

#[cfg(test)]
mod tests {
    use openpql_prelude::{board, cards};
    use quickcheck_macros::quickcheck;

    use super::*;
    use crate::PQLRankSet;

    #[test]
    fn test_has_top_board_rank_cases() {
        assert!(has_top_board_rank(&cards!("Ah 5d"), board!("Ks Qh As")));
        assert!(!has_top_board_rank(&cards!("Kh 5d"), board!("Ks Qh As")));
        assert!(!has_top_board_rank(&cards!("Ah 5d"), PQLBoard::default()));
    }

    #[quickcheck]
    fn test_has_top_board_rank(hand: Vec<PQLCard>, board: PQLBoard) {
        let top = board_ranks(board)
            .max_rank()
            .map_or_else(PQLRankSet::default, PQLRankSet::from);

        let expected = !(hand_ranks(&hand) & top).is_empty();

        assert_eq!(has_top_board_rank(&hand, board), expected);
    }
}
