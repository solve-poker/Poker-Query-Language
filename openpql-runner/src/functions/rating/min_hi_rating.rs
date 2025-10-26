use super::*;

#[pqlfn]
pub fn min_hi_rating(
    ctx: &PQLFnContext,
    player: PQLPlayer,
    street: PQLStreet,
    rating: PQLHiRating,
) -> PQLBoolean {
    hi_rating(ctx, player, street) >= rating
}
