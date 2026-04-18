use super::*;

#[pqlfn]
pub fn river_card(ctx: &PQLFnContext) -> PQLCard {
    ctx.get_board_slice(PQLStreet::River)[PQLBoard::IDX_RIVER]
}
