use super::*;

#[pqlfn]
pub fn rainbow_board(ctx: &PQLFnContext, street: PQLStreet) -> PQLBoolean {
    core::rainbow_board(ctx.get_board(street))
}
