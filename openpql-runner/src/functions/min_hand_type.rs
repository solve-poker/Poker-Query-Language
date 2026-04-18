use super::*;

#[pqlfn]
pub fn min_hand_type(
    ctx: &PQLFnContext,
    player: PQLPlayer,
    street: PQLStreet,
    ht: PQLHandType,
) -> PQLBoolean {
    core::min_hand_type(
        ctx.game,
        ctx.get_player_slice(player),
        ctx.get_board(street),
        ht,
    )
}
