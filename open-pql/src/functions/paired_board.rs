use super::*;

#[pqlfn]
pub fn paired_board(street: PQLStreet, board: Board) -> PQLBoolean {
    let ranks = board_ranks(street, board);

    match street {
        PQLStreet::Flop => ranks.count() < 3,
        PQLStreet::Turn => ranks.count() < 4,
        PQLStreet::River => ranks.count() < 5,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_paired_board(street: PQLStreet, board: Board) -> TestResult {
        let c64: Card64 = (board, street).into();

        let paired = Rank::ARR_ALL.iter().any(|r| c64.count_by_rank(*r) > 1);

        TestResult::from_bool(paired_board(street, board) == paired)
    }
}
