use super::*;

#[pqlfn(arg, rtn, eval)]
pub fn max_rank(ranks: PQLRankSet) -> Option<PQLRank> {
    ranks.max_rank().map(PQLRank::from)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_max_rank(ranks: Rank16) -> TestResult {
        TestResult::from_bool(
            ranks.max_rank() == max_rank(ranks).map(Rank::from),
        )
    }
}
