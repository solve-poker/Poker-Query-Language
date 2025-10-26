use super::*;

/// # Panics
/// won't panic since board is non-empty
#[pqlfn]
pub fn overpair(
    ctx: &PQLFnContext,
    player: PQLPlayer,
    street: PQLStreet,
) -> PQLBoolean {
    duplicated_hand_ranks(ctx, player)
        .max_rank()
        .is_some_and(|max| max > board_ranks(ctx, street).max_rank().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[quickcheck]
    fn test_overpair(ctx: TestPQLFnContext, street: PQLStreet) {
        let ctx = ctx.as_ctx();

        let board_ranks = ctx
            .get_c64_board(street)
            .iter()
            .map(|c| c.rank)
            .collect::<Vec<_>>();

        for player in PQLPlayer::iter(ctx.n_players) {
            let player_hand = ctx.get_c64_player(player);

            let mut has_overpair = false;

            for &rank in PQLRank::all::<false>() {
                if player_hand.count_by_rank(rank) > 1
                    && board_ranks.iter().all(|b| rank > *b)
                {
                    has_overpair = true;
                }
            }

            assert_eq!(has_overpair, overpair(&ctx, player, street));
        }
    }
}
