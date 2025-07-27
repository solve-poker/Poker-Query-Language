use super::*;

#[cfg(any(test, feature = "benchmark"))]
#[macro_export]
macro_rules! r16 {
    ($s:expr) => {
        $crate::Rank16::from(
            $s.chars()
                .filter(|c| !c.is_whitespace())
                .map(|c| $crate::Rank::try_from(c).unwrap())
                .collect::<Vec<_>>()
                .as_ref() as &[Rank],
        )
    };
}

/// Rank Set
/// # Memory Layout:
/// ```text
/// [15, 0]:   xxxAKQJT 98765432  // x: unused
/// ```
#[derive(
    Copy,
    Clone,
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
pub struct Rank16(u16);

impl Rank16 {
    /// Constructs [Rank16] from [u16]
    ///
    /// # Examples
    ///
    /// ```
    /// use open_pql::{Rank, Rank16};
    ///
    /// let i: u16 = 0b0011;
    /// let ranks: Rank16 = Rank16::from_u16(i);
    ///
    /// assert_eq!(ranks, Rank16::from([Rank::R2, Rank::R3].as_ref()));
    /// ```
    #[must_use]
    #[inline]
    pub const fn from_u16(v: u16) -> Self {
        Self(v)
    }

    /// Returns the inner [u16]
    ///
    /// # Examples
    ///
    /// ```
    /// use open_pql::Rank16;
    ///
    /// let i: u16 = 0b0011;
    /// let ranks: Rank16 = Rank16::from_u16(i);
    ///
    /// assert_eq!(i, ranks.to_u16());
    /// ```
    #[must_use]
    #[inline]
    pub const fn to_u16(self) -> u16 {
        self.0
    }

    #[inline]
    const fn from_rank(r: Rank) -> Self {
        Self(1 << r as u8)
    }

