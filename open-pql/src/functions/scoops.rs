use super::*;

#[pqlfn]
pub fn scoops(
    pid: PQLPlayer,
    (game, board, player_hands, ratings): (
        PQLGame,
        Board,
        &PlayerHands,
        &mut BufferRatings,
    ),
) -> PQLBoolean {
    fill_ratings(PQLStreet::River, (game, board, player_hands, ratings));

    let max = ratings.max();

    if max != ratings[pid] {
        return false;
    }

    !ratings
        .iter()
        .enumerate()
        .any(|(i, r)| i != pid.to_usize() && *r == max)
}
