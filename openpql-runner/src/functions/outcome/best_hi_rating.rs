use super::*;

// TODO: optimize
#[pqlfn]
pub fn best_hi_rating(
    ctx: &PQLFnContext,
    player: PQLPlayer,
    street: PQLStreet,
) -> PQLBoolean {
    let player_rating = hi_rating(ctx, player, street);

    (0..ctx.n_players)
        .all(|i| player_rating >= hi_rating(ctx, i.into(), street))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_best_hi_rating(ctx: TestPQLFnContext, street: PQLStreet) {
        let ctx = ctx.as_ctx();
        let n = ctx.n_players;

        let max_rating = max_hi_rating(&ctx, street);

        let winners = (0..n)
            .filter(|&p| hi_rating(&ctx, p.into(), street) == max_rating)
            .collect::<Vec<_>>();

        for i in 0..n {
            let player = i.into();

            assert_eq!(
                best_hi_rating(&ctx, player, street),
                winners.contains(&i),
            );
        }
    }
}
