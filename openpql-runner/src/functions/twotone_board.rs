use super::*;

#[pqlfn]
pub fn twotone_board(ctx: &PQLFnContext, street: PQLStreet) -> PQLBoolean {
    core::twotone_board(PQLBoard::from(ctx.get_board_slice(street)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_twotone_board(
        ctx: TestPQLFnContext,
        street: PQLStreet,
    ) -> TestResult {
        let ctx = ctx.as_ctx();

        let expected = count_suits(ctx.get_board_slice(street)) == 2;
        let res = twotone_board(&ctx, street);

        TestResult::from_bool(res == expected)
    }
}
