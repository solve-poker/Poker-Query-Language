use super::*;

#[pqlfn]
pub fn monotone_board(ctx: &PQLFnContext, street: PQLStreet) -> PQLBoolean {
    core::monotone_board(PQLBoard::from(ctx.get_board_slice(street)))
}
