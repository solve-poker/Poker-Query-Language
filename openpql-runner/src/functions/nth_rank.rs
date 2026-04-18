use super::*;

#[pqlfn]
pub fn nth_rank(
    n: PQLCardCount,
    ranks: PQLRankSet,
) -> Result<PQLRank, RuntimeError> {
    ranks
        .nth_rank(n)
        .ok_or(RuntimeError::ValueRetrievalFailed(PQLType::RANK))
}
