use super::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, Card, Card64Inner, CardCount,
    CardIdx, CardIter, Hash, Idx, Not, Rank, Rank16, Rank16Inner, Suit, fmt,
    ops,
};

#[macro_export]
macro_rules! c64 {
    ($s:expr) => {
        $crate::Card64::from($crate::cards![$s].as_ref())
    };
}

/// Bitset representation of card collections.
///
/// A 64-bit bitset for efficient card set operations. Each bit represents a specific card,
/// enabling fast membership tests and set operations.
///
/// # Memory Layout
/// ```text
/// [63, 48]:  xxxAKQJT 98765432  // Club
/// [47, 32]:  xxxAKQJT 98765432  // Diamond
/// [31, 16]:  xxxAKQJT 98765432  // Heart
/// [15, 0]:   xxxAKQJT 98765432  // Spade, x: unused
/// ```
#[derive(
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    BitAnd,
    BitAndAssign,
    BitOr,
    BitOrAssign,
    Default,
)]
pub struct Card64(pub(crate) Card64Inner);

impl Card64 {
    const OFFSET_SUIT: Idx = 16;

    pub(crate) const EMPTY: Self = Self(0);

    const ALL: Self = sealed::all::<false>();
    const ALL_SD: Self = sealed::all::<true>();

    #[must_use]
    #[inline]
    pub const fn all<const SD: bool>() -> Self {
        const { if SD { Self::ALL_SD } else { Self::ALL } }
    }

    /// Returns `true` if the set contains no cards.
    #[must_use]
    #[inline]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Adds the specified card to this set.
    #[inline]
    pub const fn set(&mut self, card: Card) {
        self.0 |= Self::from_card(card).0;
    }

    /// Removes the specified card from this set.
    #[inline]
    pub const fn unset(&mut self, card: Card) {
        self.0 &= !Self::from_card(card).0;
    }

    /// Returns `true` if this set contains all cards in `other`.
    #[must_use]
    #[inline]
    pub fn contains(self, other: Self) -> bool {
        other & self == other
    }

    /// Returns `true` if this set contains the specified card.
    #[must_use]
    #[inline]
    pub const fn contains_card(self, card: Card) -> bool {
        let v = Self::from_card(card).0;
        v & self.0 == v
    }

    /// Returns the number of cards in this set.
    #[must_use]
    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn count(&self) -> CardCount {
        self.0.count_ones() as CardCount
    }

    /// Returns the number of cards with the specified rank.
    #[must_use]
    #[inline]
    pub const fn count_by_rank(self, rank: Rank) -> CardCount {
        Self(self.0 & Self::from_ranks(Rank16::from_rank(rank)).0).count()
    }

    /// Returns the number of cards with the specified suit.
    #[must_use]
    #[inline]
    pub const fn count_by_suit(self, suit: Suit) -> CardCount {
        self.ranks_by_suit(suit).count()
    }

    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    pub(crate) const fn ranks_by_suit(self, suit: Suit) -> Rank16 {
        Rank16((self.0 >> (Self::OFFSET_SUIT * suit as Idx)) as Rank16Inner)
    }

    /// Creates a card set containing all specified ranks in all suits.
    #[inline]
    #[must_use]
    pub const fn from_ranks(rs: Rank16) -> Self {
        let [l, h] = rs.0.to_le_bytes();

        Self(Card64Inner::from_le_bytes([l, h, l, h, l, h, l, h]))
    }

    /// Creates a card set containing all ranks in a specified suit.
    #[inline]
    #[must_use]
    pub const fn from_suit(suit: Suit) -> Self {
        let v = Rank16::all::<false>().0 as Card64Inner;

        Self(v << (Self::OFFSET_SUIT * (suit as Idx)))
    }

    /// Returns the set of ranks present in this card set.
    #[inline]
    #[must_use]
    pub const fn ranks(self) -> Rank16 {
        Rank16(
            self.ranks_by_suit(Suit::S).0
                | self.ranks_by_suit(Suit::H).0
                | self.ranks_by_suit(Suit::D).0
                | self.ranks_by_suit(Suit::C).0,
        )
    }

