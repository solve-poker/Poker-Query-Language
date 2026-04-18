use super::*;

#[pqlfn]
pub fn rate_hi_hand(
    ctx: &PQLFnContext,
    text: &PQLString,
) -> Result<PQLHiRating, RuntimeError> {
    let game = match ctx.game {
        PQLGame::Holdem | PQLGame::Omaha => PQLGame::Holdem,
        PQLGame::ShortDeck => PQLGame::ShortDeck,
    };

    parse_cards(text).map_or(Err(RuntimeError::InvalidHand), |cards| {
        if cards.count() == 5 {
            Ok(game.eval_rating(cards, PQLCardSet::default()))
        } else {
            Err(RuntimeError::RequiresFiveCards)
        }
    })
}
