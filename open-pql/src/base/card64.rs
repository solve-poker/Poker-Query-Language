use super::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, Card, CardIter, Hash, MASK64_2,
    MASK64_ALL, Not, OFFSET_C, OFFSET_D, OFFSET_H, OFFSET_S, OFFSET_SUIT,
    PQLCardCount, RANK_NAMES, Rank, Rank16, Rank16Iter, SUIT_NAMES, Suit, fmt,
    prim, u16_to_rank_str,
};

#[cfg(any(test, feature = "benchmark"))]
#[macro_export]
macro_rules! c64 {
    ($s:expr) => {
        $crate::Card64::from($crate::cards![$s].as_ref())
    };
}

/// A 64-bit representation of a set of playing cards using bitwise operations.
///
/// Card64 efficiently stores multiple cards in a single u64 value, where each bit
/// represents a specific card. This allows for fast set operations like union,
/// intersection, and membership testing using bitwise operations.
///
/// Card Set
/// # Memory Layout:
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
pub struct Card64(u64);

impl Card64 {
    /// Constructs [Card64] from [u64]
    ///
    /// # Examples
    ///
    /// ```
    /// use open_pql::{Card, Card64, Rank::*, Suit::*};
    ///
    /// let i: u64 = 0b11;
    /// let c64: Card64 = Card64::from_u64(i);
    ///
    /// assert_eq!(
    ///     c64,
    ///     Card64::from([Card::new(R2, S), Card::new(R3, S)].as_ref())
    /// );
    /// ```
    #[must_use]
    #[inline]
    pub const fn from_u64(v: u64) -> Self {
        Self(v)
    }

    /// Returns the inner [u64]
    ///
    /// # Examples
    ///
    /// ```
    /// use open_pql::{Card, Card64, Rank::*, Suit::*};
    ///
    /// let i: u64 = 0b11;
    /// let c64: Card64 = Card64::from_u64(i);
    ///
    /// assert_eq!(i, c64.to_u64());
    /// ```
    #[must_use]
    #[inline]
    pub const fn to_u64(self) -> u64 {
        self.0
    }

    /// Checks whether all rank masks are unset
    ///
    /// # Examples
    ///
    /// ```
    /// use open_pql::{Card, Card64, Rank::*, Suit::*};
    ///
    /// let c64: Card64 = Card64::from(Card::new(R2, S));
    ///
    /// assert!(!c64.is_empty());
    /// ```
    #[must_use]
    #[inline]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Constructs [Card64] as the set of all 52 cards
    ///
    /// # Examples
    ///
    /// ```
    /// use open_pql::{Card, Card64, Rank::*, Suit::*};
    ///
    /// let c64: Card64 = Card64::all();
    ///
    /// assert_eq!(c64.count(), 52);
    /// ```
    #[must_use]
    #[inline]
    pub const fn all() -> Self {
        Self(MASK64_ALL)
    }

    /// checks whether another [Card64] is a subset
    ///
    /// # Examples
    ///
    /// ```
    /// use open_pql::{Card, Card64, Rank::*, Suit::*};
    ///
    /// let c64_2s: Card64 = Card64::from(Card::new(R2, S));
    /// let c64_2h: Card64 = Card64::from(Card::new(R2, H));
    /// let c64_2s_2h: Card64 = c64_2s | c64_2h;
    ///
    /// assert!(c64_2s_2h.contains(c64_2h));
    /// assert!(!c64_2s.contains(c64_2h));
    /// ```
    #[must_use]
    #[inline]
    pub fn contains(self, other: Self) -> bool {
        other & self == other
    }

    /// checks whether a [Card] is in the set
    ///
    /// # Examples
    ///
    /// ```
    /// use open_pql::{Card, Card64, Rank::*, Suit::*};
    ///
    /// let c64: Card64 = Card64::from(Card::new(R2, S));
    ///
    /// assert!(c64.contains_card(Card::new(R2, S)));
    /// assert!(!c64.contains_card(Card::new(R2, H)));
    /// ```
    #[must_use]
    #[inline]
    pub const fn contains_card(self, c: Card) -> bool {
        let v = Self::u64_from_ranksuit_i8(c.rank as i8, c.suit as i8);
        v & self.0 == v
    }

