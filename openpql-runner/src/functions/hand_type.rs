use super::*;

#[pqlfn]
pub fn hand_type(
    ctx: &PQLFnContext,
    player: PQLPlayer,
    street: PQLStreet,
) -> PQLHandType {
    core::hand_type(
        ctx.game,
        ctx.get_player_slice(player),
        ctx.get_board(street),
    )
}
