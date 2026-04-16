use crate::{
    PQLBoard, PQLCard, PQLCardCount, functions::intersecting_hand_ranks,
};

pub fn hand_board_intersections(
    hand: &[PQLCard],
    board: PQLBoard,
) -> PQLCardCount {
    intersecting_hand_ranks(hand, board).count()
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use super::*;

    #[quickcheck]
    fn test_hand_board_intersections(hand: Vec<PQLCard>, board: PQLBoard) {
        assert_eq!(
            hand_board_intersections(&hand, board),
            intersecting_hand_ranks(&hand, board).count()
        );
    }
}