    /// Marks a [Card]
    ///
    /// # Examples
    ///
    /// ```
    /// use open_pql::{Card, Card64, Rank::*, Suit::*};
    ///
    /// let mut c64: Card64 = Card64::default();
    /// c64.set(Card::new(R2, S));
    ///
    /// assert_eq!(c64, Card64::from(Card::new(R2, S)));
    /// ```
    #[inline]
    pub const fn set(&mut self, c: Card) {
        self.0 |= Self::u64_from_ranksuit_i8(c.rank as i8, c.suit as i8);
    }

    /// Unmarks a [Card]
    ///
    /// # Examples
    ///
    /// ```
    /// use open_pql::{Card, Card64, Rank::*, Suit::*};
    ///
    /// let mut c64: Card64 = Card64::from(Card::new(R2, S));
    /// c64.unset(Card::new(R2, S));
    ///
    /// assert_eq!(c64, Card64::default());
    /// ```
    #[inline]
    pub const fn unset(&mut self, c: Card) {
        self.0 &= !Self::u64_from_ranksuit_i8(c.rank as i8, c.suit as i8);
    }

    // /// Marks three Flop [Card]s
    // #[inline]
    // pub fn set_flop(&mut self, cs: Flop2) {
    //     todo!()
    // }

    // /// Marks five Board [Card]s
    // #[inline]
    // pub fn set_board(&mut self, b: Board2) {
    //     todo!()
    // }

    /// Returns the number of marked cards
    ///
    /// # Examples
    ///
    /// ```
    /// use open_pql::{Card, Card64, Rank::*, Suit::*};
    ///
    /// let c64: Card64 = Card64::from(Card::new(R2, S));
    ///
    /// assert_eq!(c64.count(), 1);
    /// ```
    #[must_use]
    #[inline]
    pub const fn count(&self) -> PQLCardCount {
        self.0.count_ones().to_le_bytes()[0]
    }

    /// Returns the number of marked cards of rank r
    ///
    /// # Examples
    ///
    /// ```
    /// use open_pql::{Card, Card64, Rank::*, Suit::*};
    ///
    /// let c64: Card64 = Card64::all();
    ///
    /// assert_eq!(c64.count_by_rank(RA), 4);
    /// ```
    pub const fn count_by_rank(self, r: Rank) -> PQLCardCount {
        (self.0 & MASK64_2 << r as u8).count_ones().to_le_bytes()[0]
    }

    /// Returns the number of marked cards of suit s
    ///
    /// # Examples
    ///
    /// ```
    /// use open_pql::{Card, Card64, Rank::*, Suit::*};
    ///
    /// let c64: Card64 = Card64::all();
    ///
    /// assert_eq!(c64.count_by_suit(D), 13);
    /// ```
    pub const fn count_by_suit(self, s: Suit) -> PQLCardCount {
        #[inline]
        const fn count_ones(v: u8) -> u8 {
            v.count_ones().to_le_bytes()[0]
        }

        let bytes = self.to_u64().to_le_bytes();

        match s {
            Suit::S => count_ones(bytes[0]) + count_ones(bytes[1]),
            Suit::H => count_ones(bytes[2]) + count_ones(bytes[3]),
            Suit::D => count_ones(bytes[4]) + count_ones(bytes[5]),
            Suit::C => count_ones(bytes[6]) + count_ones(bytes[7]),
        }
    }

    pub(crate) const fn u64_from_ranksuit_i8(r: i8, s: i8) -> u64 {
        1 << r << (s * OFFSET_SUIT)
    }

