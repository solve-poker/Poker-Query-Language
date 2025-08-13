use super::*;

#[pqlfn(arg, rtn, eval)]
pub fn exact_flop_hand_category(
    hand: &Hand,
    category: PQLFlopHandCategory,
    (game, flop): (PQLGame, Flop),
) -> PQLBoolean {
    flop_hand_category(hand, (game, flop)) == category
}

#[cfg(test)]
mod tests {
    use super::*;

    #[quickcheck]
    fn test_exact_flop_hand_category(
        hbg: HandBoardGame,
        category: FlopHandCategory,
    ) -> TestResult {
        let category = (category, hbg.game).into();
        let exact =
            flop_hand_category(&hbg.hand, (hbg.game, hbg.board.flop.unwrap()));

        TestResult::from_bool(
            exact_flop_hand_category(
                &hbg.hand,
                exact,
                (hbg.game, hbg.board.flop.unwrap()),
            ) && ((category != exact)
                ^ exact_flop_hand_category(
                    &hbg.hand,
                    category,
                    (hbg.game, hbg.board.flop.unwrap()),
                )),
        )
    }
}
