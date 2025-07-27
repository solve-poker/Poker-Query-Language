use super::*;

#[pqlfn(arg, rtn, eval)]
pub fn pocket_pair(hand: &Hand) -> PQLBoolean {
    !duplicated_hand_ranks(hand, PQLStreet::Flop).is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[quickcheck]
    fn test_pocket_pair(cards: CardN<4>) -> TestResult {
        let hand: [_; 4] = cards.into();

        let mut rs = hand.into_iter().map(|c| c.r).collect::<Vec<_>>();

        rs.sort();
        rs.dedup();

        let pocket = rs.len() < 4;

        TestResult::from_bool(pocket == pocket_pair(&hand))
    }
}
