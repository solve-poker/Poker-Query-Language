use super::*;

#[pqlfn(alias = "min_equity")]
pub fn min_hvhequity(
    ctx: &PQLFnContext,
    hero: PQLPlayer,
    street: PQLStreet,
    equity: PQLEquity,
) -> PQLBoolean {
    hvhequity(ctx, hero, street) >= equity
}
