use super::*;

#[pqlfn(arg, rtn, eval)]
pub fn board_suit_count(street: PQLStreet, board: Board) -> PQLCardCount {
    let c: Card64 = (board, street).into();
    let s4: Suit4 = c.into();

    s4.count()
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_board_suit_count(board: Board) -> TestResult {
        fn uniq_count(v: &[Card]) -> u8 {
            v.iter().map(|c| c.s).unique().count().to_le_bytes()[0]
        }

        let cs = board.to_vec();

        assert_eq!(
            uniq_count(&cs[0..3]),
            board_suit_count(PQLStreet::Flop, board)
        );

        assert_eq!(
            uniq_count(&cs[0..4]),
            board_suit_count(PQLStreet::Turn, board)
        );

        assert_eq!(
            uniq_count(&cs[0..5]),
            board_suit_count(PQLStreet::River, board)
        );

        TestResult::passed()
    }
}
