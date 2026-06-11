use super::*;

// TODO: optimize
// TODO: deadcards
#[pqlfn]
pub fn nut_hi_outs(ctx: &PQLFnContext, player: PQLPlayer, street: PQLStreet) -> PQLCardCount {
    core::nut_hi_outs(
        ctx.game,
        ctx.get_player_slice(player),
        ctx.get_board(street),
    )
}
