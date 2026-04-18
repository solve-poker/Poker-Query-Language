use super::*;

#[pqlfn]
pub fn straight_board(ctx: &PQLFnContext, street: PQLStreet) -> PQLBoolean {
    core::straight_board(ctx.game, PQLBoard::from(ctx.get_board_slice(street)))
}
