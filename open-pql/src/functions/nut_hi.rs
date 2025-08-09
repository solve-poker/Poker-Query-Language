use super::*;
///  It determines whether a specified player has the "nuts" (the best possible hand) on a given street in poker. For example, calling `nutHi(p1, flop)` returns `true` if player one (`p1`) holds the hand `'Qs5s'` and the community board shows `'JsKsAs'`. It's important to note that the function assumes all dead cards are known. Thus, `nutHi(p1, flop)` for `p1='As2s'` and `board='KsQsJs'` would initially return `false`. However, if a dead card `'Ts'` is introduced, the function would then return `true`.
#[pqlfn(arg, rtn, eval)]
pub fn nut_hi(
    hand: &Hand,
    street: PQLStreet,
    (game, board, dead): (PQLGame, Board, DeadCards),
) -> PQLBoolean {
    // TODO: optimize

    let known_cards = Card64::from(dead) | (board, street).into() | hand.into();

    let player_rating = hi_rating(hand, street, (game, board));

    for other in (!known_cards).iter().combinations(game.n_cards().into()) {
        let other_rating = hi_rating(&other, street, (game, board));

        if other_rating > player_rating {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_nut_hi_holdem() {
        let g = PQLGame::Holdem;

        assert!(nut_hi(
            cards!("Qs5s").as_ref(),
            PQLStreet::Flop,
            (g, board!("AsJsKs 3h4c"), DeadCards::default())
        ));

        assert!(!nut_hi(
            cards!("As2s").as_ref(),
            PQLStreet::Flop,
            (g, board!("KsQsJs 3h4c"), DeadCards::default())
        ));

        assert!(nut_hi(
            cards!("As2s").as_ref(),
            PQLStreet::Flop,
            (g, board!("KsQsJs 3h4c"), c64!("Ts").into())
        ));
    }

    #[test]
    fn test_nut_hi_omaha() {
        let g = PQLGame::Omaha;

        assert!(nut_hi(
            cards!("9s8s 2s2h").as_ref(),
            PQLStreet::River,
            (g, board!("KsQsJsTs 2c"), DeadCards::default())
        ));
    }
}
