use super::*;

#[pqlfn(arg, rtn, eval)]
pub fn hi_rating(
    hand: &Hand,
    street: PQLStreet,
    (game, board): (PQLGame, Board),
) -> PQLHiRating {
    let c64_board = (board, street).into();

    match game {
        PQLGame::Holdem => eval_holdem(hand, c64_board),
        PQLGame::ShortDeck => eval_shortdeck(hand, c64_board),
        PQLGame::Omaha => eval_omaha(hand, c64_board),
    }
}

#[inline]
fn eval_holdem(hand: &Hand, mut c64: Card64) -> PQLHiRating {
    c64.set(hand[0]);
    c64.set(hand[1]);

    eval_holdem7(c64)
}

#[inline]
fn eval_shortdeck(hand: &Hand, mut c64: Card64) -> PQLHiRating {
    c64.set(hand[0]);
    c64.set(hand[1]);

    eval_shortdeck7(c64)
}

#[inline]
fn eval_omaha(hand: &Hand, board: Card64) -> PQLHiRating {
    let h = hand.into();

    eval_omaha9(h, board)
}
