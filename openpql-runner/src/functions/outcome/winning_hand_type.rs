use super::*;

#[pqlfn]
pub fn winning_hand_type(ctx: &PQLFnContext) -> PQLHandType {
    max_hi_rating(ctx, PQLStreet::River).into()
}
