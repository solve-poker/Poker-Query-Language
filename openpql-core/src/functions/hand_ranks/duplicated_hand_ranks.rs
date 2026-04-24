use crate::{PQLCard, PQLCardSet, PQLRankSet, util};

/// Returns the set of ranks that appear at least twice in the hand.
pub fn duplicated_hand_ranks(hand: &[PQLCard]) -> PQLRankSet {
    let [_, has2, _, _] = util::rank_cardinality(PQLCardSet::from(hand));

    has2
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use super::*;
    use crate::PQLRank;

    #[quickcheck]
    fn test_duplicated_hand_ranks(hand: Vec<PQLCard>) {
        let c64 = PQLCardSet::from(hand.as_slice());
        let mut expected = PQLRankSet::default();
        for &rank in PQLRank::all::<false>() {
            if c64.count_by_rank(rank) > 1 {
                expected.set(rank);
            }
        }

        assert_eq!(duplicated_hand_ranks(&hand), expected);
    }
}
