use super::*;

#[pqlfn]
pub fn hi_rating(
    ctx: &PQLFnContext,
    player: PQLPlayer,
    street: PQLStreet,
) -> PQLHiRating {
    core::hi_rating(
        ctx.game,
        ctx.get_player_slice(player),
        PQLBoard::from(ctx.get_board_slice(street)),
    )
}
