use super::*;
/// # Panics
/// board is always non-empty
#[pqlfn(arg, rtn, eval)]
pub fn has_second_board_rank(
    hand: &Hand,
    street: PQLStreet,
    board: Board,
) -> PQLBoolean {
    let mut board: Rank16 = board_ranks(street, board);

    let top_rank = board.max_rank().unwrap();

    board.unset(top_rank);

    match board.max_rank() {
        Some(second_rank) => {
            for c in hand {
                if c.r == second_rank {
                    return true;
                }
            }

            false
        }
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_has_second_board_rank(hbg: HandBoardGame) -> TestResult {
        let HandBoardGame {
            hand,
            board,
            street,
            ..
        } = hbg;

        let second = nth_rank(2, board_ranks(street, board));
        let has_second = hand.iter().any(|c| Some(c.r) == second);

        TestResult::from_bool(
            has_second == has_second_board_rank(hand.as_ref(), street, board),
        )
    }

    #[test]
    fn test_has_second_board_rank_none() {
        let board = board!("AsAhAcAdKs");
        let hand = cards!("2s2h");

        assert!(!has_second_board_rank(&hand, PQLStreet::Flop, board,));
    }
}
