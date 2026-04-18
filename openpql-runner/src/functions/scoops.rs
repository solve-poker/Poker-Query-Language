use super::*;

// TODO: optimize
#[pqlfn]
pub fn scoops(ctx: &PQLFnContext, player: PQLPlayer) -> PQLBoolean {
    best_hi_rating(ctx, player, PQLStreet::River) && !ties_hi(ctx, player)
}
