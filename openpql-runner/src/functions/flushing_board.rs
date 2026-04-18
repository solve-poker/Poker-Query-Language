use super::*;

#[pqlfn]
pub fn flushing_board(ctx: &PQLFnContext, street: PQLStreet) -> PQLBoolean {
    core::flushing_board(PQLBoard::from(ctx.get_board_slice(street)))
}
