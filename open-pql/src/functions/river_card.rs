use super::*;

#[pqlfn(arg, rtn, eval)]
pub const fn river_card(board: Board) -> PQLCard {
    board.river.unwrap()
}
