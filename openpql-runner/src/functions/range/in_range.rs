use super::*;

#[pqlfn]
pub fn in_range(
    ctx: &PQLFnContext,
    player: PQLPlayer,
    range: &PQLRange,
) -> PQLBoolean {
    range.is_satisfied(ctx.get_player_slice(player))
}
