use super::*;

#[pqlfn]
pub fn min_hand_type(
    ctx: &PQLFnContext,
    player: PQLPlayer,
    street: PQLStreet,
    ht: PQLHandType,
) -> PQLBoolean {
    let computed = hand_type(ctx, player, street);

    let order = if ctx.game.is_shortdeck() {
        computed.compare::<true>(ht)
    } else {
        computed.compare::<false>(ht)
    };

    !matches!(order, cmp::Ordering::Less)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[quickcheck]
    fn test_min_hand_type(ctx: TestPQLFnContext, street: PQLStreet) {
        let ctx = ctx.as_ctx();
        let player = 0u8.into();

        let exact_ht = hand_type(&ctx, player, street);
        let compare = if ctx.game.is_shortdeck() {
            PQLHandType::compare::<true>
        } else {
            PQLHandType::compare::<false>
        };

        for ht in PQLHandType::ARR_ALL {
            assert_eq!(
                min_hand_type(&ctx, player, street, ht),
                !matches!(compare(exact_ht, ht), cmp::Ordering::Less)
            );
        }
    }
}
