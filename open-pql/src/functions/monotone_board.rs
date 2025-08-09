use super::*;

#[pqlfn]
pub fn monotone_board(street: PQLStreet, board: Board) -> PQLBoolean {
    board_suit_count(street, board) == 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_monotone_board(street: PQLStreet, board: Board) -> TestResult {
        let monotone = board_suit_count(street, board) == 1;

        TestResult::from_bool(monotone_board(street, board) == monotone)
    }
}
