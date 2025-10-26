use super::*;

#[pqlfn]
pub fn river_card(ctx: &PQLFnContext) -> PQLCard {
    ctx.get_board_slice(PQLStreet::River)[PQLBoard::IDX_RIVER]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_river_card(ctx: TestPQLFnContext) -> TestResult {
        const IDX_RIVER: usize = 4;

        let ctx = ctx.as_ctx();

        let expected = ctx.get_board_slice(PQLStreet::River)[IDX_RIVER];
        let res = river_card(&ctx);

        TestResult::from_bool(res == expected)
    }
}
