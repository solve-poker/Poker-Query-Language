use super::*;

#[pqlfn]
pub fn min_flop_hand_category(
    hand: &Hand,
    category: PQLFlopHandCategory,
    (game, flop): (PQLGame, Flop),
) -> PQLBoolean {
    flop_hand_category(hand, (game, flop)) >= category
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_min_flop_hand_category(
        hbg: HandBoardGame,
        category: FlopHandCategory,
    ) -> TestResult {
        let exact = flop_hand_category(&hbg.hand, (hbg.game, hbg.board.flop));
        let category = (category, hbg.game).into();

        let is_ge = exact >= category;

        TestResult::from_bool(
            min_flop_hand_category(
                &hbg.hand,
                category,
                (hbg.game, hbg.board.flop),
            ) == is_ge,
        )
    }
}
