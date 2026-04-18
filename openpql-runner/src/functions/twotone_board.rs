use super::*;

#[pqlfn]
pub fn twotone_board(ctx: &PQLFnContext, street: PQLStreet) -> PQLBoolean {
    core::twotone_board(ctx.get_board(street))
}
