use crate::{PQLCard, PQLRankSet};

pub fn hand_ranks(hand: &[PQLCard]) -> PQLRankSet {
    PQLRankSet::from(hand)
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use super::*;

    #[quickcheck]
    fn test_hand_ranks(hand: Vec<PQLCard>) {
        let expected: PQLRankSet = hand.iter().map(|c| c.rank).collect();

        assert_eq!(hand_ranks(&hand), expected);
    }
}
