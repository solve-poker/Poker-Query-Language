use super::*;

#[pqlfn]
pub fn hi_rating(
    ctx: &PQLFnContext,
    player: PQLPlayer,
    street: PQLStreet,
) -> PQLHiRating {
    ctx.eval_current_rating(player, street)
}
