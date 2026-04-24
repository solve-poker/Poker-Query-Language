use crate::{PQLBoard, PQLCardCount, PQLRankSet, functions::board_ranks};

/// Counts the distinct low ranks on the board (A, 2..8).
///
/// Duplicated ranks are collapsed; e.g. `AsAhAdAcKs` returns `1`.
pub fn board_lo_card_count(board: PQLBoard) -> PQLCardCount {
    (board_ranks(board) & PQLRankSet::ALL_LO).count()
}

#[cfg(test)]
mod tests {
    use openpql_prelude::cards;
    use quickcheck_macros::quickcheck;

    use super::*;
    use crate::PQLRank;

    const fn is_lo(rank: PQLRank) -> bool {
        (rank as u8) <= PQLRank::R8 as u8 || matches!(rank, PQLRank::RA)
    }

    #[test]
    fn test_board_lo_card_count_cases() {
        let f = |s| board_lo_card_count(PQLBoard::from_slice(&cards!(s)));

        assert_eq!(f("Ac 2d 3h"), 3);
        assert_eq!(f("Ac Ad 3h"), 2);
        assert_eq!(f("Ac Ad Ah As Kc"), 1);
        assert_eq!(f("Kc Qd Jh"), 0);
        assert_eq!(f("9c Tc Jc"), 0);
        assert_eq!(f("Ac 8d Kh 9s 7c"), 3);
    }

    #[quickcheck]
    fn test_board_lo_card_count(board: PQLBoard) {
        let expected = board
            .iter()
            .map(|c| c.rank)
            .filter(|&r| is_lo(r))
            .collect::<PQLRankSet>()
            .count();

        assert_eq!(board_lo_card_count(board), expected);
    }
}
