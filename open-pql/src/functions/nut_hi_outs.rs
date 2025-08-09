use super::*;
#[pqlfn(arg, rtn, eval)]
pub fn nut_hi_outs(
    _hand: &Hand,
    _street: PQLStreet,
    _: (PQLGame, Board, DeadCards),
) -> PQLCardCount {
    todo!()
    // TODO: check correctness

    //if PQLStreet::River == street {
    //    return 0;
    //}

    //let rating = nut_hi_rating(game, board, street);

    //match game {
    //    PQLGame::Holdem => {
    //        let mut outs = 0;
    //        let mut c64: Card64 = (board, street).into();
    //        c64.set(hand[0]);
    //        c64.set(hand[1]);

    //        for c in Card::ARR_ALL {
    //            if c64.contains_card(c) {
    //                continue;
    //            }

    //            let mut cards = c64;
    //            cards.set(c);

    //            if rating == eval_holdem7(cards) {
    //                outs += 1;
    //            }
    //        }

    //        outs
    //    }

    //    _ => todo!(),
    //}
}
