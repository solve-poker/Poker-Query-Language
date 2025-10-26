use super::*;

#[pqlfn]
pub fn board_suit_count(ctx: &PQLFnContext, street: PQLStreet) -> PQLCardCount {
    PQLSuitSet::from(ctx.get_c64_board(street)).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_board_suit_count(
        ctx: TestPQLFnContext,
        street: PQLStreet,
    ) -> TestResult {
        let ctx = ctx.as_ctx();

        let expected = count_suits(ctx.get_board_slice(street));
        let res = board_suit_count(&ctx, street);

        TestResult::from_bool(res == expected)
    }
}
