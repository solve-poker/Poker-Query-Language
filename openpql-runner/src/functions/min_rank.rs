use super::*;

#[pqlfn]
pub const fn min_rank(_ctx: &PQLFnContext, ranks: PQLRankSet) -> Result<PQLRank, RuntimeError> {
    match ranks.min_rank() {
        Some(r) => Ok(r),
        None => Err(RuntimeError::ValueRetrievalFailed(PQLType::RANK)),
    }
}
