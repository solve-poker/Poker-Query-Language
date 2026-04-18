use super::*;

#[pqlfn]
pub fn wins_hi(ctx: &PQLFnContext, player: PQLPlayer) -> PQLBoolean {
    best_hi_rating(ctx, player, PQLStreet::River)
}
