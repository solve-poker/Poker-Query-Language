use super::*;
#[pqlfn]
pub fn intersecting_hand_ranks(
    ctx: &PQLFnContext,
    player: PQLPlayer,
    street: PQLStreet,
) -> PQLRankSet {
    core::intersecting_hand_ranks(
        ctx.get_player_slice(player),
        ctx.get_board(street),
    )
}
