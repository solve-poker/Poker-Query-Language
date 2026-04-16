use super::*;

#[pqlfn]
pub fn min_flop_hand_category(
    ctx: &PQLFnContext,
    player: PQLPlayer,
    category: PQLFlopHandCategory,
) -> PQLBoolean {
    core::min_flop_hand_category(
        ctx.game,
        ctx.get_player_slice(player),
        PQLBoard::from(ctx.get_board_slice(PQLStreet::Flop)),
        category,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[quickcheck]
    fn test_min_flop_hand_category(ctx: TestPQLFnContext) {
        let ctx = ctx.as_ctx();
        let player = 0.into();

        let category = flop_hand_category(&ctx, player);
        let compare = if ctx.game.is_shortdeck() {
            PQLFlopHandCategory::compare::<true>
        } else {
            PQLFlopHandCategory::compare::<false>
        };

        for cat in PQLFlopHandCategory::ARR_ALL {
            assert_eq!(
                min_flop_hand_category(&ctx, player, cat),
                !matches!(compare(category, cat), cmp::Ordering::Less)
            );
        }
    }
}
