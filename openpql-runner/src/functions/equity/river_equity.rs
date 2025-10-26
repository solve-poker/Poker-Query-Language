use super::*;

#[pqlfn]
pub fn river_equity(ctx: &PQLFnContext, hero: PQLPlayer) -> PQLEquity {
    fractional_river_equity(ctx, hero).to_double()
}
