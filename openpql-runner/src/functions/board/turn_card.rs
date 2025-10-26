use super::*;

#[pqlfn]
pub fn turn_card(ctx: &PQLFnContext) -> PQLCard {
    ctx.get_board_slice(PQLStreet::River)[PQLBoard::IDX_TURN]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_turn_card(ctx: TestPQLFnContext) -> TestResult {
        const IDX_TURN: usize = 3;

        let ctx = ctx.as_ctx();

        let expected = ctx.get_board_slice(PQLStreet::River)[IDX_TURN];
        let res = turn_card(&ctx);

        TestResult::from_bool(res == expected)
    }
}
