use crate::{PQLCard, functions::duplicated_hand_ranks};

/// Returns `true` when the hand contains at least one pocket pair.
pub fn pocket_pair(hand: &[PQLCard]) -> bool {
    !duplicated_hand_ranks(hand).is_empty()
}

#[cfg(test)]
mod tests {
    use openpql_prelude::Rank;
    use quickcheck_macros::quickcheck;

    use super::*;
    use crate::PQLCardSet;

    #[quickcheck]
    fn test_pocket_pair(hand: Vec<PQLCard>) {
        let c64 = PQLCardSet::from(hand.as_slice());
        let expected = Rank::all::<false>()
            .iter()
            .any(|&rank| c64.count_by_rank(rank) > 1);

        assert_eq!(pocket_pair(&hand), expected);
    }
}
