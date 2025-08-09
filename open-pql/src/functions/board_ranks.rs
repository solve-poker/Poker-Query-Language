use super::*;
#[pqlfn(arg, rtn, eval)]
pub fn board_ranks(street: PQLStreet, board: Board) -> PQLRankSet {
    Card64::from((board, street)).into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_board_ranks(board: Board) -> TestResult {
        let cards = board.to_vec();
        let flop = Rank16::from(cards[0..3].as_ref());
        let turn = Rank16::from(cards[0..4].as_ref());
        let river = Rank16::from(cards[0..5].as_ref());

        TestResult::from_bool(
            flop == board_ranks(PQLStreet::Flop, board)
                && turn == board_ranks(PQLStreet::Turn, board)
                && river == board_ranks(PQLStreet::River, board),
        )
    }
}
