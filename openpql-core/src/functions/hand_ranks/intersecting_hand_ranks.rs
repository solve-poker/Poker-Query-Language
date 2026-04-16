use crate::{
    PQLBoard, PQLCard, PQLRankSet,
    functions::{board_ranks, hand_ranks},
};

pub fn intersecting_hand_ranks(
    hand: &[PQLCard],
    board: PQLBoard,
) -> PQLRankSet {
    hand_ranks(hand) & board_ranks(board)
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use super::*;

    #[quickcheck]
    fn test_intersecting_hand_ranks(hand: Vec<PQLCard>, board: PQLBoard) {
        assert_eq!(
            intersecting_hand_ranks(&hand, board),
            hand_ranks(&hand) & board_ranks(board)
        );
    }
}
