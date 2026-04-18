use super::*;

#[pqlfn]
pub fn turn_card(ctx: &PQLFnContext) -> PQLCard {
    ctx.get_board_slice(PQLStreet::River)[PQLBoard::IDX_TURN]
}
