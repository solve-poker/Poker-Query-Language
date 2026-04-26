use crate::{
    PQLBoard, PQLCard, PQLCardSet, PQLFraction, PQLGame, PQLPlayerCount,
};

/// Returns the hero's exact share of the pot on the river.
///
/// Returns zero if the hero does not hold the best hand; otherwise splits the
/// pot evenly across all tied winners.
///
/// # Panics
/// Panics if `player_cards.len()` is not a multiple of `game.player_cards_len()`,
/// or if `hero_idx` is not a valid player index,
/// or if `board` has less than 5 cards.
pub fn fractional_river_equity(
    game: PQLGame,
    board: PQLBoard,
    player_cards: &[PQLCard],
    hero_idx: PQLPlayerCount,
) -> PQLFraction {
    debug_assert!(
        board.len() == PQLBoard::N_RIVER,
        "board must have {} cards, got {}",
        PQLBoard::N_RIVER,
        board.len(),
    );

    let n_cards = game.player_cards_len() as usize;
    debug_assert!(
        player_cards.len().is_multiple_of(n_cards),
        "player_cards length ({}) is not a multiple of {n_cards}",
        player_cards.len(),
    );
    let n_players = player_cards.len() / n_cards;
    debug_assert!(
        (hero_idx as usize) < n_players,
        "hero_idx {hero_idx} out of range (n_players = {n_players})",
    );

    let b = PQLCardSet::from(board);

    let ratings: Vec<_> = player_cards
        .chunks_exact(n_cards)
        .map(|h| game.eval_rating(PQLCardSet::from(h), b))
        .collect();

    let max = *ratings.iter().max().unwrap();
    let hero = ratings[hero_idx as usize];

    if hero < max {
        PQLFraction::zero()
    } else {
        let n_winners = ratings.iter().filter(|&&r| r == max).count();
        PQLFraction::pot_share(n_winners)
    }
}

#[cfg(test)]
mod tests {
    use openpql_prelude::{board, cards};

    use super::*;

    #[test]
    fn test_fractional_river_equity_cases() {
        let game = PQLGame::Holdem;
        let b = board!("Ad 7c 2d Jh 9s");

        let solo = cards!("As Ah");
        let f = fractional_river_equity(game, b, &solo, 0);
        assert_eq!(f, PQLFraction::pot_share(1));

        let both = cards!("As Ah Ks Kh");
        let f = fractional_river_equity(game, b, &both, 0);
        assert_eq!(f, PQLFraction::pot_share(1));
        let f = fractional_river_equity(game, b, &both, 1);
        assert_eq!(f, PQLFraction::zero());

        let tie_b = board!("7h 7c 7d As Ah");
        let tie_hands = cards!("2c 3d 4c 5d");
        let f = fractional_river_equity(game, tie_b, &tie_hands, 0);
        assert_eq!(f, PQLFraction::pot_share(2));
        let f = fractional_river_equity(game, tie_b, &tie_hands, 1);
        assert_eq!(f, PQLFraction::pot_share(2));
    }

    #[test]
    #[should_panic(expected = "board must have 5 cards")]
    fn test_invalid_board() {
        let game = PQLGame::Holdem;
        let bad = board!("Ad 7c 2d Jh");
        let player = cards!("As Ah Ks");
        let _ = fractional_river_equity(game, bad, &player, 0);
    }

    #[test]
    #[should_panic(expected = "not a multiple")]
    fn test_invalid_player_cards_len() {
        let game = PQLGame::Holdem;
        let b = board!("Ad 7c 2d Jh 9s");
        let bad = cards!("As Ah Ks");
        let _ = fractional_river_equity(game, b, &bad, 0);
    }

    #[test]
    #[should_panic(expected = "out of range")]
    fn test_invalid_hero_idx() {
        let game = PQLGame::Holdem;
        let b = board!("Ad 7c 2d Jh 9s");
        let hands = cards!("As Ah Ks Kh");
        let _ = fractional_river_equity(game, b, &hands, 2);
    }
}
