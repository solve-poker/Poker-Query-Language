use super::*;

/// # Panics
/// won't panic since `ctx.n_players` ≥ 1
#[pqlfn]
pub fn max_hi_rating(ctx: &PQLFnContext, street: PQLStreet) -> PQLHiRating {
    (0..ctx.n_players)
        .map(|p| hi_rating(ctx, p.into(), street))
        .max()
        .unwrap()
}
