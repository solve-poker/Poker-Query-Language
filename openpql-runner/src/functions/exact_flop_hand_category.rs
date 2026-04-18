use super::*;

#[pqlfn]
pub fn exact_flop_hand_category(
    ctx: &PQLFnContext,
    player: PQLPlayer,
    category: PQLFlopHandCategory,
) -> PQLBoolean {
    core::exact_flop_hand_category(
        ctx.game,
        ctx.get_player_slice(player),
        ctx.get_board(PQLStreet::Flop),
        category,
    )
}
