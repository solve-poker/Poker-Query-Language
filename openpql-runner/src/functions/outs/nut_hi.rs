use super::*;

// TODO: optimize
// TODO: deadcards
#[pqlfn]
pub fn nut_hi(
    ctx: &PQLFnContext,
    player: PQLPlayer,
    street: PQLStreet,
) -> PQLBoolean {
    let p64 = ctx.get_c64_player(player);
    let b64 = ctx.get_c64_board(street);
    let known_cards = p64 | b64;

    let player_rating = ctx.game.eval_rating(p64, b64);

    for other in ctx.iter_c64_player() {
        if !(other & known_cards).is_empty() {
            continue;
        }

        let other_rating = ctx.game.eval_rating(other, b64);

        if other_rating > player_rating {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_nut_hi_holdem() {
        let game = PQLGame::Holdem;

        let ctx = TestPQLFnContext::from_cards(game, cards!("Qs5s AsJsKs3h4c"));
        assert!(nut_hi(&ctx.as_ctx(), 0.into(), PQLStreet::Flop));

        let ctx = TestPQLFnContext::from_cards(game, cards!("As2s KsQsJs3h4c"));
        assert!(!nut_hi(&ctx.as_ctx(), 0.into(), PQLStreet::Flop));

        // Ts dead
        // let ctx = TestPQLFnContext::from_cards(game, cards!("As2s KsQsJs3h4c"));
        // assert!(!nut_hi(&mut ctx.as_ctx(), 0.into(), PQLStreet::Flop));
    }

    #[test]
    fn test_nut_hi_omaha() {
        let game = PQLGame::Omaha;

        let ctx =
            TestPQLFnContext::from_cards(game, cards!("9s8s2s2h KsQsJsTs2c"));
        assert!(nut_hi(&ctx.as_ctx(), 0.into(), PQLStreet::River));
    }
}
