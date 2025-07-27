use super::*;
#[pqlfn(arg, rtn, eval)]
pub fn intersecting_hand_ranks(
    hand: &Hand,
    street: PQLStreet,
    board: Board,
) -> PQLRankSet {
    let ranks_hand: Rank16 = hand_ranks(hand, street);
    let ranks_board: Rank16 = board_ranks(street, board);

    ranks_hand & ranks_board
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_intersecting_hand_ranks(
        cards: (Card, Card, Card, Card),
        board: Board,
        street: PQLStreet,
    ) -> TestResult {
        let hand = [cards.0, cards.1, cards.2, cards.3];

        let res = intersecting_hand_ranks(&hand, street, board);

        let ranks_hand: Rank16 = hand_ranks(&hand, street);
        let ranks_board: Rank16 = board_ranks(street, board);

        let intersection = ranks_hand & ranks_board;

        TestResult::from_bool(intersection == res)
    }
}