    /// Returns an iterator over all cards in this set.
    pub const fn iter(self) -> CardIter {
        CardIter::new(self)
    }

    const fn from_indices(r: Idx, s: Idx) -> Self {
        Self(1 << r << (s * Self::OFFSET_SUIT))
    }

    const fn from_card(card: Card) -> Self {
        Self::from_indices(card.rank as Idx, card.suit as Idx)
    }
}

// compiler-time functions
#[allow(clippy::cast_possible_wrap)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod sealed {
    use super::{Card, Card64, CardIdx, Idx};

    pub(super) const fn all<const SD: bool>() -> Card64 {
        let start_idx = if SD {
            Card::N_CARDS - Card::N_CARDS_SD
        } else {
            0
        };

        let mut res = Card64::EMPTY;
        let mut i = 0;

        while i < Card::N_CARDS {
            if i >= start_idx {
                res.set(CardIdx(i as Idx).to_card().unwrap());
            }
            i += 1;
        }

        res
    }
}

impl Not for Card64 {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0 & Self::ALL.0)
    }
}

impl fmt::Debug for Card64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn ranks_str(r16: Rank16) -> String {
            if r16.is_empty() {
                "_".to_string()
            } else {
                r16.to_string()
            }
        }

        if self.count() == 1 {
            write!(f, "Card64<{}>", self.iter().next().unwrap())
        } else {
            f.debug_tuple("Card64")
                .field(&format_args!(
                    "{}",
                    ranks_str(self.ranks_by_suit(Suit::S))
                ))
                .field(&format_args!(
                    "{}",
                    ranks_str(self.ranks_by_suit(Suit::H))
                ))
                .field(&format_args!(
                    "{}",
                    ranks_str(self.ranks_by_suit(Suit::D))
                ))
                .field(&format_args!(
                    "{}",
                    ranks_str(self.ranks_by_suit(Suit::C))
                ))
                .finish()
        }
    }
}

impl From<Card> for Card64 {
    fn from(c: Card) -> Self {
        Self::from_card(c)
    }
}

impl From<Card64Inner> for Card64 {
    fn from(i: Card64Inner) -> Self {
        Self(i & Self::ALL.0)
    }
}

impl From<Card64> for Card64Inner {
    fn from(v: Card64) -> Self {
        v.0
    }
}

impl FromIterator<Card> for Card64 {
    fn from_iter<T: IntoIterator<Item = Card>>(iter: T) -> Self {
        let mut res = Self::default();

        for card in iter {
            res.set(card);
        }

        res
    }
}

impl From<&[Card]> for Card64 {
    fn from(cards: &[Card]) -> Self {
        cards.iter().copied().collect()
    }
}

impl ops::BitOrAssign<Card> for Card64 {
    fn bitor_assign(&mut self, rhs: Card) {
        self.set(rhs);
    }
}

#[cfg(any(test, feature = "quickcheck"))]
impl quickcheck::Arbitrary for Card64 {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let inner = Card64Inner::arbitrary(g);

        Self(Self::ALL.0 & inner)
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_all() {
        for c in Card::all::<false>() {
            if c.rank >= Rank::R6 {
                assert!(Card64::all::<true>().contains_card(*c));
            }
            assert!(Card64::all::<false>().contains_card(*c));
        }
    }

    #[test]
    fn test_empty() {
        assert!(Card64::default().is_empty());
        assert!(!Card64::all::<false>().is_empty());
    }

    #[quickcheck]
    fn test_set(card: Card) {
        let mut res = Card64::default();
        res.set(card);

        assert!(res.contains_card(card));
    }

    #[quickcheck]
    fn test_unset(card: Card) {
        let mut res = Card64::all::<false>();
        res.unset(card);

        assert!(!res.contains_card(card));
    }

