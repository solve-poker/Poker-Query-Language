use crate::{PQLBoard, functions::board_lo_card_count};

/// Returns `true` when exactly two distinct low ranks are present on the board.
pub fn board_has_two_distinct_lo_cards(board: PQLBoard) -> bool {
    board_lo_card_count(board) == 2
}

#[cfg(test)]
mod tests {
    use openpql_prelude::cards;

    use super::*;

    fn f(s: &str) -> bool {
        board_has_two_distinct_lo_cards(PQLBoard::from_slice(&cards!(s)))
    }

    #[test]
    fn test_board_has_two_distinct_lo_cards() {
        assert!(f("Ac 2d Kh"));
        assert!(f("Ac Ad 2h"));
        assert!(f("8c 2d 8s Qh"));
        assert!(!f("Ac Kd Qh"));
        assert!(!f("Ac 2d 3h"));
        assert!(!f("Kc Qd Jh"));
    }
}
