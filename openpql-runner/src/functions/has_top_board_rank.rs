use super::*;

#[pqlfn]
pub fn has_top_board_rank(
    ctx: &PQLFnContext,
    player: PQLPlayer,
    street: PQLStreet,
) -> PQLBoolean {
    core::has_top_board_rank(
        ctx.get_player_slice(player),
        PQLBoard::from(ctx.get_board_slice(street)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_has_top_board_rank(ctx: TestPQLFnContext, street: PQLStreet) {
        let ctx = ctx.as_ctx();
        let top_rank = board_ranks(&ctx, street)
            .max_rank()
            .map_or_else(PQLRankSet::default, PQLRankSet::from);

        for player in PQLPlayer::iter(ctx.n_players) {
            let has_top_rank = !(intersecting_hand_ranks(&ctx, player, street)
                & top_rank)
                .is_empty();

            assert_eq!(has_top_board_rank(&ctx, player, street), has_top_rank);
        }
    }
}
