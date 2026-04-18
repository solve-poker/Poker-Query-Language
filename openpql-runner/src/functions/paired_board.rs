use super::*;

#[pqlfn]
pub fn paired_board(ctx: &PQLFnContext, street: PQLStreet) -> PQLBoolean {
    core::paired_board(PQLBoard::from(ctx.get_board_slice(street)))
}
