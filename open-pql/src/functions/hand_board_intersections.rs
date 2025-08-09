use super::*;
#[pqlfn(arg, rtn, eval)]
pub fn hand_board_intersections(
    hand: &Hand,
    street: PQLStreet,
    board: Board,
) -> PQLCardCount {
    rank_count(intersecting_hand_ranks(hand, street, board))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_hand_board_intersections(
        cards: (Card, Card, Card, Card),
        board: Board,
        street: PQLStreet,
    ) -> TestResult {
        let hand = [cards.0, cards.1, cards.2, cards.3];

        let i = hand_board_intersections(&hand, street, board);

        TestResult::from_bool(
            i == rank_count(intersecting_hand_ranks(&hand, street, board)),
        )
    }
}
