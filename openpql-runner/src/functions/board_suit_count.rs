use super::*;

#[pqlfn]
pub fn board_suit_count(ctx: &PQLFnContext, street: PQLStreet) -> PQLCardCount {
    core::board_suit_count(PQLBoard::from(ctx.get_board_slice(street)))
}
