use super::*;
/// It evaluates whether a specified player holds the best possible hand for their specific hand type on the given street. For example, `nutHiForHandType(p1, flop)` returns `true` if player one (`p1`) has the hand `'AsKh'` and the flop shows `'AdTd2d'`, because `p1` holds the top possible hand within the "one pair" category.
#[pqlfn]
pub fn nut_hi_for_hand_type(
    hand: &Hand,
    street: PQLStreet,
    (game, board, dead): (PQLGame, Board, DeadCards),
) -> PQLBoolean {
    // TODO: optimize

    let known_cards = Card64::from(dead) | (board, street).into() | hand.into();

    let player_rating = hi_rating(hand, street, (game, board));
    let player_ht = player_rating.to_hand_type(game);

    for other in (!known_cards).iter().combinations(game.n_cards().into()) {
        let other_rating = hi_rating(&other, street, (game, board));
        let other_ht = other_rating.to_hand_type(game);

        if other_ht == player_ht && other_rating > player_rating {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    pub fn nut_hi_for_hand_type_brute_force(
        game: PQLGame,
        hand: &Hand,
        board: Board,
        street: PQLStreet,
        dead: DeadCards,
    ) -> PQLBoolean {
        let known_cards =
            Card64::from(dead) | (board, street).into() | hand.into();

        let player_rating = hi_rating(hand, street, (game, board));
        let player_ht = player_rating.to_hand_type(game);

        for other in (!known_cards).iter().combinations(game.n_cards().into()) {
            let other_rating = hi_rating(&other, street, (game, board));
            let other_ht = other_rating.to_hand_type(game);

            if other_ht == player_ht && other_rating > player_rating {
                return false;
            }
        }

        true
    }

    #[test]
    fn test_nut_hi_for_hand_type_holdem() {
        let g = PQLGame::Holdem;

        assert!(nut_hi_for_hand_type(
            cards!("AsKh").as_ref(),
            PQLStreet::Flop,
            (g, board!("AdTd2d 3h4c"), DeadCards::default())
        ));
    }

    #[quickcheck]
    fn test_nut_hi_for_hand_type(
        hbg: HandBoardGame,
        street: PQLStreet,
    ) -> TestResult {
        TestResult::from_bool(
            nut_hi_for_hand_type_brute_force(
                hbg.game, &hbg.hand, hbg.board, street, hbg.dead,
            ) == nut_hi_for_hand_type(
                &hbg.hand,
                street,
                (hbg.game, hbg.board, hbg.dead),
            ),
        )
    }
}
