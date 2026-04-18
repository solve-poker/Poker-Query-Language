use super::*;

#[pqlfn]
pub fn pocket_pair(ctx: &PQLFnContext, player: PQLPlayer) -> PQLBoolean {
    core::pocket_pair(ctx.get_player_slice(player))
}
