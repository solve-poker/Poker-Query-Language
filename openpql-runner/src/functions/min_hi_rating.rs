use super::*;

#[pqlfn]
pub fn min_hi_rating(
    ctx: &PQLFnContext,
    player: PQLPlayer,
    street: PQLStreet,
    rating: PQLHiRating,
) -> PQLBoolean {
    core::min_hi_rating(
        ctx.game,
        ctx.get_player_slice(player),
        PQLBoard::from(ctx.get_board_slice(street)),
        rating,
    )
}
