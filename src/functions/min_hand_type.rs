use super::*;
#[pqlfn]
pub fn min_hand_type(
    hand: &Hand,
    street: PQLStreet,
    kind: PQLHandType,
    (game, board): (PQLGame, Board),
) -> PQLBoolean {
    hand_type(hand, street, (game, board)) >= kind
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_min_hand_type(hbg: HandBoardGame) -> TestResult {
        let HandBoardGame {
            street,
            game,
            hand,
            board,
            ..
        } = hbg;

        let exact_ht = hand_type(&hand, street, (game, board));

        for ht in HandType::ARR_ALL {
            let ht = (ht, game).into();
            assert_eq!(
                exact_ht >= ht,
                min_hand_type(&hand, street, ht, (game, board))
            );
        }

        TestResult::passed()
    }
}
