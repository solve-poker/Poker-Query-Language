use super::*;

#[pqlfn]
pub const fn rank_count(ranks: PQLRankSet) -> PQLCardCount {
    ranks.count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_rank_count(ranks: PQLRankSet) -> TestResult {
        TestResult::from_bool(ranks.count() == rank_count(ranks))
    }
}
