use super::*;

#[allow(clippy::cast_precision_loss)]
#[pqlfn(arg, rtn, eval)]
pub fn equity(
    pid: PQLPlayer,
    street: PQLStreet,
    (game, board, player_hands, ratings): (
        PQLGame,
        Board,
        &PlayerHands,
        &mut BufferRatings,
    ),
) -> PQLDouble {
    if street == PQLStreet::River {
        fill_ratings(PQLStreet::River, (game, board, player_hands, ratings));

        let max = ratings.max();

        if max == ratings[pid] {
            1.0 / ratings.iter().filter(|r| **r == max).count() as f64
        } else {
            0.0
        }
    } else if street == PQLStreet::Turn {
        let mut eq = 0.0;
        let mut count = 0;
        for c in Card::ARR_ALL {
            if board.contains_card(c) || card_in(player_hands, c) {
                continue;
            }

            fill_ratings(
                PQLStreet::River,
                (game, board.swap_river(c), player_hands, ratings),
            );

            let max = ratings.max();

            if max == ratings[pid] {
                eq +=
                    1.0 / ratings.iter().filter(|r| **r == max).count() as f64;
            }

            count += 1;
        }

        eq / f64::from(count)
    } else {
        todo!()
    }
}

fn card_in(hs: &PlayerHands, c: Card) -> bool {
    for h in hs {
        if h.contains(&c) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_min_outs_to_hand_type(hbg: HandBoardGame, ht: HandType) {
        let HandBoardGame {
            game,
            hand,
            board,
            dead,
            ..
        } = hbg;

        let ht = PQLHandType::from((ht, game));
        let outs =
            outs_to_hand_type(&hand, PQLStreet::Flop, ht, (game, board, dead));

        assert!(min_outs_to_hand_type(
            &hand,
            PQLStreet::Flop,
            ht,
            outs.saturating_sub(1),
            (game, board, dead)
        ));

        assert!(!min_outs_to_hand_type(
            &hand,
            PQLStreet::Flop,
            ht,
            outs + 1,
            (game, board, dead)
        ));
    }
}
