use super::*;

#[pqlfn]
pub fn rainbow_board(street: PQLStreet, board: Board) -> PQLBoolean {
    match street {
        PQLStreet::Flop => board_suit_count(street, board) == 3,
        PQLStreet::Turn => board_suit_count(street, board) == 4,
        PQLStreet::River => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_rainbow_board_flop(board: Board) -> TestResult {
        let rainbow = board_suit_count(PQLStreet::Flop, board) == 3;

        TestResult::from_bool(rainbow_board(PQLStreet::Flop, board) == rainbow)
    }

    #[quickcheck]
    fn test_rainbow_board_turn(board: Board) -> TestResult {
        let rainbow = board_suit_count(PQLStreet::Turn, board) == 4;

        TestResult::from_bool(rainbow_board(PQLStreet::Turn, board) == rainbow)
    }

    #[quickcheck]
    fn test_rainbow_board_river(board: Board) -> TestResult {
        TestResult::from_bool(!rainbow_board(PQLStreet::River, board))
    }
}