    /// Constructs an empty [Rank16]
    ///
    /// # Examples
    ///
    /// ```
    /// use open_pql::{Card, Rank16};
    ///
    /// let ranks: Rank16 = Rank16::empty();
    /// let cards: [Card; 0] = [];
    ///
    /// assert_eq!(ranks, Rank16::from(cards.as_ref()));
    /// ```
    #[must_use]
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }

    /// Checks whether all rank masks are unset
    ///
    /// # Examples
    ///
    /// ```
    /// use open_pql::{Rank, Rank16};
    ///
    /// let ranks: Rank16 = Rank16::from(Rank::RA);
    ///
    /// assert!(!ranks.is_empty());
    /// ```
    #[must_use]
    #[inline]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Mark a [Rank]
    ///
    /// # Examples
    ///
    /// ```
    /// use open_pql::{Rank, Rank16};
    ///
    /// let mut ranks: Rank16 = Rank16::empty();
    /// ranks.set(Rank::RA);
    ///
    /// assert_eq!(ranks, Rank16::from(Rank::RA));
    /// ```
    #[inline]
    pub fn set(&mut self, r: Rank) {
        self.0 |= 1 << r as i8;
    }

    /// Unmark a [Rank]
    ///
    /// # Examples
    ///
    /// ```
    /// use open_pql::{Rank, Rank16};
    ///
    /// let mut ranks: Rank16 = Rank16::from(Rank::RA);
    /// ranks.unset(Rank::RA);
    ///
    /// assert_eq!(ranks, Rank16::empty());
    /// ```
    #[inline]
    pub fn unset(&mut self, r: Rank) {
        self.0 &= !(1 << r as i8);
    }

    /// Checks whether a [Rank] is marked
    ///
    /// # Examples
    ///
    /// ```
    /// use open_pql::{Rank, Rank16};
    ///
    /// let ranks: Rank16 = Rank16::from(Rank::RA);
    ///
    /// assert!(ranks.contains_rank(Rank::RA));
    /// assert!(!ranks.contains_rank(Rank::RK));
    /// ```
    #[must_use]
    #[inline]
    pub const fn contains_rank(self, r: Rank) -> bool {
        let v = 1u16 << (r as usize);
        v == v & self.0
    }

    /// Returns the number of marked ranks
    ///
    /// # Examples
    ///
    /// ```
    /// use open_pql::{Rank, Rank16};
    ///
    /// let ranks: Rank16 = Rank16::from(Rank::RA);
    ///
    /// assert_eq!(ranks.count(), 1);
    /// ```
    #[must_use]
    #[inline]
    pub const fn count(self) -> PQLCardCount {
        self.0.count_ones().to_le_bytes()[0]
    }

    /// Returns smallest Rank in the set
    #[must_use]
    #[inline]
    pub const fn min_rank(self) -> Option<Rank> {
        match self.0.trailing_zeros() {
            0 => Some(Rank::R2),
            1 => Some(Rank::R3),
            2 => Some(Rank::R4),
            3 => Some(Rank::R5),
            4 => Some(Rank::R6),
            5 => Some(Rank::R7),
            6 => Some(Rank::R8),
            7 => Some(Rank::R9),
            8 => Some(Rank::RT),
            9 => Some(Rank::RJ),
            10 => Some(Rank::RQ),
            11 => Some(Rank::RK),
            12 => Some(Rank::RA),
            _ => None,
        }
    }

    /// Returns largest Rank in the set
    #[must_use]
    #[inline]
    pub const fn max_rank(self) -> Option<Rank> {
        match (MASK16_RANKS & self.0).leading_zeros() {
            15 => Some(Rank::R2),
            14 => Some(Rank::R3),
            13 => Some(Rank::R4),
            12 => Some(Rank::R5),
            11 => Some(Rank::R6),
            10 => Some(Rank::R7),
            9 => Some(Rank::R8),
            8 => Some(Rank::R9),
            7 => Some(Rank::RT),
            6 => Some(Rank::RJ),
            5 => Some(Rank::RQ),
            4 => Some(Rank::RK),
            3 => Some(Rank::RA),
            _ => None,
        }
    }

    /// Returns n-th Rank in the set
    #[must_use]
    #[inline]
    pub fn nth_rank(self, mut n: u8) -> Option<Rank> {
        for rank in Rank::ARR_ALL.iter().rev() {
            if self.contains_rank(*rank) {
                if n == 1 {
                    return Some(*rank);
                } else if n == 0 {
                    return None;
                }

                n -= 1;
            }
        }

        None
    }

    /// Returns all higher ranks than the max rank in the set
    #[must_use]
    #[inline]
    pub const fn higher_of(r: Self) -> Self {
        if r.is_empty() {
            Self(MASK16_RANKS)
        } else {
            let i = r.to_u16().leading_zeros();

            Self(!((U16_LEADING_ONE >> i) * 2 - 1) & MASK16_RANKS)
        }
    }
}

impl Not for Rank16 {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0 & MASK16_RANKS)
    }
}

impl From<Rank> for Rank16 {
    fn from(r: Rank) -> Self {
        Self::from_rank(r)
    }
}

impl From<&[Rank]> for Rank16 {
    fn from(rs: &[Rank]) -> Self {
        let mut res = Self::empty();

        for r in rs {
            res.set(*r);
        }

        res
    }
}

impl From<&[Card]> for Rank16 {
    fn from(cs: &[Card]) -> Self {
        let mut res = Self::empty();

        for c in cs {
            res.set(c.r);
        }

        res
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

pub fn u16_to_rank_str(v: u16) -> String {
    let to_c = |i: u8| {
        if v & 1 << i == 0 {
            '\0'
        } else {
            RANK_NAMES[i as usize]
        }
    };

    (0..N_RANKS)
        .map(to_c)
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
}

impl fmt::Debug for Rank16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("Rank16<{}>", u16_to_rank_str(self.0)))
    }
}

#[cfg(test)]
mod tests {
    use constants::MASK16_RANKS;
    use quickcheck::{Arbitrary, TestResult};

    use super::*;

    impl Arbitrary for Rank16 {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let inner = u16::arbitrary(g);

