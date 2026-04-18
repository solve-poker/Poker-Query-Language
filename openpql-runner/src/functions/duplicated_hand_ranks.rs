use super::*;

#[pqlfn]
pub fn duplicated_hand_ranks(
    ctx: &PQLFnContext,
    player: PQLPlayer,
) -> PQLRankSet {
    core::duplicated_hand_ranks(ctx.get_player_slice(player))
}
