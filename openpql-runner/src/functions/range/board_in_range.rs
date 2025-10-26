use super::*;

#[pqlfn]
pub fn board_in_range(ctx: &PQLFnContext, board: &PQLBoardRange) -> PQLBoolean {
    board.is_satisfied(ctx.get_board_slice(PQLStreet::River))
}
