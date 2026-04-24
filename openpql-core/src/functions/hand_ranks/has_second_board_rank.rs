use crate::{
    PQLBoard, PQLCard,
    functions::{board_ranks, hand_ranks},
};

/// Returns `true` when the hand contains the board's second-highest rank.
pub fn has_second_board_rank(hand: &[PQLCard], board: PQLBoard) -> bool {
    board_ranks(board)
        .nth_rank(2)
        .is_some_and(|second| hand_ranks(hand).contains_rank(second))
}

#[cfg(test)]
mod tests {
    use openpql_prelude::{board, cards};
    use quickcheck_macros::quickcheck;

    use super::*;
    use crate::PQLRankSet;

    #[test]
    fn test_has_second_board_rank_cases() {
        assert!(has_second_board_rank(&cards!("Kh 5d"), board!("Ks Qh As")));
        assert!(!has_second_board_rank(&cards!("Qh 5d"), board!("Ks Qh As")));
        assert!(!has_second_board_rank(
            &cards!("Ah 5d"),
            PQLBoard::default()
        ));
    }

    #[quickcheck]
    fn test_has_second_board_rank(hand: Vec<PQLCard>, board: PQLBoard) {
        let second = board_ranks(board)
            .nth_rank(2)
            .map_or_else(PQLRankSet::default, PQLRankSet::from);

        let expected = !(hand_ranks(&hand) & second).is_empty();

        assert_eq!(has_second_board_rank(&hand, board), expected);
    }
}
