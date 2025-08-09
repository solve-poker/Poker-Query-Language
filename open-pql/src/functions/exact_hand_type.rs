use super::*;

#[pqlfn(arg, rtn, eval)]
pub fn exact_hand_type(
    hand: &Hand,
    street: PQLStreet,
    kind: PQLHandType,
    (game, board): (PQLGame, Board),
) -> PQLBoolean {
    hand_type(hand, street, (game, board)) == kind
}

#[cfg(test)]
mod tests {
    use super::*;

    #[quickcheck]
    fn test_exact_hand_type(hbg: HandBoardGame) -> TestResult {
        let HandBoardGame {
            game,
            hand,
            board,
            street,
            ..
        } = hbg;

        let ht = hand_type(&hand, street, (game, board));

        TestResult::from_bool(exact_hand_type(&hand, street, ht, (game, board)))
    }
}
