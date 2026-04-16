use super::*;

#[pqlfn]
pub fn straight_board(ctx: &PQLFnContext, street: PQLStreet) -> PQLBoolean {
    core::straight_board(ctx.game, PQLBoard::from(ctx.get_board_slice(street)))
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::*;

    fn assert_straight_board(
        game: PQLGame,
        street: PQLStreet,
        s: &str,
        expected: bool,
    ) {
        let ctx = TestPQLFnContext::from_board(&cards!(s));
        let mut ctx = ctx.as_ctx();
        ctx.game = game;

        assert_eq!(straight_board(&ctx, street), expected, "{s} {expected}");
    }

    #[test]
    fn test_holdem_omaha() {
        for game in [PQLGame::Holdem, PQLGame::Omaha] {
            assert_straight_board(game, PQLStreet::Flop, "5h 6c 7d", true);
            assert_straight_board(game, PQLStreet::Flop, "2h 5c 9d", false);
            assert_straight_board(game, PQLStreet::Flop, "Ah 2c 3d", true);
            assert_straight_board(game, PQLStreet::Flop, "Qh Kc Ad", true);
            assert_straight_board(
                game,
                PQLStreet::River,
                "5h 6c 7d 8s 9h",
                true,
            );
        }
    }

    #[test]
    fn test_shortdeck() {
        let game = PQLGame::ShortDeck;
        assert_straight_board(game, PQLStreet::Flop, "9h Tc Jd", true);
        assert_straight_board(game, PQLStreet::Flop, "Ah 8d 9c", true);
    }
}
