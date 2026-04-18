use super::*;

#[pqlfn]
pub fn has_top_board_rank(
    ctx: &PQLFnContext,
    player: PQLPlayer,
    street: PQLStreet,
) -> PQLBoolean {
    core::has_top_board_rank(
        ctx.get_player_slice(player),
        PQLBoard::from(ctx.get_board_slice(street)),
    )
}
