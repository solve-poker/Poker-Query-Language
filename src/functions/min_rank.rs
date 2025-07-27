use super::*;

#[pqlfn]
pub fn min_rank(ranks: PQLRankSet) -> Option<PQLRank> {
    ranks.min_rank().map(PQLRank::from)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_min_rank(ranks: Rank16) -> TestResult {
        TestResult::from_bool(
            ranks.min_rank() == min_rank(ranks).map(Rank::from),
        )
    }
}
