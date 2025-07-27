use super::*;

#[pqlfn(arg, rtn, eval)]
pub const fn rank_count(ranks: PQLRankSet) -> PQLCardCount {
    ranks.count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_rank_count(ranks: Rank16) -> TestResult {
        let count: u8 = rank_count(ranks);

        TestResult::from_bool(count == ranks.count())
    }
}
