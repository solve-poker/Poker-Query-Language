use super::*;

#[pqlfn]
pub fn twotone_board(street: PQLStreet, board: Board) -> PQLBoolean {
    board_suit_count(street, board) == 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_twotone_board(street: PQLStreet, board: Board) -> TestResult {
        let twotone = board_suit_count(street, board) == 2;

        TestResult::from_bool(twotone_board(street, board) == twotone)
    }
}
