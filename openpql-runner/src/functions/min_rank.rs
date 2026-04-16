use super::*;

#[pqlfn]
pub const fn min_rank(
    _ctx: &PQLFnContext,
    ranks: PQLRankSet,
) -> Result<PQLRank, RuntimeError> {
    match ranks.min_rank() {
        Some(r) => Ok(r),
        None => Err(RuntimeError::ValueRetrievalFailed(PQLType::RANK)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_min_rank(ctx: TestPQLFnContext, ranks: PQLRankSet) -> TestResult {
        let ctx = ctx.as_ctx();

        let expected = ranks
            .min_rank()
            .ok_or(RuntimeError::ValueRetrievalFailed(PQLType::RANK));

        TestResult::from_bool(min_rank(&ctx, ranks) == expected)
    }
}
