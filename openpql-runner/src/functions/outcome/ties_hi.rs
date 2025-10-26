use super::*;

// TODO: optimize
#[pqlfn]
pub fn ties_hi(ctx: &PQLFnContext, player: PQLPlayer) -> PQLBoolean {
    let player_rating = hi_rating(ctx, player, PQLStreet::River);
    let max_rating = max_hi_rating(ctx, PQLStreet::River);

    if player_rating != max_rating {
        return false;
    }

    for i in 0..ctx.n_players {
        let other = PQLPlayer::from(i);

        if player != other
            && max_rating == hi_rating(ctx, other, PQLStreet::River)
        {
            return true;
        }
    }

    false
}
