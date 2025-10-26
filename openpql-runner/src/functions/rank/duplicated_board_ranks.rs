use super::*;

#[pqlfn]
pub fn duplicated_board_ranks(
    ctx: &PQLFnContext,
    street: PQLStreet,
) -> PQLRankSet {
    let board = ctx.get_c64_board(street);

    let [_, more_than_two, _, _] = rank_cardinality(board);

    more_than_two
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_duplicated_board_ranks(
        ctx: TestPQLFnContext,
        street: PQLStreet,
    ) -> TestResult {
        let ctx = ctx.as_ctx();

        let c64: PQLCardSet = ctx.get_c64_board(street);
        let mut ranks = PQLRankSet::default();

        for &rank in PQLRank::all::<false>() {
            if c64.count_by_rank(rank) > 1 {
                ranks.set(rank);
            }
        }

        TestResult::from_bool(ranks == duplicated_board_ranks(&ctx, street))
    }
}
