use super::*;
#[pqlfn]
pub fn board_ranks(ctx: &PQLFnContext, street: PQLStreet) -> PQLRankSet {
    PQLRankSet::from(ctx.get_c64_board(street))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_board_ranks(
        ctx: TestPQLFnContext,
        street: PQLStreet,
    ) -> TestResult {
        let ctx = ctx.as_ctx();

        let expected =
            PQLRankSet::from(get_ranks(ctx.get_board_slice(street)).as_slice());
        let res = board_ranks(&ctx, street);

        TestResult::from_bool(res == expected)
    }
}
