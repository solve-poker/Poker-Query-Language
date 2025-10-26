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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_nth_rank(ranks: PQLRankSet) {
        for n in 0..PQLRank::N_RANKS {
            let expected = ranks
                .nth_rank(n)
                .ok_or(RuntimeError::ValueRetrievalFailed(PQLType::RANK));

            assert_eq!(expected, nth_rank(n, ranks));
        }
    }
}
