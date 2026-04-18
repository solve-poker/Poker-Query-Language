use super::*;

#[pqlfn]
pub const fn max_rank(ranks: PQLRankSet) -> Result<PQLRank, RuntimeError> {
    match ranks.max_rank() {
        Some(r) => Ok(r),
        None => Err(RuntimeError::ValueRetrievalFailed(PQLType::RANK)),
    }
}
