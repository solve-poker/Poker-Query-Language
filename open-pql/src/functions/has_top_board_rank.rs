use super::*;

#[pqlfn]
pub fn has_top_board_rank(
    hand: &Hand,
    street: PQLStreet,
    board: Board,
) -> PQLBoolean {
    let top_rank: Rank = max_rank_of_board(street, board);

    for c in hand {
        if c.rank == top_rank {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_has_top_board_rank(
        cards: (Card, Card, Card, Card),
        board: Board,
        street: PQLStreet,
    ) -> TestResult {
        let rs_board = board_ranks(street, board);

        let hand = [cards.0, cards.1, cards.2, cards.3];

        let top = rs_board.max_rank();

        let has_top =
            hand.into_iter().filter(|c| Some(c.rank) == top).count() > 0;

        TestResult::from_bool(
            has_top == has_top_board_rank(&hand, street, board),
        )
    }
}
