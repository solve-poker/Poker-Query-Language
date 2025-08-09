use super::*;

#[pqlfn]
pub const fn turn_card(board: Board) -> PQLCard {
    board.turn.unwrap()
}
