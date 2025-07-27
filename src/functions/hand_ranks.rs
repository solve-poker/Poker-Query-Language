use super::*;
#[pqlfn(arg, rtn, eval)]
pub fn hand_ranks(hand: &Hand, _street: PQLStreet) -> PQLRankSet {
    let mut ranks = Rank16::empty();

    for c in hand {
        ranks.set(c.r);
    }

    ranks
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_hand_ranks(
        cards: (Card, Card, Card, Card),
        street: PQLStreet,
    ) -> TestResult {
        let hand = [cards.0, cards.1, cards.2, cards.3];

        let ranks: Rank16 = hand_ranks(&hand, street);

        let rs = Rank16::from(
            hand.into_iter().map(|c| c.r).collect::<Vec<_>>().as_ref()
                as &[Rank],
        );

        TestResult::from_bool(rs == ranks)
    }
}
