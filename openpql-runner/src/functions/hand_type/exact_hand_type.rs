use super::*;

#[pqlfn]
pub fn exact_hand_type(
    ctx: &PQLFnContext,
    player: PQLPlayer,
    street: PQLStreet,
    ht: PQLHandType,
) -> PQLBoolean {
    hand_type(ctx, player, street) == ht
}

#[cfg(test)]
mod tests {
    use super::*;

    #[quickcheck]
    fn test_exact_hand_type(ctx: TestPQLFnContext, street: PQLStreet) {
        let ctx = ctx.as_ctx();
        let player = 0u8.into();

        let ht = hand_type(&ctx, player, street);

        for kind in PQLHandType::ARR_ALL {
            assert_eq!(exact_hand_type(&ctx, player, street, kind), kind == ht);
        }
    }
}
