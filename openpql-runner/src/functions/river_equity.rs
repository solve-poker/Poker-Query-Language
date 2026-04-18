use super::*;

#[pqlfn]
pub fn river_equity(ctx: &PQLFnContext, hero: PQLPlayer) -> PQLEquity {
    let idx_board = PQLFnContext::idx_board_start(
        ctx.n_players,
        ctx.game.player_cards_len(),
    );
    let player_cards = &ctx.sampled_cards[..idx_board];
    let board = ctx.get_board(PQLStreet::River);

    core::river_equity(ctx.game, board, player_cards, hero.into())
}
