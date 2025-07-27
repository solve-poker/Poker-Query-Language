use super::*;

#[pqlfn(arg, rtn, eval)]
pub fn best_hi_rating(
    pid: PQLPlayer,
    street: PQLStreet,
    (game, board, player_hands, ratings): (
        PQLGame,
        Board,
        &PlayerHands,
        &mut BufferRatings,
    ),
) -> PQLBoolean {
    fill_ratings(street, (game, board, player_hands, ratings));

    ratings[pid] == ratings.max()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_best_hi_rating() {
        let board = board!("2s7hTd Qc As");
        let hands = vec![cards!("7c Ts"), cards!("2h 2d")];
        let mut ratings = BufferRatings::new(2);

        assert!(best_hi_rating(
            1.into(),
            PQLStreet::Flop,
            (PQLGame::Holdem, board, &hands, &mut ratings)
        ));
    }
}
