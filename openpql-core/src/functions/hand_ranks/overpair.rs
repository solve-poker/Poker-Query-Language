use crate::{
    PQLBoard, PQLCard,
    functions::{board_ranks, duplicated_hand_ranks},
};

/// Returns `true` when the hand has a pocket pair higher than every board rank.
pub fn overpair(hand: &[PQLCard], board: PQLBoard) -> bool {
    let Some(top) = board_ranks(board).max_rank() else {
        return false;
    };

    duplicated_hand_ranks(hand)
        .max_rank()
        .is_some_and(|pair| pair > top)
}

#[cfg(test)]
mod tests {
    use openpql_prelude::{Rank, board, cards};
    use quickcheck_macros::quickcheck;

    use super::*;
    use crate::PQLCardSet;

    #[test]
    fn test_overpair_cases() {
        assert!(overpair(&cards!("Ah Ad"), board!("Ks Qh 2s")));
        assert!(!overpair(&cards!("Qh Qd"), board!("Ks Qh 2s")));
        assert!(!overpair(&cards!("Ah Kd"), board!("Ks Qh 2s")));
        assert!(!overpair(&cards!("Ah Ad"), PQLBoard::default()));
    }

    #[quickcheck]
    fn test_overpair(hand: Vec<PQLCard>, board: PQLBoard) {
        let c64 = PQLCardSet::from(hand.as_slice());
        let board_ranks: Vec<_> = board.iter().map(|c| c.rank).collect();

        let expected = Rank::all::<false>().iter().any(|&rank| {
            c64.count_by_rank(rank) > 1
                && !board_ranks.is_empty()
                && board_ranks.iter().all(|b| rank > *b)
        });

        assert_eq!(overpair(&hand, board), expected);
    }
}
