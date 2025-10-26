use super::*;

#[pqlfn]
pub fn pocket_pair(ctx: &PQLFnContext, player: PQLPlayer) -> PQLBoolean {
    !duplicated_hand_ranks(ctx, player).is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[quickcheck]
    fn test_pocket_pair(ctx: TestPQLFnContext) {
        let ctx = ctx.as_ctx();

        for player in PQLPlayer::iter(ctx.n_players) {
            let c64 = ctx.get_c64_player(player);

            let mut has_pair = false;

            for &rank in PQLRank::all::<false>() {
                if c64.count_by_rank(rank) > 1 {
                    has_pair = true;
                }
            }

            assert_eq!(has_pair, pocket_pair(&ctx, player));
        }
    }
}
