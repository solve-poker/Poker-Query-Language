use super::*;

#[pqlfn]
pub fn duplicated_board_ranks(street: PQLStreet, board: Board) -> PQLRankSet {
    let c64: Card64 = (board, street).into();
    let (_, has2, _, _) = get_card_count(c64.to_u64());

    PQLRankSet::from_u16(has2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_duplicated_board_ranks(
        board: Board,
        street: PQLStreet,
    ) -> TestResult {
        let c64: Card64 = (board, street).into();
        let mut ranks = Rank16::empty();

        for rank in Rank::ARR_ALL {
            if c64.count_by_rank(rank) > 1 {
                ranks.set(rank);
            }
        }

        TestResult::from_bool(ranks == duplicated_board_ranks(street, board))
    }
}
