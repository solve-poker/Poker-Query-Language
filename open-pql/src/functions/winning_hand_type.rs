use super::*;

#[pqlfn(arg, rtn, eval)]
pub fn winning_hand_type(
    (game, board, player_hands, ratings): (
        PQLGame,
        Board,
        &PlayerHands,
        &mut BufferRatings,
    ),
) -> PQLHandType {
    fill_ratings(PQLStreet::River, (game, board, player_hands, ratings));

    ratings.max().to_hand_type(game)
}
