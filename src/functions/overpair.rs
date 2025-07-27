use super::*;

#[pqlfn]
pub fn overpair(hand: &Hand, street: PQLStreet, board: Board) -> PQLBoolean {
    max_rank(duplicated_hand_ranks(hand, street))
        .map_or(false, |r| r > max_rank_of_board(street, board))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[quickcheck]
    fn test_overpair(hbg: HandBoardGame) -> TestResult {
        let HandBoardGame {
            hand,
            board,
            street,
            ..
        } = hbg;

        let pocket_rank =
            max_rank(duplicated_hand_ranks(hand.as_ref(), street));
        let board_rank = max_rank(board_ranks(street, board)).unwrap();

        TestResult::from_bool(pocket_rank.map_or_else(
            || !overpair(hand.as_ref(), street, board),
            |r| (r > board_rank) == overpair(hand.as_ref(), street, board),
        ))
    }
}
