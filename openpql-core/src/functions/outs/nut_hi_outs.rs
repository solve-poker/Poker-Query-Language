use openpql_prelude::HandN;

use crate::{PQLBoard, PQLCard, PQLCardCount, PQLCardSet, PQLGame};

/// Counts the unseen cards that would give the hand an unbeatable high hand.
// TODO: optimize
#[expect(clippy::cast_possible_truncation, reason = "num of cards < u8::MAX")]
pub fn nut_hi_outs(game: PQLGame, hand: &[PQLCard], board: PQLBoard) -> PQLCardCount {
    let p = PQLCardSet::from(hand);
    let b = PQLCardSet::from(board);
    let known = p | b;

    let all = if game.is_shortdeck() {
        PQLCardSet::all::<true>()
    } else {
        PQLCardSet::all::<false>()
    };

    (all & !known)
        .iter()
        .filter(|&c| {
            let c64 = PQLCardSet::from(c);
            is_nut_hi(game, p, b | c64, known | c64)
        })
        .count() as PQLCardCount
}

fn is_nut_hi(game: PQLGame, p: PQLCardSet, b: PQLCardSet, known: PQLCardSet) -> bool {
    let player_rating = game.eval_rating(p, b);

    let check = |opp: PQLCardSet| -> bool {
        if !(opp & known).is_empty() {
            return true;
        }
        game.eval_rating(opp, b) <= player_rating
    };

    match game {
        PQLGame::Holdem => HandN::<2>::iter_all::<false>().all(|h| check(h.into())),
        PQLGame::ShortDeck => HandN::<2>::iter_all::<true>().all(|h| check(h.into())),
        PQLGame::Omaha => HandN::<4>::iter_all::<false>().all(|h| check(h.into())),
    }
}

#[cfg(test)]
mod tests {
    use openpql_prelude::{board, cards};

    use super::*;

    #[test]
    fn test_nut_hi_outs_holdem() {
        // KsJs + QdTd4c: 3 As (not Ad) + 3 9s (not 9d) = 6
        // Ad gives broadway but board becomes 4-flush → opp with any diamond wins
        // 9d gives straight but board becomes 4-flush → opp with any diamond wins
        assert_eq!(
            nut_hi_outs(PQLGame::Holdem, &cards!("Ks Js"), board!("Qd Td 4c"),),
            6,
        );
    }
}
