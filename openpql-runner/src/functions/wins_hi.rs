use super::*;

#[pqlfn]
pub fn wins_hi(ctx: &PQLFnContext, player: PQLPlayer) -> PQLBoolean {
    best_hi_rating(ctx, player, PQLStreet::River)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_wins_hi(ctx: TestPQLFnContext) {
        let ctx = ctx.as_ctx();
        let player = 0.into();

        let player_win = wins_hi(&ctx, player);
        let player_tie = ties_hi(&ctx, player);
        let player_scoop = scoops(&ctx, player);
        let player_best = best_hi_rating(&ctx, player, PQLStreet::River);

        // best <=> win <=> (scoop || tie)
        // win => (tie <=> not scoop)
        assert_eq!(player_win, player_best);
        assert_eq!(player_win, player_scoop || player_tie);
        if player_win {
            assert_eq!(player_scoop, !player_tie);
        }
    }
}
