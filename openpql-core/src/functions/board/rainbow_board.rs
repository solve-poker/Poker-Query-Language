use crate::{PQLBoard, functions::board_suit_count};

/// Returns `true` when every board card has a distinct suit.
///
/// Always `false` on the river since a 5-card board cannot be rainbow.
pub fn rainbow_board(board: PQLBoard) -> bool {
    let len = board.len();

    len > 0 && len < 5 && board_suit_count(board) as usize == len
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use super::*;

    #[quickcheck]
    fn test_rainbow_board(board: PQLBoard) {
        let len = board.len();
        let expected = len > 0 && len < 5 && board_suit_count(board) as usize == len;

        assert_eq!(rainbow_board(board), expected);
    }
}
