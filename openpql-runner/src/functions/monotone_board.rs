use super::*;

#[pqlfn]
pub fn monotone_board(ctx: &PQLFnContext, street: PQLStreet) -> PQLBoolean {
    core::monotone_board(ctx.get_board(street))
}
