use crate::{PQLBoard, PQLCardSet, PQLRankSet, util};

/// Returns the set of ranks that appear at least twice on the board.
pub fn duplicated_board_ranks(board: PQLBoard) -> PQLRankSet {
    let [_, has2, _, _] = util::rank_cardinality(PQLCardSet::from(board));

    has2
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use super::*;
    use crate::PQLRank;

    #[quickcheck]
    fn test_duplicated_board_ranks(board: PQLBoard) {
        let c64 = PQLCardSet::from(board);
        let mut expected = PQLRankSet::default();
        for &rank in PQLRank::all::<false>() {
            if c64.count_by_rank(rank) > 1 {
                expected.set(rank);
            }
        }

        assert_eq!(duplicated_board_ranks(board), expected);
    }
}
