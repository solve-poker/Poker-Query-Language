use super::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, Card64, CardCount, Hash, Idx,
    N_STRAIGHT, N_STRAIGHT_SD, Not, Rank, Rank16Inner, RankIdx, Suit, fmt, ops,
};

#[macro_export]
macro_rules! r16 {
    ($s:expr) => {
        $crate::Rank16::from(
            $s.chars()
                .filter(|c| !c.is_whitespace())
                .map(|c| $crate::Rank::from_char(c).unwrap())
                .collect::<Vec<_>>()
                .as_slice(),
        )
    };
}

/// Bitset representation of rank collections.
///
/// A 16-bit bitset for efficient rank set operations. Each bit represents a specific rank,
/// enabling fast membership tests and set operations.
///
/// # Memory Layout
/// ```text
/// [15, 0]:   xxxAKQJT 98765432  // x: unused
/// ```
#[derive(
    Copy,
    Clone,
    derive_more::Debug,
    PartialEq,
    Eq,
    BitAnd,
    BitOr,
    PartialOrd,
    Ord,
    Hash,
    Default,
    BitOrAssign,
    BitAndAssign,
)]
#[debug("Rank16({})", self)]
pub struct Rank16(pub(crate) Rank16Inner);

impl Rank16 {
    /// Set containing all 13 ranks.
    pub(crate) const ALL: Self = Self(0b0001_1111_1111_1111);
    /// Set containing all 9 short deck ranks (6+).
    pub(crate) const ALL_SD: Self = Self(0b0001_1111_1111_0000);

    pub const STRAIGHT_A6789: Self = Self(0b0001_0000_1111_0000);
    pub const STRAIGHT_A2345: Self = Self(0b0001_0000_0000_1111);
    pub const STRAIGHT_23456: Self = Self(0b0000_0000_0001_1111);
    pub const STRAIGHT_34567: Self = Self(0b0000_0000_0011_1110);
    pub const STRAIGHT_45678: Self = Self(0b0000_0000_0111_1100);
    pub const STRAIGHT_56789: Self = Self(0b0000_0000_1111_1000);
    pub const STRAIGHT_6789T: Self = Self(0b0000_0001_1111_0000);
    pub const STRAIGHT_789TJ: Self = Self(0b0000_0011_1110_0000);
    pub const STRAIGHT_89TJQ: Self = Self(0b0000_0111_1100_0000);
    pub const STRAIGHT_9TJQK: Self = Self(0b0000_1111_1000_0000);
    pub const STRAIGHT_TJQKA: Self = Self(0b0001_1111_0000_0000);

    const ALL_STRAIGHT: [Self; N_STRAIGHT] = [
        Self::STRAIGHT_A2345,
        Self::STRAIGHT_23456,
        Self::STRAIGHT_34567,
        Self::STRAIGHT_45678,
        Self::STRAIGHT_56789,
        Self::STRAIGHT_6789T,
        Self::STRAIGHT_789TJ,
        Self::STRAIGHT_89TJQ,
        Self::STRAIGHT_9TJQK,
        Self::STRAIGHT_TJQKA,
    ];

    const ALL_STRAIGHT_SD: [Self; N_STRAIGHT_SD] = [
        Self::STRAIGHT_A6789,
        Self::STRAIGHT_6789T,
        Self::STRAIGHT_789TJ,
        Self::STRAIGHT_89TJQ,
        Self::STRAIGHT_9TJQK,
        Self::STRAIGHT_TJQKA,
    ];

    #[inline]
    pub const fn all<const SD: bool>() -> Self {
        const { if SD { Self::ALL_SD } else { Self::ALL } }
    }

