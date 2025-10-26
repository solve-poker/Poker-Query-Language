use super::*;

#[pqlfn]
pub fn hi_rating(
    ctx: &PQLFnContext,
    player: PQLPlayer,
    street: PQLStreet,
) -> PQLHiRating {
    ctx.game
        .eval_rating(ctx.get_c64_player(player), ctx.get_c64_board(street))
}
