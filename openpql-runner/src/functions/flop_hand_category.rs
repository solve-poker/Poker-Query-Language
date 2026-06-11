use super::*;

#[pqlfn]
pub fn flop_hand_category(ctx: &PQLFnContext, player: PQLPlayer) -> PQLFlopHandCategory {
    core::flop_hand_category(
        ctx.game,
        ctx.get_player_slice(player),
        ctx.get_board(PQLStreet::Flop),
    )
}