    /// Attempts to mark a card of rank r in the order S, H, D, C
    #[allow(unused)]
    #[inline]
    pub(crate) fn set_available_card_by_rank(&mut self, r: Rank) {
        for s in Suit::ARR_ALL {
            let c = Card::new(r, s);

            if !self.contains_card(c) {
                return self.set(c);
            }
        }
    }

    /// Normalize u64 so that each u16 indicates rank count
    #[allow(unused)]
    #[inline]
    pub(crate) const fn normalize(&mut self) {
        self.0 = u64::from_le_bytes(prim::normalize_u64(self.0));
    }

    #[inline]
    pub(crate) const fn ranks_by_suit(self, s: Suit) -> Rank16 {
        let bytes = self.0.to_le_bytes();

        match s {
            Suit::S => {
                Rank16::from_u16(u16::from_le_bytes([bytes[0], bytes[1]]))
            }
            Suit::H => {
                Rank16::from_u16(u16::from_le_bytes([bytes[2], bytes[3]]))
            }
            Suit::D => {
                Rank16::from_u16(u16::from_le_bytes([bytes[4], bytes[5]]))
            }
            Suit::C => {
                Rank16::from_u16(u16::from_le_bytes([bytes[6], bytes[7]]))
            }
        }
    }

    /// Creates a Card64 with all four cards of each rank in the given Rank16.
    ///
    /// # Examples
    ///
    /// ```
    /// use open_pql::{Card64, Rank::*, Rank16};
    ///
    /// let ranks = Rank16::from([RA, RK].as_ref());
    /// let c64 = Card64::from_ranks(ranks);
    ///
    /// assert_eq!(c64.count_by_rank(RA), 4);
    /// assert_eq!(c64.count_by_rank(RK), 4);
    /// assert_eq!(c64.count_by_rank(RQ), 0);
    /// ```
    #[inline]
    #[must_use]
    pub const fn from_ranks(rs: Rank16) -> Self {
        let v = rs.to_u16() as u64;

        Self(v << OFFSET_S | v << OFFSET_H | v << OFFSET_D | v << OFFSET_C)
    }

    /// Returns a Rank16 containing all ranks that have at least one card marked.
    ///
    /// # Examples
    ///
    /// ```
    /// use open_pql::{Card, Card64, Rank::*, Suit::*};
    ///
    /// let c64 = Card64::from([Card::new(RA, S), Card::new(RK, H)].as_ref());
    /// let ranks = c64.ranks();
    ///
    /// assert!(ranks.contains_rank(RA));
    /// assert!(ranks.contains_rank(RK));
    /// assert!(!ranks.contains_rank(RQ));
    /// ```
    #[inline]
    #[must_use]
    pub const fn ranks(self) -> Rank16 {
        let arr = self.0.to_le_bytes();

        let lo = arr[0] | arr[2] | arr[4] | arr[6];
        let hi = arr[1] | arr[3] | arr[5] | arr[7];

        Rank16::from_u16(u16::from_le_bytes([lo, hi]))
    }

    /// Returns an iterator over all cards in this set.
    ///
    /// # Examples
    ///
    /// ```
    /// use open_pql::{Card, Card64, Rank::*, Suit::*};
    ///
    /// let c64 = Card64::from([Card::new(RA, S), Card::new(RK, H)].as_ref());
    /// let cards: Vec<Card> = c64.iter().collect();
    ///
    /// assert_eq!(cards.len(), 2);
    /// ```
    pub const fn iter(self) -> CardIter {
        CardIter::new(self)
    }

    /// Returns an iterator over the ranks in each suit.
    ///
    /// The iterator yields tuples of (Rank16, Suit) for each suit in the order S, H, D, C.
    ///
    /// # Examples
    ///
    /// ```
    /// use open_pql::{Card, Card64, Rank::*, Suit::*};
    ///
    /// let c64 = Card64::from([Card::new(RA, S), Card::new(RK, H)].as_ref());
    /// let ranks: Vec<_> = c64.iter_ranks().collect();
    ///
    /// assert_eq!(ranks.len(), 4);
    /// assert!(ranks[0].0.contains_rank(RA)); // Spades
    /// assert!(ranks[1].0.contains_rank(RK)); // Hearts
    /// ```
    pub const fn iter_ranks(self) -> Rank16Iter {
        Rank16Iter::new(self)
    }
}

