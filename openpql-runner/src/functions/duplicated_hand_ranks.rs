use super::*;

#[pqlfn]
pub fn duplicated_hand_ranks(
    ctx: &PQLFnContext,
    player: PQLPlayer,
) -> PQLRankSet {
    core::duplicated_hand_ranks(ctx.get_player_slice(player))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_duplicated_hand_ranks(ctx: TestPQLFnContext) {
        let ctx = ctx.as_ctx();

        for player in PQLPlayer::iter(ctx.n_players) {
            let c64: PQLCardSet = ctx.get_c64_player(player);
            let mut ranks = PQLRankSet::default();

            for &rank in PQLRank::all::<false>() {
                if c64.count_by_rank(rank) > 1 {
                    ranks.set(rank);
                }
            }

            assert_eq!(ranks, duplicated_hand_ranks(&ctx, player));
        }
    }
}
