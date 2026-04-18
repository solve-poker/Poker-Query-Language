use super::*;
#[pqlfn]
pub fn board_ranks(ctx: &PQLFnContext, street: PQLStreet) -> PQLRankSet {
    core::board_ranks(PQLBoard::from(ctx.get_board_slice(street)))
}
