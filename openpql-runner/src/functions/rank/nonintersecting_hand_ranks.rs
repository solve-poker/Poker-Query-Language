use super::*;

#[pqlfn]
pub fn nonintersecting_hand_ranks(
    ctx: &PQLFnContext,
    player: PQLPlayer,
    street: PQLStreet,
) -> PQLRankSet {
    hand_ranks(ctx, player) & !board_ranks(ctx, street)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_nonintersecting_hand_ranks(
        ctx: TestPQLFnContext,
        street: PQLStreet,
    ) {
        let ctx = ctx.as_ctx();

        for player in PQLPlayer::iter(ctx.n_players) {
            let p = hand_ranks(&ctx, player);
            let b = board_ranks(&ctx, street);

            assert_eq!(
                nonintersecting_hand_ranks(&ctx, player, street),
                p & !b
            );
        }
    }
}