    #[inline]
    pub const fn all_straights<const SD: bool>() -> &'static [Self] {
        const {
            if SD {
                &Self::ALL_STRAIGHT_SD
            } else {
                &Self::ALL_STRAIGHT
            }
        }
    }

    /// Returns `true` if the set contains no ranks.
    #[must_use]
    #[inline]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Adds the specified rank to this set.
    #[inline]
    pub const fn set(&mut self, r: Rank) {
        self.0 |= 1 << r as Idx;
    }

    /// Removes the specified rank from this set.
    #[inline]
    pub const fn unset(&mut self, r: Rank) {
        self.0 &= !(1 << r as Idx);
    }

    /// Returns `true` if this set contains the specified rank.
    #[must_use]
    #[inline]
    pub const fn contains_rank(self, r: Rank) -> bool {
        let v = 1 << (r as Idx);
        v == v & self.0
    }

    /// Returns the number of ranks in this set.
    #[must_use]
    #[inline]
    pub const fn count(self) -> CardCount {
        self.0.count_ones().to_le_bytes()[0]
    }

    /// Returns the lowest rank in this set, or `None` if empty.
    #[must_use]
    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn min_rank(self) -> Option<Rank> {
        RankIdx(self.0.trailing_zeros() as Idx).to_rank()
    }

    /// Returns the highest rank in this set, or `None` if empty.
    #[must_use]
    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn max_rank(self) -> Option<Rank> {
        const N_ZEROS_R2: Idx = 15;

        RankIdx(N_ZEROS_R2 - (Self::ALL.0 & self.0).leading_zeros() as Idx)
            .to_rank()
    }

    /// Returns the nth highest rank in this set (1-indexed), or `None` if not found.
    #[must_use]
    #[inline]
    pub const fn nth_rank(self, mut n: CardCount) -> Option<Rank> {
        let ranks = Rank::all::<false>();
        let mut i = ranks.len();

        while i > 0 {
            i -= 1;
            let rank = ranks[i];

            if self.contains_rank(rank) {
                if n == 1 {
                    return Some(rank);
                } else if n == 0 {
                    return None;
                }

                n -= 1;
            }
        }

        None
    }

    #[inline]
    pub(crate) const fn from_rank(rank: Rank) -> Self {
        Self(1 << rank as Idx)
    }
}

impl fmt::Display for Rank16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ranks: String = Rank::all::<false>()
            .iter()
            .filter(|&r| self.contains_rank(*r))
            .map(|r| r.to_char())
            .collect();

        write!(f, "{ranks}")
    }
}

impl Not for Rank16 {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0 & Self::ALL.0)
    }
}

impl From<Rank> for Rank16 {
    fn from(r: Rank) -> Self {
        Self(1 << r as Idx)
    }
}

impl FromIterator<Rank> for Rank16 {
    fn from_iter<T: IntoIterator<Item = Rank>>(iter: T) -> Self {
        let mut res = Self::default();

        for rank in iter {
            res.set(rank);
        }

        res
    }
}

impl From<&[Rank]> for Rank16 {
    fn from(ranks: &[Rank]) -> Self {
        ranks.iter().copied().collect()
    }
}

impl From<Card64> for Rank16 {
    fn from(c: Card64) -> Self {
        c.ranks_by_suit(Suit::S)
            | c.ranks_by_suit(Suit::H)
            | c.ranks_by_suit(Suit::D)
            | c.ranks_by_suit(Suit::C)
    }
}

impl From<Rank16Inner> for Rank16 {
    fn from(i: Rank16Inner) -> Self {
        Self(i & Self::ALL.0)
    }
}

impl From<Rank16> for Rank16Inner {
    fn from(v: Rank16) -> Self {
        v.0
    }
}

impl ops::BitOrAssign<Rank> for Rank16 {
    fn bitor_assign(&mut self, rhs: Rank) {
        self.set(rhs);
    }
}

#[cfg(any(test, feature = "quickcheck"))]
impl quickcheck::Arbitrary for Rank16 {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let inner = Rank16Inner::arbitrary(g);

