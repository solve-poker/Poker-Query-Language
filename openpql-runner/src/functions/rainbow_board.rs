use super::*;

#[pqlfn]
pub fn rainbow_board(ctx: &PQLFnContext, street: PQLStreet) -> PQLBoolean {
    core::rainbow_board(PQLBoard::from(ctx.get_board_slice(street)))
}
