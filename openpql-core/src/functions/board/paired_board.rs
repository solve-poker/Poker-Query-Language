use crate::{PQLBoard, PQLCardSet, PQLRankSet};

pub fn paired_board(board: PQLBoard) -> bool {
    let rank_count = PQLRankSet::from(PQLCardSet::from(board)).count();

    (rank_count as usize) < board.len()
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use super::*;

    #[quickcheck]
    fn test_paired_board(board: PQLBoard) {
        let mut ranks = board.iter().map(|c| c.rank).collect::<Vec<_>>();
        let n = ranks.len();

        ranks.sort_unstable();
        ranks.dedup();

        assert_eq!(paired_board(board), ranks.len() != n);
    }
}