        Self(Self::ALL.0 & inner)
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_all(rank: Rank) {
        if rank >= Rank::R6 {
            assert!(Rank16::all::<true>().contains_rank(rank));
        }
        assert!(Rank16::all::<false>().contains_rank(rank));
    }

    #[test]
    fn test_all_straights() {
        for s in ["23456", "34567", "45678", "56789"] {
            assert!(Rank16::all_straights::<false>().contains(&r16!(s)));
        }
        for s in ["6789T", "789TJ", "89TJQ", "9TJQK", "TJQKA"] {
            assert!(Rank16::all_straights::<false>().contains(&r16!(s)));
            assert!(Rank16::all_straights::<true>().contains(&r16!(s)));
        }

        assert!(Rank16::all_straights::<false>().contains(&r16!("A2345")));
        assert!(Rank16::all_straights::<true>().contains(&r16!("A6789")));
    }

    #[test]
    fn test_empty() {
        assert!(Rank16::default().is_empty());
        assert!(!Rank16::all::<false>().is_empty());
    }

    #[quickcheck]
    fn test_set(rank: Rank) {
        let mut res = Rank16::default();
        res.set(rank);

        assert!(res.contains_rank(rank));
    }

    #[quickcheck]
    fn test_unset(rank: Rank) {
        let mut res = Rank16::all::<false>();
        res.unset(rank);

        assert!(!res.contains_rank(rank));
    }

    #[quickcheck]
    #[allow(clippy::cast_possible_truncation)]
    fn test_count(r16: Rank16) {
        assert_eq!(
            r16.count(),
            Rank::all::<false>()
                .iter()
                .filter(|&r| r16.contains_rank(*r))
                .count() as CardCount
        );
    }

    #[quickcheck]
    fn test_min_rank_and_max_rank(ranks: Distinct<5, Rank>) {
        let r16 = Rank16::from(ranks.as_slice());

        let max = ranks.iter().max().copied();
        let min = ranks.iter().min().copied();

        assert_eq!(r16.max_rank(), max);
        assert_eq!(r16.min_rank(), min);
    }

    #[test]
    fn test_nth_rank() {
        let ranks = r16!("26K");

        assert_eq!(ranks.nth_rank(0), None);
        assert_eq!(ranks.nth_rank(1), Some(Rank::RK));
        assert_eq!(ranks.nth_rank(2), Some(Rank::R6));
        assert_eq!(ranks.nth_rank(3), Some(Rank::R2));
        assert_eq!(ranks.nth_rank(4), None);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{:?}", r16!("ATJ")), "Rank16(TJA)");
    }

    #[quickcheck]
    fn test_bit_not(r16: Rank16) {
        assert_eq!((!r16).0, Rank16::ALL.0 & !(r16.0));
    }

    #[quickcheck]
    fn test_bit_and(r1: Rank, r2: Rank) {
        let a = Rank16::from(r1);
        let b = Rank16::from(r2);

        assert_eq!((a & b).is_empty(), r1 != r2);
    }

    #[quickcheck]
    fn test_bit_or(r1: Rank, r2: Rank) {
        let a = Rank16::from(r1);
        let b = Rank16::from(r2);

        assert!((a | b).contains_rank(r1));
        assert!((a | b).contains_rank(r2));

        let mut ab = Rank16::default();
        ab |= r1;
        ab |= r2;
        assert_eq!(ab, a | b);
    }

    #[quickcheck]
    fn test_from_cards_and_card64(cards: Vec<Card>) {
        let mut ranks = Rank16::default();

        for i in 0..cards.len() {
            ranks.set(cards[i].rank);

            let c64 = Card64::from(&cards[0..=i]);

            assert_eq!(Rank16::from(c64), ranks);
        }
    }

    #[quickcheck]
    fn test_from_and_to_int(i: Rank16Inner) {
        let obj = Rank16::from(i);
        let mask = Rank16::ALL.0;

        assert_eq!(obj.0, i & mask);
        assert_eq!(i & mask, Rank16Inner::from(obj));
    }
}
