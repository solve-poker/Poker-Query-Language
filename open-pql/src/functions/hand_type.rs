use super::*;
#[pqlfn(arg, rtn, eval)]
pub fn hand_type(
    hand: &Hand,
    street: PQLStreet,
    (game, board): (PQLGame, Board),
) -> PQLHandType {
    hi_rating(hand, street, (game, board)).to_hand_type(game)
}
