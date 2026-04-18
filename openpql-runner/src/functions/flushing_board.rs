use super::*;

#[pqlfn]
pub fn flushing_board(ctx: &PQLFnContext, street: PQLStreet) -> PQLBoolean {
    core::flushing_board(ctx.get_board(street))
}
