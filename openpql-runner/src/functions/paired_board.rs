use super::*;

#[pqlfn]
pub fn paired_board(ctx: &PQLFnContext, street: PQLStreet) -> PQLBoolean {
    core::paired_board(ctx.get_board(street))
}
