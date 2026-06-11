use super::*;
#[pqlfn]
pub fn hand_board_intersections(
    ctx: &PQLFnContext,
    player: PQLPlayer,
    street: PQLStreet,
) -> PQLCardCount {
    core::hand_board_intersections(ctx.get_player_slice(player), ctx.get_board(street))
}
