use super::*;

#[pqlfn]
pub fn overpair(ctx: &PQLFnContext, player: PQLPlayer, street: PQLStreet) -> PQLBoolean {
    core::overpair(ctx.get_player_slice(player), ctx.get_board(street))
}
