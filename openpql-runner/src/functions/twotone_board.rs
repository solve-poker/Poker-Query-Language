use super::*;

#[pqlfn]
pub fn twotone_board(ctx: &PQLFnContext, street: PQLStreet) -> PQLBoolean {
    core::twotone_board(PQLBoard::from(ctx.get_board_slice(street)))
}
