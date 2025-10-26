use super::*;

#[pqlfn]
pub fn monotone_board(ctx: &PQLFnContext, street: PQLStreet) -> PQLBoolean {
    board_suit_count(ctx, street) == 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_monotone_board(
        ctx: TestPQLFnContext,
        street: PQLStreet,
    ) -> TestResult {
        let ctx = ctx.as_ctx();

        let expected = count_suits(ctx.get_board_slice(street)) == 1;
        let res = monotone_board(&ctx, street);

        TestResult::from_bool(res == expected)
    }
}
