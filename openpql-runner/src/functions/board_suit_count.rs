use super::*;

#[pqlfn]
pub fn board_suit_count(ctx: &PQLFnContext, street: PQLStreet) -> PQLCardCount {
    core::board_suit_count(ctx.get_board(street))
}
