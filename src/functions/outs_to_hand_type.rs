use outs_info::{outs_holdem, outs_shortdeck};

use super::*;

/// It calculates how many cards would enhance the current hand to form the desired hand type, provided it's superior to the existing hand.
///
/// For instance,
///   outsToHandType(p1, flop, straight) with p1=JsTh and board=9h8s2c would return 8, unless the 7s is a dead card, in which case it returns 7.
/// Similarly,
///   outsToHandType(p1, turn, straight) with p1=Th9h and board=8h7h2c3c results in 6, since two of the 'straight' outs would actually result in a straight-flush rather than just a straight.
/// In another example,
///   outsToHandType(p1, flop, straight) with p1=Td9d and board=8d7d2d evaluates to 0, as the player already holds a superior hand to a straight.
#[pqlfn(arg, rtn, eval)]
pub fn outs_to_hand_type(
    hand: &Hand,
    street: PQLStreet,
    target_ht: PQLHandType,
    (game, board, dead): (PQLGame, Board, DeadCards),
) -> PQLCardCount {
    let _next_is_river = match street {
        PQLStreet::Flop => false,
        PQLStreet::Turn => true,
        PQLStreet::River => return 0,
    };

    let player: Card64 = hand.into();
    let c64_board: Card64 = (board, street).into();
    let used = used_cards(hand, board, street, dead);

    let info = match game {
        PQLGame::Holdem => outs_holdem(player, c64_board),
        PQLGame::ShortDeck => outs_shortdeck(player, c64_board),
        PQLGame::Omaha => {
            return outs_to_hand_type_tmp(
                hand,
                street,
                target_ht,
                (game, board, dead),
            );
        }
    };

    let cards = info.outs_of(target_ht.ht);

    (cards & !used).count()
}

fn outs_to_hand_type_tmp(
    hand: &Hand,
    street: PQLStreet,
    target_ht: PQLHandType,
    (game, board, dead): (PQLGame, Board, DeadCards),
) -> PQLCardCount {
    let next_is_river = match street {
        PQLStreet::Flop => false,
        PQLStreet::Turn => true,
        PQLStreet::River => return 0,
    };

    let used = used_cards(hand, board, street, dead);
    let current_rating = hi_rating(hand, street, (game, board));
    let current_ht = current_rating.to_hand_type(game);

    if target_ht < current_ht {
        return 0;
    }

    let mut outs = 0;

    for c in PQLCard::ARR_ALL {
        if used.contains_card(c) {
            continue;
        }

        let new_rating = if next_is_river {
            hi_rating(hand, PQLStreet::River, (game, board.swap_river(c)))
        } else {
            hi_rating(hand, PQLStreet::Turn, (game, board.swap_turn(c)))
        };

        let new_ht = new_rating.to_hand_type(game);

        if new_rating > current_rating && new_ht == target_ht {
            outs += 1;
        }
    }

    outs
}

#[inline]
fn used_cards(
    hand: &Hand,
    board: Board,
    street: PQLStreet,
    dead: DeadCards,
) -> Card64 {
    let mut c = Card64::from((board, street));
    c |= dead.into();
    c |= hand.into();

    c
}

#[cfg_attr(coverage_nightly, coverage(off))]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    pub fn outs_to_hand_type_brute_force(
        hand: &Hand,
        street: PQLStreet,
        target_ht: PQLHandType,
        (game, board, dead): (PQLGame, Board, DeadCards),
    ) -> PQLCardCount {
        let next_is_river = match street {
            PQLStreet::Flop => false,
            PQLStreet::Turn => true,
            PQLStreet::River => return 0,
        };

        let used = used_cards(hand, board, street, dead);
        let current_rating = hi_rating(hand, street, (game, board));
        let current_ht = current_rating.to_hand_type(game);

        if target_ht < current_ht {
            return 0;
        }

        let mut outs = 0;

        for c in PQLCard::ARR_ALL {
            if used.contains_card(c) {
                continue;
            }

            let new_rating = if next_is_river {
                hi_rating(hand, PQLStreet::River, (game, board.swap_river(c)))
            } else {
                hi_rating(hand, PQLStreet::Turn, (game, board.swap_turn(c)))
            };

            let new_ht = new_rating.to_hand_type(game);

            if new_rating > current_rating && new_ht == target_ht {
                outs += 1;
            }
        }

        outs
    }

    #[quickcheck]
    fn test_outs_to_hand_type(hbg: HandBoardGame, ht: HandType) {
        let HandBoardGame {
            game,
            hand,
            board,
            dead,
            ..
        } = hbg;

        let ht = PQLHandType::from((ht, game));

        assert_eq!(
            outs_to_hand_type_brute_force(
                &hand,
                PQLStreet::Flop,
                ht,
                (game, board, dead)
            ),
            outs_to_hand_type(&hand, PQLStreet::Flop, ht, (game, board, dead))
        );

        assert_eq!(
            outs_to_hand_type_brute_force(
                &hand,
                PQLStreet::Turn,
                ht,
                (game, board, dead)
            ),
            outs_to_hand_type(&hand, PQLStreet::Turn, ht, (game, board, dead))
        );

        assert_eq!(
            0,
            outs_to_hand_type(&hand, PQLStreet::River, ht, (game, board, dead))
        );
    }
}