impl Not for Card64 {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0 & MASK64_ALL)
    }
}

impl fmt::Debug for Card64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[inline]
        fn to_s(v: u16) -> String {
            let s = u16_to_rank_str(v);
            if s.is_empty() { "_".into() } else { s }
        }

        #[inline]
        const fn truncate_i8(v: usize) -> i8 {
            i8::from_le_bytes([v.to_le_bytes()[0]])
        }

        let n = self.0.count_ones();

        if n == 1 {
            for (sv, s) in SUIT_NAMES.iter().enumerate() {
                for (rv, r) in RANK_NAMES.iter().enumerate() {
                    if self.0
                        == Self::u64_from_ranksuit_i8(
                            truncate_i8(rv),
                            truncate_i8(sv),
                        )
                    {
                        return f.write_str(&format!("Card64<{r}{s}>"));
                    }
                }
            }
        }

        let bs = self.0.to_le_bytes();

        f.debug_tuple("Card64")
            .field(&format_args!(
                "{}",
                to_s(u16::from_le_bytes([bs[0], bs[1]]))
            ))
            .field(&format_args!(
                "{}",
                to_s(u16::from_le_bytes([bs[2], bs[3]]))
            ))
            .field(&format_args!(
                "{}",
                to_s(u16::from_le_bytes([bs[4], bs[5]]))
            ))
            .field(&format_args!(
                "{}",
                to_s(u16::from_le_bytes([bs[6], bs[7]]))
            ))
            .finish()
    }
}

impl From<&[Card]> for Card64 {
    fn from(cs: &[Card]) -> Self {
        let mut res = Self::default();

        for c in cs {
            res.0 |= Self::u64_from_ranksuit_i8(c.rank as i8, c.suit as i8);
        }

        res
    }
}

impl From<Card> for Card64 {
    fn from(c: Card) -> Self {
        Self::from_u64(Self::u64_from_ranksuit_i8(c.rank as i8, c.suit as i8))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    impl Arbitrary for Card64 {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let inner = u64::arbitrary(g);

            Self(MASK64_ALL & inner)
        }
    }

    #[test]
    fn test_empty() {
        assert_eq!(Card64::default(), Card64(0));
        assert!(Card64::default().is_empty());
        assert!(!Card64(1).is_empty());
    }

    #[quickcheck]
    fn test_all(c: Card) {
        let all = Card64::all();

        assert!(all.contains_card(c));
    }

    #[quickcheck]
    fn test_u64(i: u64) -> TestResult {
        if i & MASK64_ALL != i {
            return TestResult::discard();
        }

        assert_eq!(Card64(i), Card64::from_u64(i));
        assert_eq!(i, Card64(i).to_u64());

        TestResult::passed()
    }

    #[quickcheck]
    fn test_set_and_contains_card(c1: Card, c2: Card) {
        let mut c64 = Card64::default();
        c64.set(c1);
        c64.set(c2);

        assert!(c64.contains_card(c1));
        assert!(c64.contains_card(c2));

        c64.unset(c1);

        assert!(!c64.contains_card(c1));
        assert_eq!(c64.contains_card(c2), c2 != c1);
    }

    //#[quickcheck]
    //fn test_set_flop_and_board(b: Board2) -> TestResult {
    //if !board_distinct(b) {
    //    return TestResult::discard();
    //}

    //let mut flop = Card64::default();

    //flop.set_flop(b.0);
    //assert!(flop.contains_card(b.0 .0));
    //assert!(flop.contains_card(b.0 .1));
    //assert!(flop.contains_card(b.0 .2));

    //let mut board = Card64::default();