            Self(MASK16_RANKS & inner)
        }
    }

    #[test]
    fn test_empty() {
        assert_eq!(Rank16::empty(), Rank16(0));
        assert!(Rank16::empty().is_empty());
        assert!(!Rank16(1).is_empty());
    }

    #[quickcheck]
    fn test_set_and_contains(r: Rank) {
        let mut ranks = Rank16::empty();

        ranks.set(r);

        assert!(!ranks.is_empty());
        assert!(ranks.contains_rank(r));

        ranks.unset(r);

        assert!(ranks.is_empty());
        assert!(!ranks.contains_rank(r));
    }

    #[quickcheck]
    fn test_u16(i: u16) -> TestResult {
        if i > 0b1_1111_1111_1111 {
            return TestResult::discard();
        }

        assert_eq!(Rank16(i), Rank16::from_u16(i));
        assert_eq!(i, Rank16(i).to_u16());

        TestResult::passed()
    }

    #[quickcheck]
    fn test_from_rank(r1: Rank, r2: Rank) {
        let ranks = Rank16::from(r1);

        assert!(ranks.contains_rank(r1));

        let ranks = Rank16::from([r1, r2].as_ref());

        assert!(ranks.contains_rank(r1));
        assert!(ranks.contains_rank(r2));
    }

    #[test]
    fn test_bit_not() {
        assert_eq!((!Rank16::default()).to_u16(), MASK16_RANKS);
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
    }

    #[quickcheck]
    fn test_count(r1: Rank, r2: Rank) {
        let ranks = Rank16::from([r1, r2].as_ref());

        let count = if r1 == r2 { 1 } else { 2 };

        assert_eq!(count, ranks.count());
    }

    #[quickcheck]
    fn test_min_max_rank(cards: CardN<3>) {
        let ranks = Rank16::from(cards.as_ref());

        let min_r = ranks.min_rank().unwrap();
        let max_r = ranks.max_rank().unwrap();

        let min_i = cards.as_ref().iter().map(|c| c.r as usize).min().unwrap();
        let max_i = cards.as_ref().iter().map(|c| c.r as usize).max().unwrap();

        for c in cards {
            let r = Rank16::from(&[c] as &[Card]);
            assert_eq!(c.r, r.min_rank().unwrap());
            assert_eq!(c.r, r.max_rank().unwrap());
        }

        assert_eq!(min_i, min_r as usize);
        assert_eq!(max_i, max_r as usize);
        assert_eq!(None, Rank16::empty().min_rank());
        assert_eq!(None, Rank16::empty().max_rank());
    }

    #[test]
    fn test_nth_rank() {
        let ranks = r16!("256KA");

        assert_eq!(ranks.nth_rank(0), None);
        assert_eq!(ranks.nth_rank(1), Some(Rank::RA));
        assert_eq!(ranks.nth_rank(2), Some(Rank::RK));
        assert_eq!(ranks.nth_rank(3), Some(Rank::R6));
        assert_eq!(ranks.nth_rank(4), Some(Rank::R5));
        assert_eq!(ranks.nth_rank(5), Some(Rank::R2));
        assert_eq!(ranks.nth_rank(6), None);
    }

    #[test]
    fn test_higher_of() {
        assert_eq!(r16!("KA"), Rank16::higher_of(r16!("2Q")));
        assert_eq!(!Rank16::default(), Rank16::higher_of(r16!("")));
        assert_eq!(Rank16::default(), Rank16::higher_of(r16!("A")));
    }

    #[quickcheck]
    fn test_from_card64(cards: Vec<Card>) -> TestResult {
        let mut ranks = Rank16::empty();

        for i in 0..cards.len() {
            ranks.set(cards[i].r);

            let c64: Card64 = cards[0..=i].into();

            assert_eq!(Rank16::from(c64), ranks);
        }

        TestResult::passed()
    }

    #[test]
    fn test_debug() {
        let s = format!("{:?}", r16!("J") | r16!("9"));
        assert_eq!(s, "Rank16<9J>");
    }
}
