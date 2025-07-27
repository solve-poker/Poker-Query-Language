use super::*;
#[pqlfn(arg, rtn, eval)]
pub fn min_outs_to_hand_type(
    hand: &Hand,
    street: PQLStreet,
    hand_type: PQLHandType,
    min: PQLCardCount,
    args: (PQLGame, Board, DeadCards),
) -> PQLBoolean {
    let outs = outs_to_hand_type(hand, street, hand_type, args);

    outs >= min
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_min_outs_to_hand_type(hbg: HandBoardGame, ht: HandType) {
        let HandBoardGame {
            game,
            hand,
            board,
            dead,
            ..
        } = hbg;

        let ht = PQLHandType::from((ht, game));
        let outs =
            outs_to_hand_type(&hand, PQLStreet::Flop, ht, (game, board, dead));

        assert!(min_outs_to_hand_type(
            &hand,
            PQLStreet::Flop,
            ht,
            outs.saturating_sub(1),
            (game, board, dead)
        ));

        assert!(!min_outs_to_hand_type(
            &hand,
            PQLStreet::Flop,
            ht,
            outs + 1,
            (game, board, dead)
        ));
    }
}
