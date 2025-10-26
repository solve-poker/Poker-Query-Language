use super::*;

#[pqlfn]
pub fn flop_hand_category(
    ctx: &PQLFnContext,
    player: PQLPlayer,
) -> PQLFlopHandCategory {
    ctx.game
        .eval_flop_category(ctx.get_c64_player(player), ctx.get_flop().into())
}
