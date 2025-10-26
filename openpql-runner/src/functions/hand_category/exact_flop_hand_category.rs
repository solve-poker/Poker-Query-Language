use super::*;

#[pqlfn]
pub fn exact_flop_hand_category(
    ctx: &PQLFnContext,
    player: PQLPlayer,
    category: PQLFlopHandCategory,
) -> PQLBoolean {
    flop_hand_category(ctx, player) == category
}

#[cfg(test)]
mod tests {
    use super::*;

    #[quickcheck]
    fn test_exact_flop_hand_category(ctx: TestPQLFnContext) {
        let ctx = ctx.as_ctx();
        let player = 0.into();

        let category = flop_hand_category(&ctx, player);

        for cat in PQLFlopHandCategory::ARR_ALL {
            assert_eq!(
                exact_flop_hand_category(&ctx, player, cat),
                cat == category
            );
        }
    }
}
