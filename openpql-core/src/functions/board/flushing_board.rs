use crate::{PQLBoard, PQLCardSet, PQLSuit};

pub fn flushing_board(board: PQLBoard) -> bool {
    let board = PQLCardSet::from(board);

    board.count_by_suit(PQLSuit::S) >= 3
        || board.count_by_suit(PQLSuit::H) >= 3
        || board.count_by_suit(PQLSuit::D) >= 3
        || board.count_by_suit(PQLSuit::C) >= 3
}

#[cfg(test)]
mod tests {
    use openpql_prelude::cards;
    use quickcheck_macros::quickcheck;

    use super::*;

    fn f(s: &str) -> bool {
        flushing_board(PQLBoard::from_slice(&cards!(s)))
    }

    #[test]
    fn test_flushing_board_cases() {
        assert!(f("2s 3s 4s"));
        assert!(f("2s 3s 4s 5h"));
        assert!(f("2s 3s 4s 5h 6d"));
        assert!(!f("2s 3s 4h"));
        assert!(!f("2s 3s 4h 5h"));
        assert!(f("2s 3s 4h 5s"));
        assert!(f("2s 3h 4d 5s 6s"));
        assert!(!f("2s 3h 4d 5s 6h"));
    }

    #[quickcheck]
    fn test_flushing_board(board: PQLBoard) {
        let cs = PQLCardSet::from(board);
        let expected =
            PQLSuit::ARR_ALL.iter().any(|&s| cs.count_by_suit(s) >= 3);

        assert_eq!(flushing_board(board), expected);
    }
}
