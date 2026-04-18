use super::*;

#[pqlfn]
pub const fn rank_count(ranks: PQLRankSet) -> PQLCardCount {
    ranks.count()
}
