use crate::{
    PQLBoard, PQLCard, PQLRankSet,
    functions::{board_ranks, hand_ranks},
};

// note: player \ board
pub fn nonintersecting_hand_ranks(
    hand: &[PQLCard],
    board: PQLBoard,
) -> PQLRankSet {
    hand_ranks(hand) & !board_ranks(board)
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use super::*;

    #[quickcheck]
    fn test_nonintersecting_hand_ranks(hand: Vec<PQLCard>, board: PQLBoard) {
        assert_eq!(
            nonintersecting_hand_ranks(&hand, board),
            hand_ranks(&hand) & !board_ranks(board)
        );
    }
}
