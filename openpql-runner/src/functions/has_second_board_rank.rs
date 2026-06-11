use super::*;

#[pqlfn]
pub fn has_second_board_rank(
    ctx: &PQLFnContext,
    player: PQLPlayer,
    street: PQLStreet,
) -> PQLBoolean {
    core::has_second_board_rank(ctx.get_player_slice(player), ctx.get_board(street))
}
