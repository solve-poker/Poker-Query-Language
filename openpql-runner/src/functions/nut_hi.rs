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

    let player_rating = ctx.eval_current_rating(player, street);

    for other in ctx.iter_c64_player() {
        if !(other & known_cards).is_empty() {
            continue;
        }

        // TODO: cache
        let other_rating = ctx.game.eval_rating(other, b64);

        if other_rating > player_rating {
            return false;
        }
    }

    true
}
