use crate::{PQLBoard, functions::board_lo_card_count};

pub fn board_allows_made_lo(board: PQLBoard) -> bool {
    board_lo_card_count(board) >= 3
}

#[cfg(test)]
mod tests {
    use openpql_prelude::cards;

    use super::*;

    fn f(s: &str) -> bool {
        board_allows_made_lo(PQLBoard::from_slice(&cards!(s)))
    }

    #[test]
    fn test_board_allows_made_lo() {
        assert!(f("Ac 2d 3h"));
        assert!(f("8c 2d 3h"));
        assert!(!f("Ac Ad 3h"));
        assert!(!f("Kc Qd Jh"));
        assert!(!f("9c Tc Jc"));
        assert!(!f("Ac 2d"));
        assert!(f("Ac 8d Kh 9s 7c"));
        assert!(f("Ac 2d 3h 4c Kd"));
    }
}
