use crate::{PQLBoard, functions::board_suit_count};

/// Returns `true` when the board uses exactly two suits.
pub fn twotone_board(board: PQLBoard) -> bool {
    board_suit_count(board) == 2
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use super::*;

    #[quickcheck]
    fn test_twotone_board(board: PQLBoard) {
        assert_eq!(twotone_board(board), board_suit_count(board) == 2);
    }
}
