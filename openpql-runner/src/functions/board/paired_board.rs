use super::*;

#[pqlfn]
pub fn paired_board(ctx: &PQLFnContext, street: PQLStreet) -> PQLBoolean {
    let rank_count = PQLRankSet::from(ctx.get_c64_board(street)).count();

    rank_count < street.board_card_count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_paired_board(
        ctx: TestPQLFnContext,
        street: PQLStreet,
    ) -> TestResult {
        let ctx = ctx.as_ctx();

        let n = ctx.get_board_slice(street).len();
        let mut ranks = ctx
            .get_board_slice(street)
            .iter()
            .copied()
            .map(|c| c.rank)
            .collect::<Vec<_>>();

        ranks.sort_unstable();
        ranks.dedup();

        let paired = ranks.len() != n;

        TestResult::from_bool(paired_board(&ctx, street) == paired)
    }
}