    //board.set_board(b);
    //assert!(board.contains_card(b.0 .0));
    //assert!(board.contains_card(b.0 .1));
    //assert!(board.contains_card(b.0 .2));
    //assert!(board.contains_card(b.1));
    //assert!(board.contains_card(b.2));

    //assert!(board.contains(flop));

    //TestResult::passed()
    //}

    //#[quickcheck]
    //fn test_set_cards(cards: (Card, Card, Card, Card)) {
    //    let mut lhs = Card64::default();
    //    lhs.set_cards([cards.0, cards.1, cards.2, cards.3].as_ref());

    //    let mut rhs = Card64::default();
    //    rhs.set(cards.0);
    //    rhs.set(cards.1);
    //    rhs.set(cards.2);
    //    rhs.set(cards.3);

    //    assert_eq!(lhs, rhs);
    //}

    #[quickcheck]
    fn test_from_card(c1: Card, c2: Card) {
        let cards = Card64::from(c1);

        assert!(cards.contains_card(c1));

        let cards = Card64::from([c1, c2].as_ref());

        assert!(cards.contains_card(c1));
        assert!(cards.contains_card(c2));
    }

    #[quickcheck]
    fn test_bit_not(c: Card) {
        let c64 = Card64::from(c);
        let c64_complement = !c64;

        assert!(c64.contains_card(c));
        assert!(!c64_complement.contains_card(c));
        assert_eq!(c64 | c64_complement, Card64::all());
        assert_eq!(c64, !c64_complement);
    }

    #[quickcheck]
    fn test_bit_and(c1: Card, c2: Card) {
        let mut a = Card64::from(c1);
        let b = Card64::from(c2);

        assert_eq!((a & b).is_empty(), c1 != c2);

        a &= Card64::default();

        assert_eq!(a, Card64::default());
    }

    #[quickcheck]
    fn test_bit_or(c1: Card, c2: Card) {
        let mut a = Card64::from(c1);
        let b = Card64::from(c2);

        assert!((a | b).contains_card(c1));
        assert!((a | b).contains_card(c2));

        a |= Card64::all();

        assert_eq!(a, Card64::all());
    }

    #[quickcheck]
    fn test_count(c1: Card, c2: Card) {
        let c = Card64::from([c1, c2].as_ref());

        let count = if c1 == c2 { 1 } else { 2 };

        assert_eq!(count, c.count());
    }

    #[quickcheck]
    fn test_count_by_rank(cards: CardN<20>) -> TestResult {
        let c: Card64 = cards.clone().into();

        for r in Rank::ARR_ALL {
            let count = cards.as_ref().iter().filter(|c| c.rank == r).count();

            assert_eq!(count, c.count_by_rank(r) as usize);
        }

        TestResult::passed()
    }

    #[quickcheck]
    fn test_count_by_suit(cards: CardN<5>) -> TestResult {
        let c: Card64 = cards.clone().into();

        for s in Suit::ARR_ALL {
            let count = cards.as_ref().iter().filter(|c| c.suit == s).count();

            assert_eq!(count, c.count_by_suit(s) as usize);
        }

        TestResult::passed()
    }

    #[quickcheck]
    fn test_set_available_card_by_rank(mut c64: Card64, r: Rank) -> TestResult {
        let n = c64.count_by_rank(r);

        c64.set_available_card_by_rank(r);

        let m = c64.count_by_rank(r);

        TestResult::from_bool(m == n + 1 || n == 4 && m == 4)
    }

    #[quickcheck]
    fn test_normalize(mut c64: Card64) {
        let rank_count = Rank::ARR_ALL
            .into_iter()
            .map(|r| (r, c64.count_by_rank(r)))
            .collect::<Vec<_>>();

        c64.normalize();

        for (r, count) in rank_count {
            assert_eq!(c64.contains_card(Card::new(r, Suit::S)), count > 0);
            assert_eq!(c64.contains_card(Card::new(r, Suit::H)), count > 1);
            assert_eq!(c64.contains_card(Card::new(r, Suit::D)), count > 2);
            assert_eq!(c64.contains_card(Card::new(r, Suit::C)), count > 3);
        }
    }

