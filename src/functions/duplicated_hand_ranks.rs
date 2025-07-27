use super::*;

#[pqlfn]
pub fn duplicated_hand_ranks(hand: &Hand, _street: PQLStreet) -> PQLRankSet {
    let c64: Card64 = hand.into();
    let (_, has2, _, _) = get_card_count(c64.to_u64());

    PQLRankSet::from_u16(has2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_duplicated_hand_ranks(hbg: HandBoardGame) -> TestResult {
        let hand = hbg.hand;

        let duplicated: Rank16 = duplicated_hand_ranks(&hand, hbg.street);

        let rs = hand.into_iter().map(|c| c.r).collect::<Vec<_>>();

        for r in &rs {
            if rs.iter().filter(|el| **el == *r).count() > 1 {
                assert!(duplicated.contains_rank(*r));
            } else {
                assert!(!duplicated.contains_rank(*r));
            }
        }

        TestResult::passed()
    }
}
