use super::{
    Card, Card64, CardCount, From, HandIter, Hash, Index, Into, Rank, Vec, fmt,
    iter, mem, slice,
};

/// A hand of N cards
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, From, Into)]
pub struct HandN<const N: usize>(pub(crate) [Card; N]);

impl<const N: usize> HandN<N> {
    /// Creates a new hand from a sorted array of cards
    pub(crate) fn new(array: [Card; N]) -> Self {
        debug_assert!(
            array.is_sorted(),
            "Hand initialized from unsorted array {array:?}"
        );
        Self(array)
    }

    /// Creates a hand from a slice of cards
    pub fn from_slice(cs: &[Card]) -> Self {
        debug_assert!(
            cs.len() >= N,
            "from_slice: not enough cards for Hand<{}> slice has {} elements",
            N,
            cs.len()
        );

        let mut cards = [Card::default(); N];
        cards.copy_from_slice(&cs[..N]);
        cards.sort_unstable();

        Self(cards)
    }

    /// Returns the underlying card array
    pub const fn as_slice(&self) -> &[Card] {
        &self.0
    }

    pub fn to_vec(&self) -> Vec<Card> {
        self.0.to_vec()
    }

    /// Returns an iterator over the cards in the hand
    pub fn iter(&self) -> impl Iterator<Item = Card> + '_ {
        self.0.iter().copied()
    }

    /// Returns the number of cards in the hand
    pub const fn len(&self) -> usize {
        N
    }

    /// Returns true if the hand is empty
    pub const fn is_empty(&self) -> bool {
        N == 0
    }

    pub fn iter_all_short() -> HandIter<true, N> {
        HandIter::default()
    }

    pub fn iter_all() -> HandIter<false, N> {
        HandIter::default()
    }
}

impl HandN<2> {
    pub const fn to_u16(&self) -> u16 {
        (self.0[0].to_u8() as u16) | (self.0[1].to_u8() as u16) << 8
    }

    pub fn from_u16(v: u16) -> Self {
        let [c0, c1] = v.to_le_bytes();
        Self::from_slice(&[Card::from_u8(c0), Card::from_u8(c1)])
    }
}

impl HandN<3> {
    pub(crate) fn count_by_rank(self, rank: Rank) -> CardCount {
        CardCount::from(self[0].rank == rank)
            + CardCount::from(self[1].rank == rank)
            + CardCount::from(self[2].rank == rank)
    }

    pub(crate) fn sorted_ranks(self) -> (Rank, Rank, Rank) {
        let (mut x, mut y, mut z) = (self[0].rank, self[1].rank, self[2].rank);

        if x < y {
            mem::swap(&mut x, &mut y);
        }

        if y < z {
            mem::swap(&mut y, &mut z);
        }

        if x < y {
            mem::swap(&mut x, &mut y);
        }

        (x, y, z)
    }
}

impl<const N: usize> Index<usize> for HandN<N> {
    type Output = Card;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<'a, const N: usize> IntoIterator for &'a HandN<N> {
    type Item = Card;
    type IntoIter = iter::Copied<slice::Iter<'a, Card>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter().copied()
    }
}

impl<const N: usize> fmt::Debug for HandN<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Hand<")?;
        for c in &self.0 {
            write!(f, "{c}")?;
        }
        write!(f, ">")
    }
}

impl<const N: usize> fmt::Display for HandN<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for card in &self.0 {
            write!(f, "{card}")?;
        }

        Ok(())
    }
}

impl<const N: usize> From<HandN<N>> for Card64 {
    fn from(hand: HandN<N>) -> Self {
        let mut card64 = Self::default();
        for card in hand.iter() {
            card64 |= Self::from(card);
        }
        card64
    }
}

