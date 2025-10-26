use super::*;
#[pqlfn]
pub fn hand_board_intersections(
    ctx: &PQLFnContext,
    player: PQLPlayer,
    street: PQLStreet,
) -> PQLCardCount {
    intersecting_hand_ranks(ctx, player, street).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_hand_board_intersections(ctx: TestPQLFnContext, street: PQLStreet) {
        let ctx = ctx.as_ctx();

        for player in PQLPlayer::iter(ctx.n_players) {
            let p = hand_ranks(&ctx, player);
            let b = board_ranks(&ctx, street);

            assert_eq!(
                hand_board_intersections(&ctx, player, street),
                (p & b).count()
            );
        }
    }
}
