use super::*;

// TODO: optimize
#[pqlfn]
pub fn scoops(ctx: &PQLFnContext, player: PQLPlayer) -> PQLBoolean {
    best_hi_rating(ctx, player, PQLStreet::River) && !ties_hi(ctx, player)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_scoops(ctx: TestPQLFnContext) {
        let ctx = ctx.as_ctx();
        let street = PQLStreet::River;

        let max_rating = max_hi_rating(&ctx, street);

        let winners = PQLPlayer::iter(ctx.n_players)
            .filter(|&p| hi_rating(&ctx, p, street) == max_rating)
            .collect::<Vec<_>>();

        for player in PQLPlayer::iter(ctx.n_players) {
            assert_eq!(scoops(&ctx, player), winners == [player]);
        }
    }
}