    #[quickcheck]
    fn test_from_ranks_and_ranks(ranks: Rank16) {
        let c = Card64::from_ranks(ranks);

        for r in Rank::ARR_ALL {
            if ranks.contains_rank(r) {
                assert_eq!(4, c.count_by_rank(r));
            } else {
                assert_eq!(0, c.count_by_rank(r));
            }
        }

        assert_eq!(ranks, c.ranks());
    }

    #[quickcheck]
    fn test_contains(c1: Card64, c2: Card64) -> TestResult {
        let combined = c1 | c2;

        assert!(combined.contains(c1));
        assert!(combined.contains(c2));
        assert!(Card64::all().contains(c1));

        if (c1 & c2).is_empty() && !c2.is_empty() {
            assert!(!c1.contains(c2));
        }

        TestResult::passed()
    }

    #[test]
    fn test_iter() {
        let empty = Card64::default();
        assert_eq!(empty.iter().count(), 0);

        let single = c64!("As");
        let cards: Vec<Card> = single.iter().collect();
        assert_eq!(cards.len(), 1);
        assert_eq!(cards[0], cards!("As")[0]);

        let multiple = c64!("As Kh 2d");
        let cards: Vec<Card> = multiple.iter().collect();
        assert_eq!(cards.len(), 3);
        assert!(cards.contains(&cards!("As")[0]));
        assert!(cards.contains(&cards!("Kh")[0]));
        assert!(cards.contains(&cards!("2d")[0]));

        let all = Card64::all();
        assert_eq!(all.iter().count(), constants::N_CARDS as usize);
    }

    #[test]
    fn test_iter_ranks() {
        let empty = Card64::default();
        assert_eq!(empty.iter_ranks().count(), 4);

        let single = c64!("As");
        let ranks: Vec<(Rank16, Suit)> = single.iter_ranks().collect();
        assert_eq!(ranks.len(), 4);
        assert!(ranks[0].0.contains_rank(Rank::RA));
        assert_eq!(ranks[0].1, Suit::S);

        let same_rank = c64!("As Ah Ad");
        let ranks: Vec<(Rank16, Suit)> = same_rank.iter_ranks().collect();
        assert_eq!(ranks.len(), 4);
        assert!(ranks[0].0.contains_rank(Rank::RA));
        assert!(ranks[1].0.contains_rank(Rank::RA));
        assert!(ranks[2].0.contains_rank(Rank::RA));
        assert!(!ranks[3].0.contains_rank(Rank::RA));

        let multiple = c64!("As Kh 2d");
        let ranks: Vec<(Rank16, Suit)> = multiple.iter_ranks().collect();
        assert_eq!(ranks.len(), 4);

        let (spade_ranks, spade) = ranks[0];
        assert_eq!(spade, Suit::S);
        assert!(spade_ranks.contains_rank(Rank::RA));
        assert!(!spade_ranks.contains_rank(Rank::RK));
        assert!(!spade_ranks.contains_rank(Rank::R2));

        let (heart_ranks, heart) = ranks[1];
        assert_eq!(heart, Suit::H);
        assert!(!heart_ranks.contains_rank(Rank::RA));
        assert!(heart_ranks.contains_rank(Rank::RK));
        assert!(!heart_ranks.contains_rank(Rank::R2));

        let (diamond_ranks, diamond) = ranks[2];
        assert_eq!(diamond, Suit::D);
        assert!(!diamond_ranks.contains_rank(Rank::RA));
        assert!(!diamond_ranks.contains_rank(Rank::RK));
        assert!(diamond_ranks.contains_rank(Rank::R2));
    }

    #[test]
    fn test_debug() {
        let s = format!("{:?}", c64!("As"));
        assert_eq!(s, "Card64<As>");

        let s = format!("{:?}", c64!("As 9h"));
        assert_eq!(s, "Card64(A, 9, _, _)");
    }
}
