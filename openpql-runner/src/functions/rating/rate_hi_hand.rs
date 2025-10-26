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

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_rate_hi_hand(ctx: TestPQLFnContext) {
        let ctx = &mut ctx.as_ctx();
        if ctx.game != PQLGame::Omaha {
            let player = 0.into();

            let rating = hi_rating(ctx, player, PQLStreet::Flop);
            let text = ctx
                .get_player_slice(player)
                .iter()
                .chain(ctx.get_board_slice(PQLStreet::Flop))
                .map(ToString::to_string)
                .collect::<String>();

            assert_eq!(rating, rate_hi_hand(ctx, &text).unwrap(), "{text}");
        }
    }

    #[test]
    fn test_rate_hi_hand_error() {
        let ctx = &PQLFnContext::default();
        let exec = |hand: &str| rate_hi_hand(ctx, &hand.to_string());

        assert!(exec(" As Ks Qs Js Ts ").is_ok());

        let s = "AsKsQsJsTs";
        for i in 0..s.len() {
            assert!(exec(&s[0..i]).is_err());
        }

        assert_eq!(exec(" A Ks Qs Js Ts "), Err(RuntimeError::InvalidHand));
        assert_eq!(exec(" sA Ks Qs Js Ts "), Err(RuntimeError::InvalidHand));

        assert_eq!(exec("AsKsQsJsTs9s"), Err(RuntimeError::RequiresFiveCards));

        assert_eq!(exec("AsAsAsAsAs"), Err(RuntimeError::RequiresFiveCards));
    }
}
