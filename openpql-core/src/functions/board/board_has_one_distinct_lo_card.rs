use crate::{PQLBoard, functions::board_lo_card_count};

pub fn board_has_one_distinct_lo_card(board: PQLBoard) -> bool {
    board_lo_card_count(board) == 1
}

#[cfg(test)]
mod tests {
    use openpql_prelude::cards;

    use super::*;

    fn f(s: &str) -> bool {
        board_has_one_distinct_lo_card(PQLBoard::from_slice(&cards!(s)))
    }

    #[test]
    fn test_board_has_one_distinct_lo_card() {
        assert!(f("Ac Kd Qh"));
        assert!(f("Ac Ad Kh"));
        assert!(f("2c 2d 2h"));
        assert!(!f("Ac 2d Kh"));
        assert!(!f("Kc Qd Jh"));
        assert!(!f("Ac 2d 3h"));
    }
}
