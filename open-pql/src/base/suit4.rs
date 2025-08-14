use super::{
    BitAnd, BitOr, Card64, N_SUITS, PQLCardCount, SUIT_NAMES, Suit, fmt,
};

#[cfg(any(test, feature = "benchmark"))]
#[macro_export]
macro_rules! s4 {
    ($s:expr) => {
        $crate::Suit4::from(
            $s.chars()
                .filter(|c| !c.is_whitespace())
                .map(|c| $crate::Suit::try_from(c).unwrap())
                .collect::<Vec<_>>()
                .as_ref(),
        )
    };
}

/// Suit Masks
///
/// A compact bit-set representation for storing multiple suits using a single u8.
/// Each bit represents whether a specific suit is present in the set.
/// Supports efficient set operations like union, intersection, and membership testing.
///
/// # Memory Layout:
/// ```text
/// [8, 0]:   xxxxCDHS // x: unused
/// ```
///
/// # Examples
///
/// ```
/// use open_pql::{Suit::*, Suit4};
///
/// let mut suits = Suit4::default();
/// suits.set(S);
/// suits.set(H);
///
/// assert!(suits.contains_suit(S));
/// assert_eq!(suits.count(), 2);
/// ```
#[derive(Copy, Clone, PartialEq, Eq, BitAnd, BitOr, Default)]
pub struct Suit4(u8);

impl Suit4 {
    /// Constructs [Suit4] from u8 representation [xxxxxxSS]
    #[must_use]
    #[inline]
    pub(crate) const fn from_u8(v: u8) -> Self {
        Self(v)
    }

    /// Returns the inner [u8]
    #[allow(unused)]
    #[must_use]
    #[inline]
    pub(crate) const fn to_u8(self) -> u8 {
        self.0
    }

    #[inline]
    const fn from_suit(s: Suit) -> Self {
        Self(1 << s as u8)
    }

    /// Checks whether all suit masks are unset
    ///
    /// # Examples
    ///
    /// ```
    /// use open_pql::{Suit, Suit4};
    ///
    /// let empty: Suit4 = Suit4::default();
    /// let not_empty: Suit4 = Suit4::from(Suit::D);
    ///
    /// assert!(empty.is_empty());
    /// assert!(!not_empty.is_empty());
    /// ```
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Mark a [Suit]
    ///
    /// # Examples
    ///
    /// ```
    /// use open_pql::{Suit, Suit4};
    ///
    /// let mut suits: Suit4 = Suit4::default();
    /// suits.set(Suit::D);
    ///
    /// assert_eq!(suits, Suit4::from(Suit::D));
    /// ```
    #[inline]
    pub const fn set(&mut self, s: Suit) {
        self.0 |= 1 << s as u8;
    }

    /// Unmark a [Suit]
    ///
    /// # Examples
    ///
    /// ```
    /// use open_pql::{Suit, Suit4};
    ///
    /// let mut suits: Suit4 = Suit4::from(Suit::D);
    /// suits.unset(Suit::D);
    ///
    /// assert_eq!(suits, Suit4::default());
    /// ```
    #[inline]
    pub const fn unset(&mut self, s: Suit) {
        self.0 &= !(1 << s as u8);
    }

    /// Checks whether a [Suit] is marked
    ///
    /// # Examples
    ///
    /// ```
    /// use open_pql::{Suit, Suit4};
    ///
    /// let mut suits: Suit4 = Suit4::from(Suit::D);
    ///
    /// assert!(suits.contains_suit(Suit::D));
    /// assert!(!suits.contains_suit(Suit::H));
    /// ```
    #[must_use]
    #[inline]
    pub const fn contains_suit(self, s: Suit) -> bool {
        let v = 1u8 << (s as u8);
        v == v & self.0
    }

