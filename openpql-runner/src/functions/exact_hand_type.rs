use super::*;

#[pqlfn]
pub fn exact_hand_type(
    ctx: &PQLFnContext,
    player: PQLPlayer,
    street: PQLStreet,
    ht: PQLHandType,
) -> PQLBoolean {
    core::exact_hand_type(
        ctx.game,
        ctx.get_player_slice(player),
        ctx.get_board(street),
        ht,
    )
}
