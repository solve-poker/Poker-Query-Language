use super::*;

#[pqlfn]
pub fn hand_type(
    ctx: &PQLFnContext,
    player: PQLPlayer,
    street: PQLStreet,
) -> PQLHandType {
    hi_rating(ctx, player, street).into()
}
