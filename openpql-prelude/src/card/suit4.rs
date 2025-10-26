use super::{
    BitAnd, BitOr, Card64, CardCount, Idx, Suit, Suit4Inner, fmt, ops,
};

#[macro_export]
macro_rules! s4 {
    ($s:expr) => {
        $crate::Suit4::from(
            $s.chars()
                .filter(|c| !c.is_whitespace())
                .map(|c| $crate::Suit::from_char(c).unwrap())
                .collect::<Vec<_>>()
                .as_ref(),
        )
    };
}

/// Bitset representation of suit collections.
///
/// A 4-bit bitset for efficient suit set operations. Each bit represents a specific suit,
/// enabling fast membership tests and set operations.
///
/// # Memory Layout
/// ```text
/// [8, 0]:   xxxxcdhs // x: unused
/// ```
#[derive(
    Copy, Clone, derive_more::Debug, PartialEq, Eq, BitAnd, BitOr, Default,
)]
#[debug("Suit4({})", self)]
pub struct Suit4(pub(crate) Suit4Inner);

impl Suit4 {
    /// Set containing all 4 suits.
    pub const ALL: Self = Self(0b1111);

    /// Returns `true` if the set contains no suits.
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Adds the specified suit to this set.
    #[inline]
    pub const fn set(&mut self, s: Suit) {
        self.0 |= 1 << s as Idx;
    }

    /// Removes the specified suit from this set.
    #[inline]
    pub const fn unset(&mut self, s: Suit) {
        self.0 &= !(1 << s as Idx);
    }

    /// Returns `true` if this set contains the specified suit.
    #[must_use]
    #[inline]
    pub const fn contains_suit(self, s: Suit) -> bool {
        let v = 1 << (s as Idx);
        v == v & self.0
    }

    /// Returns the number of suits in this set.
    #[must_use]
    #[inline]
    pub const fn count(&self) -> CardCount {
        self.0.count_ones().to_le_bytes()[0]
    }
}

impl fmt::Display for Suit4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let suits: String = Suit::ARR_ALL
            .iter()
            .filter(|&s| self.contains_suit(*s))
            .map(|s| s.to_char())
            .collect();

        write!(f, "{suits}")
    }
}

impl From<Suit> for Suit4 {
    fn from(s: Suit) -> Self {
        Self(1 << s as Idx)
    }
}

impl FromIterator<Suit> for Suit4 {
    fn from_iter<T: IntoIterator<Item = Suit>>(iter: T) -> Self {
        let mut res = Self::default();

        for suit in iter {
            res.set(suit);
        }

        res
    }
}

impl From<&[Suit]> for Suit4 {
    fn from(suits: &[Suit]) -> Self {
        suits.iter().copied().collect()
    }
}

impl ops::BitOrAssign<Suit> for Suit4 {
    fn bitor_assign(&mut self, rhs: Suit) {
        self.set(rhs);
    }
}

impl From<Card64> for Suit4 {
    fn from(c: Card64) -> Self {
        let [s, h, d, c] = Suit::ARR_ALL.map(|s| c.count_by_suit(s) > 0);

        Self(
            Suit4Inner::from(s)
                | Suit4Inner::from(h) << 1
                | Suit4Inner::from(d) << 2
                | Suit4Inner::from(c) << 3,
        )
    }
}

impl From<Suit4Inner> for Suit4 {
    fn from(i: Suit4Inner) -> Self {
        Self(i & Self::ALL.0)
    }
}

impl From<Suit4> for Suit4Inner {
    fn from(v: Suit4) -> Self {
        v.0
    }
}

#[cfg(any(test, feature = "quickcheck"))]
impl quickcheck::Arbitrary for Suit4 {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let inner = Suit4Inner::arbitrary(g);

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
        assert_eq!(Suit4::ALL, Suit4(0b1111));
    }

    #[test]
    fn test_empty() {
        assert!(Suit4::default().is_empty());
        assert!(!Suit4(1).is_empty());
    }

    #[quickcheck]
    fn test_set(suit: Suit) {
        let mut res = Suit4::default();
        res.set(suit);

        assert!(res.contains_suit(suit));
    }

    #[quickcheck]
    fn test_unset(suit: Suit) {
        let mut res = Suit4::ALL;
        res.unset(suit);

        assert!(!res.contains_suit(suit));
    }

    #[quickcheck]
    #[allow(clippy::cast_possible_truncation)]
    fn test_count(s4: Suit4) {
        assert_eq!(
            s4.count(),
            Suit::ARR_ALL
                .iter()
                .filter(|&s| s4.contains_suit(*s))
                .count() as CardCount
        );
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{:?}", s4!("ds")), "Suit4(sd)");
    }

    #[quickcheck]
    fn test_bit_and(s1: Suit, s2: Suit) {
        let a = Suit4::from(s1);
        let b = Suit4::from(s2);

        assert_eq!((a & b).is_empty(), s1 != s2);
    }

    #[quickcheck]
    fn test_bit_or(s1: Suit, s2: Suit) {
        let a = Suit4::from(s1);
        let b = Suit4::from(s2);

        assert!((a | b).contains_suit(s1));
        assert!((a | b).contains_suit(s2));

        let mut ab = Suit4::default();
        ab |= s1;
        ab |= s2;
        assert_eq!(ab, a | b);
    }

    #[quickcheck]
    fn test_from_suit(s1: Suit, s2: Suit) {
        let suits = Suit4::from(s1);

        assert!(suits.contains_suit(s1));

        let suits = Suit4::from([s1, s2].as_ref());

        assert!(suits.contains_suit(s1));
        assert!(suits.contains_suit(s2));
    }

    #[quickcheck]
    fn test_from_card64(cards: Vec<Card>) {
        let mut suits = Suit4::default();

        for i in 0..cards.len() {
            suits.set(cards[i].suit);

            let c64: Card64 = cards[0..=i].into();

            assert_eq!(Suit4::from(c64), suits);
        }
    }

    #[quickcheck]
    fn test_from_and_to_int(i: Suit4Inner) {
        let obj = Suit4::from(i);
        let mask = Suit4::ALL.0;

        assert_eq!(obj.0, i & mask);
        assert_eq!(i & mask, Suit4Inner::from(obj));
    }
}
