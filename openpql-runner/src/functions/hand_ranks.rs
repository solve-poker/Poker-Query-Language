use super::*;
#[pqlfn]
pub fn hand_ranks(ctx: &PQLFnContext, player: PQLPlayer) -> PQLRankSet {
    core::hand_ranks(ctx.get_player_slice(player))
}
