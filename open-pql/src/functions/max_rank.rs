use super::*;

#[pqlfn(arg, rtn, eval)]
pub const fn max_rank(ranks: PQLRankSet) -> Option<PQLRank> {
    ranks.max_rank()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_max_rank(ranks: Rank16) -> TestResult {
        TestResult::from_bool(ranks.max_rank() == max_rank(ranks))
    }
}
