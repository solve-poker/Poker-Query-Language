use super::*;

#[pqlfn]
pub fn board_ranks(ctx: &PQLFnContext, street: PQLStreet) -> PQLRankSet {
    core::board_ranks(ctx.get_board(street))
}
