use crate::{PQLBoard, PQLRankSet};

pub fn board_ranks(board: PQLBoard) -> PQLRankSet {
    let mut res = PQLRankSet::default();

    if let Some(flop) = board.flop {
        res.set(flop[0].rank);
        res.set(flop[1].rank);
        res.set(flop[2].rank);
    }

    if let Some(c) = board.turn {
        res.set(c.rank);
    }

    if let Some(c) = board.river {
        res.set(c.rank);
    }

    res
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use super::*;

    #[quickcheck]
    fn test_board_ranks(board: PQLBoard) {
        let res = board_ranks(board);

        for card in board.iter() {
            assert!(res.contains_rank(card.rank));
        }

        let expected: PQLRankSet = board.iter().map(|c| c.rank).collect();
        assert_eq!(res, expected);
    }
}
