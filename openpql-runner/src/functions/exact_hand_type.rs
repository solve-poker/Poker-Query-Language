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
