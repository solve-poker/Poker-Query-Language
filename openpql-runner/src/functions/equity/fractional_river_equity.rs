use super::*;

#[pqlfn]
pub fn fractional_river_equity(
    ctx: &PQLFnContext,
    hero: PQLPlayer,
) -> PQLFraction {
    let street = PQLStreet::River;

    let ratings: Vec<_> = PQLPlayer::iter(ctx.n_players)
        .map(|player| hi_rating(ctx, player, street))
        .collect();

    let max = *ratings.iter().max().unwrap();

    if ratings[usize::from(hero)] == max {
        let n_winners = ratings.iter().filter(|&r| *r == max).count();

        PQLFraction::pot_share(n_winners)
    } else {
        PQLFraction::zero()
    }
}
