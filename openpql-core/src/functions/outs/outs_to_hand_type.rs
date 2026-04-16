use crate::{
    PQLBoard, PQLCard, PQLCardCount, PQLCardSet, PQLGame, PQLHandType,
};

#[expect(clippy::cast_possible_truncation, reason = "num of cards < u8::MAX")]
pub fn outs_to_hand_type(
    game: PQLGame,
    hand: &[PQLCard],
    board: PQLBoard,
    target: PQLHandType,
) -> PQLCardCount {
    let p = PQLCardSet::from(hand);
    let b = PQLCardSet::from(board);
    let current = game.eval_rating(p, b);
    let known = p | b;

    let all = if game.is_shortdeck() {
        PQLCardSet::all::<true>()
    } else {
        PQLCardSet::all::<false>()
    };

    (all & !known)
        .iter()
        .filter(|&c| {
            let new_b = b | PQLCardSet::from(c);
            let r = game.eval_rating(p, new_b);
            PQLHandType::from(r) == target && r > current
        })
        .count() as PQLCardCount
}

#[cfg(test)]
mod tests {
    use openpql_prelude::{board, cards};

    use super::*;

    #[test]
    fn test_outs_to_hand_type_cases() {
        // JsTh + 9h8s2c: 4 sevens + 4 queens = 8 outs to straight
        assert_eq!(
            outs_to_hand_type(
                PQLGame::Holdem,
                &cards!("Js Th"),
                board!("9h 8s 2c"),
                PQLHandType::Straight,
            ),
            8,
        );

        // Th9h + 8h7h2c3c: 8 straight outs (J/6) - 2 straight flushes (Jh, 6h) = 6
        assert_eq!(
            outs_to_hand_type(
                PQLGame::Holdem,
                &cards!("Th 9h"),
                board!("8h 7h 2c 3c"),
                PQLHandType::Straight,
            ),
            6,
        );

        // Td9d + 8d7d2d: already a flush, straight can't beat it = 0
        assert_eq!(
            outs_to_hand_type(
                PQLGame::Holdem,
                &cards!("Td 9d"),
                board!("8d 7d 2d"),
                PQLHandType::Straight,
            ),
            0,
        );
    }
}
