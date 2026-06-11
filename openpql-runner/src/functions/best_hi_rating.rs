use super::*;

// TODO: optimize
#[pqlfn]
pub fn best_hi_rating(ctx: &PQLFnContext, player: PQLPlayer, street: PQLStreet) -> PQLBoolean {
    let player_rating = hi_rating(ctx, player, street);

    (0..ctx.n_players).all(|i| player_rating >= hi_rating(ctx, i.into(), street))
}