    /// Returns the number of marked suits
    ///
    /// # Examples
    ///
    /// ```
    /// use open_pql::{Suit, Suit4};
    ///
    /// let mut suits: Suit4 = Suit4::from(Suit::D);
    ///
    /// assert_eq!(suits.count(), 1);
    /// ```
    #[must_use]
    #[inline]
    pub const fn count(&self) -> PQLCardCount {
        self.0.count_ones().to_le_bytes()[0]
    }
}

impl From<Suit> for Suit4 {
    fn from(s: Suit) -> Self {
        Self::from_suit(s)
    }
}

impl From<&[Suit]> for Suit4 {
    fn from(ss: &[Suit]) -> Self {
        let mut res = Self::default();

        for s in ss {
            res.set(*s);
        }

        res
    }
}

/// Converts a u8 bitmask to a string representation of suits
pub fn u8_to_suit_str(v: u8) -> String {
    let to_c = |i: u8| {
        if v & 1 << i == 0 {
            '\0'
        } else {
            SUIT_NAMES[i as usize]
        }
    };

    (0..N_SUITS)
        .map(to_c)
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
}

impl fmt::Debug for Suit4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Suit4")
            .field(&format_args!("{}", u8_to_suit_str(self.0)))
            .finish()
    }
}

impl From<Card64> for Suit4 {
    fn from(c: Card64) -> Self {
        // TODO: refactor

        let [s, h, d, c] = Suit::ARR_ALL.map(|s| c.count_by_suit(s) > 0);

        Self(
            u8::from(s)
                | u8::from(h) << 1
                | u8::from(d) << 2
                | u8::from(c) << 3,
        )
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_empty() {
        assert_eq!(Suit4::default(), Suit4(0));
        assert_eq!(Suit4::default(), Suit4(0));
        assert!(Suit4::default().is_empty());
        assert!(Suit4::default().is_empty());
        assert!(!Suit4(1).is_empty());
    }

    #[quickcheck]
    fn test_set_and_contains(s: Suit) {
        let mut suits = Suit4::default();

        suits.set(s);

        assert!(!suits.is_empty());
        assert!(suits.contains_suit(s));

        suits.unset(s);

        assert!(suits.is_empty());
        assert!(!suits.contains_suit(s));
    }

    #[quickcheck]
    fn test_u8(i: u8) -> TestResult {
        if i > 0b1111 {
            return TestResult::discard();
        }

        assert_eq!(Suit4(i), Suit4::from_u8(i));
        assert_eq!(i, Suit4(i).to_u8());

        TestResult::passed()
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
    }

    #[quickcheck]
    fn test_count(s1: Suit, s2: Suit) {
        let suits = Suit4::from([s1, s2].as_ref());

        let count = if s1 == s2 { 1 } else { 2 };

        assert_eq!(count, suits.count());
    }

    #[quickcheck]
    fn test_from_card64(cards: Vec<Card>) -> TestResult {
        let mut suits = Suit4::default();

        for i in 0..cards.len() {
            suits.set(cards[i].suit);

            let c64: Card64 = cards[0..=i].into();

            assert_eq!(Suit4::from(c64), suits);
        }

        TestResult::passed()
    }

    #[test]
    fn test_debug() {
        let s = format!("{:?}", s4!("s") | s4!("d"));
        assert_eq!(s, "Suit4(sd)");
    }

    #[test]
    #[cfg(debug_assertions)]
    fn test_suit_from_u8_debug_assert() {
        use std::panic;

        for i in 0..N_SUITS {
            let result = panic::catch_unwind(|| {
                Suit::from_u8(i);
            });
            assert!(result.is_ok(), "Suit::from_u8({i}) should not panic");
        }

        let result = panic::catch_unwind(|| {
            Suit::from_u8(N_SUITS);
        });
        assert!(result.is_err(), "Suit::from_u8({N_SUITS}) should panic");

        let result = panic::catch_unwind(|| {
            Suit::from_u8(100);
        });
        assert!(result.is_err(), "Suit::from_u8(100) should panic");
    }
}
