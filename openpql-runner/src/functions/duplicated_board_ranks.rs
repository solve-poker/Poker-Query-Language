use super::*;

#[pqlfn]
pub fn duplicated_board_ranks(
    ctx: &PQLFnContext,
    street: PQLStreet,
) -> PQLRankSet {
    core::duplicated_board_ranks(ctx.get_board(street))
}