impl From<(Card, Card, Card)> for HandN<3> {
    fn from(cs: (Card, Card, Card)) -> Self {
        Self::from_slice(&[cs.0, cs.1, cs.2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_hand() {
        let cards = cards!("2s Kc Ad Kh");
        let [c1, c2, c3, c4] = cards.clone().try_into().unwrap();

        let hand: HandN<3> = HandN::from_slice(&cards);

        assert_eq!(hand.as_slice(), &[c1, c2, c3]);

        let hand: HandN<4> = HandN::from_slice(&cards);

        assert_eq!(hand.as_slice(), &[c1, c4, c2, c3]);
    }

    #[test]
    #[should_panic(expected = "not enough")]
    fn test_hand_n3_not_enough_cards() {
        let cards = cards!("2s Kc");

        let _hand: HandN<3> = HandN::from_slice(&cards);
    }

    // ... existing tests ...

    #[test]
    fn test_hand_iter() {
        let cards = cards!("2s Kc Ad");
        let hand: HandN<3> = HandN::from_slice(&cards);

        let collected: Vec<Card> = hand.iter().collect();
        assert_eq!(collected, cards!("2s Kc Ad"));
    }

    #[test]
    fn test_hand_into_iter() {
        let cards = cards!("2s Kc Ad");
        let hand: HandN<3> = HandN::from_slice(&cards);

        let collected: Vec<Card> = (&hand).into_iter().collect();
        assert_eq!(collected, cards!("2s Kc Ad"));
    }

    #[test]
    fn test_hand_index() {
        let cards = cards!("2s Kc Ad");
        let hand: HandN<3> = HandN::from_slice(&cards);

        assert_eq!(hand[0], cards!("2s")[0]);
        assert_eq!(hand[1], cards!("Kc")[0]);
        assert_eq!(hand[2], cards!("Ad")[0]);
    }

    #[test]
    fn test_hand_len_and_is_empty() {
        let cards = cards!("2s Kc Ad");
        let hand3: HandN<3> = HandN::from_slice(&cards);
        assert_eq!(hand3.len(), 3);
        assert!(!hand3.is_empty());

        let empty_cards = cards!("2s Kc Ad");
        let hand0: HandN<0> = HandN::from_slice(&empty_cards);
        assert_eq!(hand0.len(), 0);
        assert!(hand0.is_empty());
    }

    #[test]
    fn test_hand_debug_and_display() {
        let cards = cards!("2s Kc Ad");
        let hand: HandN<3> = HandN::from_slice(&cards);

        // Test Debug implementation
        let debug_str = format!("{hand:?}");
        assert_eq!(debug_str, "Hand<2sKcAd>");
    }

    #[test]
    fn test_hand_sorting() {
        // Test that cards are sorted when creating a hand
        let unsorted_cards = cards!("Ad Kc 2s");
        let hand: HandN<3> = HandN::from_slice(&unsorted_cards);

        // Cards should be sorted by rank and suit
        assert_eq!(hand[0], cards!("2s")[0]);
        assert_eq!(hand[1], cards!("Kc")[0]);
        assert_eq!(hand[2], cards!("Ad")[0]);
    }

    #[test]
    fn test_hand_equality() {
        let cards1 = cards!("2s Kc Ad");
        let cards2 = cards!("Ad Kc 2s"); // Same cards but different order

        let hand1: HandN<3> = HandN::from_slice(&cards1);
        let hand2: HandN<3> = HandN::from_slice(&cards2);

        // Hands should be equal because they contain the same cards
        // and Hand sorts the cards internally
        assert_eq!(hand1, hand2);

        // Different hands should not be equal
        let cards3 = cards!("2s Kc Ah");
        let hand3: HandN<3> = HandN::from_slice(&cards3);
        assert_ne!(hand1, hand3);
    }

    #[quickcheck]
    fn test_hand2_to_u16((c0, c1): (Card, Card)) {
        if c0 != c1 {
            let hand = HandN::<2>::from_slice(&<[Card; 2]>::from((c0, c1)));
            assert_eq!(hand, HandN::<2>::from_u16(hand.to_u16()));
        }
    }

    #[test]
    #[should_panic(expected = "Hand initialized from unsorted array")]
    #[cfg(debug_assertions)]
    fn test_new_unsorted_array_debug_assert() {
        // Create an unsorted array of cards
        let unsorted = [
            Card::from_str("Ad").unwrap(),
            Card::from_str("2s").unwrap(),
            Card::from_str("Kc").unwrap(),
        ];

        // This should panic in debug mode because the array is not sorted
        let _hand = HandN::new(unsorted);
    }

    #[test]
    #[should_panic(expected = "not enough cards for Hand")]
    #[cfg(debug_assertions)]
    fn test_from_slice_not_enough_cards_debug_assert() {
        let cards = cards!("2s Kc"); // Only 2 cards

        // This should panic in debug mode because we need 3 cards for Hand<3>
        let _hand: HandN<3> = HandN::from_slice(&cards);
    }

    #[test]
    fn test_from_slice_sorts_cards() {
        let unsorted_cards = cards!("Ad Kc 2s");
        let hand: HandN<3> = HandN::from_slice(&unsorted_cards);

        // Verify that the cards are sorted in the resulting hand
        let expected_order = cards!("2s Kc Ad");
        for (i, card) in expected_order.iter().enumerate() {
            assert_eq!(hand[i], *card);
        }
    }
}
