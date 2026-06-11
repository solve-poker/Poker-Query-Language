use crate::{
    PQLBoard, PQLCard, PQLEquity, PQLGame, PQLPlayerCount, functions::fractional_river_equity,
};

/// Returns the hero's river equity as a `PQLEquity` in `[0.0, 1.0]`.
pub fn river_equity(
    game: PQLGame,
    board: PQLBoard,
    player_cards: &[PQLCard],
    hero_idx: PQLPlayerCount,
) -> PQLEquity {
    fractional_river_equity(game, board, player_cards, hero_idx).to_double()
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use openpql_prelude::{board, cards};

    use super::*;

    #[test]
    fn test_river_equity_cases() {
        let game = PQLGame::Holdem;
        let b = board!("Ad 7c 2d Jh 9s");

        let both = cards!("As Ah Ks Kh");
        assert_relative_eq!(river_equity(game, b, &both, 0), 1.0);
        assert_relative_eq!(river_equity(game, b, &both, 1), 0.0);

        let tie_b = board!("7h 7c 7d As Ah");
        let tie_hands = cards!("2c 3d 4c 5d");
        assert_relative_eq!(river_equity(game, tie_b, &tie_hands, 0), 0.5);
        assert_relative_eq!(river_equity(game, tie_b, &tie_hands, 1), 0.5);
    }
}
