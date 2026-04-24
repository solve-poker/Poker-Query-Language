use crate::{PQLBoard, functions::board_suit_count};

/// Returns `true` when every board card shares a single suit.
pub fn monotone_board(board: PQLBoard) -> bool {
    board_suit_count(board) == 1
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use super::*;

    #[quickcheck]
    fn test_monotone_board(board: PQLBoard) {
        assert_eq!(monotone_board(board), board_suit_count(board) == 1);
    }
}
