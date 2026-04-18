use super::*;

#[pqlfn]
pub fn duplicated_board_ranks(
    ctx: &PQLFnContext,
    street: PQLStreet,
) -> PQLRankSet {
    let board = ctx.get_c64_board(street);

    let [_, more_than_two, _, _] = rank_cardinality(board);

    more_than_two
}
