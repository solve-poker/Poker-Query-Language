use super::*;

#[pqlfn]
pub const fn max_rank(ranks: PQLRankSet) -> Result<PQLRank, RuntimeError> {
    match ranks.max_rank() {
        Some(r) => Ok(r),
        None => Err(RuntimeError::ValueRetrievalFailed(PQLType::RANK)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_max_rank(ranks: PQLRankSet) -> TestResult {
        let expected = ranks
            .max_rank()
            .ok_or(RuntimeError::ValueRetrievalFailed(PQLType::RANK));

        TestResult::from_bool(max_rank(ranks) == expected)
    }
}
