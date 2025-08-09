use super::*;

#[pqlfn(arg, rtn, eval)]
pub fn flushing_board(street: PQLStreet, board: Board) -> PQLBoolean {
    let c64: Card64 = (board, street).into();

    for s in Suit::ARR_ALL {
        if c64.count_by_suit(s) >= 3 {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    fn f(s: &str, st: PQLStreet) -> bool {
        flushing_board(st, board!(s))
    }

    #[test]
    fn test_flushing_board() {
        assert!(f("2s 3s 4s 5h 6d", PQLStreet::Flop));
        assert!(f("2s 3s 4s 5h 6d", PQLStreet::Turn));
        assert!(f("2s 3s 4s 5h 6d", PQLStreet::River));
        assert!(!f("2s 3s 4h 5h 6d", PQLStreet::Flop));
        assert!(!f("2s 3s 4h 5h 6d", PQLStreet::Turn));
        assert!(!f("2s 3s 4h 5h 6d", PQLStreet::River));

        assert!(f("2s 3s 4h 5s 6d", PQLStreet::Turn));
        assert!(f("2s 3s 4h 5s 6d", PQLStreet::River));
        assert!(!f("2s 3s 4h 5h 6d", PQLStreet::Turn));
        assert!(!f("2s 3s 4h 5h 6d", PQLStreet::River));

        assert!(f("2s 3h 4d 5s 6s", PQLStreet::River));
        assert!(!f("2s 3h 4d 5s 6h", PQLStreet::River));
    }
}
