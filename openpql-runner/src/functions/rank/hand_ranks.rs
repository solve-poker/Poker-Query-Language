use super::*;
#[pqlfn]
pub fn hand_ranks(ctx: &PQLFnContext, player: PQLPlayer) -> PQLRankSet {
    PQLRankSet::from(ctx.get_c64_player(player))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_hand_ranks(ctx: TestPQLFnContext) {
        let ctx = ctx.as_ctx();

        for player in PQLPlayer::iter(ctx.n_players) {
            let res = hand_ranks(&ctx, player);
            // maybe use some alternative logic?
            let ranks = PQLRankSet::from(ctx.get_c64_player(player));

            assert_eq!(res, ranks);
        }
    }
}