    #[quickcheck]
    fn test_contains(cards: Vec<Card>) {
        let all = Card64::from(cards.as_slice());
        let half = Card64::from(&cards[..cards.len() / 2]);

        assert!(all.contains(half));
    }

    #[quickcheck]
    #[allow(clippy::cast_possible_truncation)]
    fn test_count(c64: Card64) {
        assert_eq!(
            c64.count(),
            Card::all::<false>()
                .iter()
                .filter(|&c| c64.contains_card(*c))
                .count() as CardCount
        );
    }

    #[quickcheck]
    #[allow(clippy::cast_possible_truncation)]
    fn test_count_by_rank(c64: Card64, rank: Rank) {
        assert_eq!(
            c64.count_by_rank(rank),
            Card::all::<false>()
                .iter()
                .filter(|&c| c64.contains_card(*c) && c.rank == rank)
                .count() as CardCount
        );
    }

    #[quickcheck]
    #[allow(clippy::cast_possible_truncation)]
    fn test_count_by_suit(c64: Card64, suit: Suit) {
        assert_eq!(
            c64.count_by_suit(suit),
            Card::all::<false>()
                .iter()
                .filter(|&c| c64.contains_card(*c) && c.suit == suit)
                .count() as CardCount
        );
    }

    #[quickcheck]
    fn test_ranks_by_suit(c64: Card64, suit: Suit) {
        let expected = Rank::all::<false>()
            .iter()
            .filter(|&&r| c64.contains_card(Card::new(r, suit)))
            .copied()
            .collect();

        assert_eq!(c64.ranks_by_suit(suit), expected);
    }

    #[quickcheck]
    fn test_from_ranks(r16: Rank16) {
        let expected = Card::all::<false>()
            .iter()
            .filter(|c| r16.contains_rank(c.rank))
            .copied()
            .collect();

        assert_eq!(Card64::from_ranks(r16), expected);
    }

    #[quickcheck]
    fn test_from_suit(suit: Suit) {
        let expected = Card::all::<false>()
            .iter()
            .filter(|c| c.suit == suit)
            .copied()
            .collect();

        assert_eq!(Card64::from_suit(suit), expected);
    }

    #[quickcheck]
    fn test_ranks(c64: Card64) {
        let expected = Card::all::<false>()
            .iter()
            .filter(|&&c| c64.contains_card(c))
            .map(|c| c.rank)
            .collect();

        assert_eq!(c64.ranks(), expected);
    }

    #[test]
    fn test_iter() {
        fn assert_iter(s: &str) {
            let card64 = c64!(s);
            let v = cards!(s);

            for c in card64.iter() {
                assert!(v.contains(&c));
            }

            assert_eq!(card64.iter().count(), v.len());
        }

        assert_iter("");
        assert_iter("As");
        assert_iter("As Kh 2d");
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{:?}", c64!("As")), "Card64<As>");
        assert_eq!(format!("{:?}", c64!("As 9h")), "Card64(A, 9, _, _)");
    }

    #[quickcheck]
    fn test_bit_not(c64: Card64) {
        assert_eq!((!c64).0, Card64::ALL.0 & !(c64.0));
    }

    #[quickcheck]
    fn test_bit_and(c1: Card, c2: Card) {
        let a = Card64::from(c1);
        let b = Card64::from(c2);

        assert_eq!((a & b).is_empty(), c1 != c2);
    }

    #[quickcheck]
    fn test_bit_or(c1: Card, c2: Card) {
        let a = Card64::from(c1);
        let b = Card64::from(c2);

        assert!((a | b).contains_card(c1));
        assert!((a | b).contains_card(c2));

        let mut ab = Card64::default();
        ab |= c1;
        ab |= c2;
        assert_eq!(ab, a | b);
    }

    #[quickcheck]
    fn test_from_and_to_int(i: Card64Inner) {
        let obj = Card64::from(i);
        let mask = Card64::ALL.0;

        assert_eq!(obj.0, i & mask);
        assert_eq!(i & mask, Card64Inner::from(obj));
    }
}
