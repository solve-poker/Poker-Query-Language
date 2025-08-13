use super::{
    Display, FromStr, Hash, ParseError, Rank, RankIdx, Suit, SuitIdx, fmt,
};

/// Creates a vector of cards from a string representation.
///
/// # Examples
///
/// ```
/// use open_pql::cards;
///
/// let cards = cards!("As Kh 2d");
/// assert_eq!(cards.len(), 3);
/// ```
#[cfg(any(test, feature = "benchmark"))]
#[macro_export]
macro_rules! cards {
    ($s:expr) => {
        $crate::Card::new_vec($s)
    };
}

/// Creates a single card from a string representation.
///
/// # Examples
///
/// ```
/// use open_pql::card;
///
/// let ace_spades = card!("As");
/// ```
#[cfg(any(test, feature = "benchmark"))]
#[macro_export]
macro_rules! card {
    ($s:expr) => {
        cards!($s)[0]
    };
}

/// Single Card
///
/// Represents a single playing card with a rank and suit.
/// Cards are ordered first by rank then by suit for consistent comparison.
///
/// # Examples
///
/// ```
/// use open_pql::{Card, Rank::*, Suit::*};
///
/// let card = Card::new(RA, S);
/// assert_eq!(card.rank, RA);
/// assert_eq!(card.suit, S);
/// assert_eq!(card.to_string(), "As");
/// ```
#[derive(Copy, Clone, Display, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[display("{rank}{suit}")]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    /// Array of all 52 playing cards in a standard deck.
    /// Cards are ordered by rank (2-A) then by suit (S, H, D, C).
    pub const ARR_ALL: [Self; 52] = [
        Self::new(Rank::R2, Suit::S),
        Self::new(Rank::R2, Suit::H),
        Self::new(Rank::R2, Suit::D),
        Self::new(Rank::R2, Suit::C),
        Self::new(Rank::R3, Suit::S),
        Self::new(Rank::R3, Suit::H),
        Self::new(Rank::R3, Suit::D),
        Self::new(Rank::R3, Suit::C),
        Self::new(Rank::R4, Suit::S),
        Self::new(Rank::R4, Suit::H),
        Self::new(Rank::R4, Suit::D),
        Self::new(Rank::R4, Suit::C),
        Self::new(Rank::R5, Suit::S),
        Self::new(Rank::R5, Suit::H),
        Self::new(Rank::R5, Suit::D),
        Self::new(Rank::R5, Suit::C),
        Self::new(Rank::R6, Suit::S),
        Self::new(Rank::R6, Suit::H),
        Self::new(Rank::R6, Suit::D),
        Self::new(Rank::R6, Suit::C),
        Self::new(Rank::R7, Suit::S),
        Self::new(Rank::R7, Suit::H),
        Self::new(Rank::R7, Suit::D),
        Self::new(Rank::R7, Suit::C),
        Self::new(Rank::R8, Suit::S),
        Self::new(Rank::R8, Suit::H),
        Self::new(Rank::R8, Suit::D),
        Self::new(Rank::R8, Suit::C),
        Self::new(Rank::R9, Suit::S),
        Self::new(Rank::R9, Suit::H),
        Self::new(Rank::R9, Suit::D),
        Self::new(Rank::R9, Suit::C),
        Self::new(Rank::RT, Suit::S),
        Self::new(Rank::RT, Suit::H),
        Self::new(Rank::RT, Suit::D),
        Self::new(Rank::RT, Suit::C),
        Self::new(Rank::RJ, Suit::S),
        Self::new(Rank::RJ, Suit::H),
        Self::new(Rank::RJ, Suit::D),
        Self::new(Rank::RJ, Suit::C),
        Self::new(Rank::RQ, Suit::S),
        Self::new(Rank::RQ, Suit::H),
        Self::new(Rank::RQ, Suit::D),
        Self::new(Rank::RQ, Suit::C),
        Self::new(Rank::RK, Suit::S),
        Self::new(Rank::RK, Suit::H),
        Self::new(Rank::RK, Suit::D),
        Self::new(Rank::RK, Suit::C),
        Self::new(Rank::RA, Suit::S),
        Self::new(Rank::RA, Suit::H),
        Self::new(Rank::RA, Suit::D),
        Self::new(Rank::RA, Suit::C),
    ];

    /// Array of 36 cards for short deck poker (6-A).
    /// Excludes ranks 2-5, commonly used in some poker variants.
    pub const ARR_ALL_SHORT: [Self; 36] = [
        Self::new(Rank::R6, Suit::S),
        Self::new(Rank::R6, Suit::H),
        Self::new(Rank::R6, Suit::D),
        Self::new(Rank::R6, Suit::C),
        Self::new(Rank::R7, Suit::S),
        Self::new(Rank::R7, Suit::H),
        Self::new(Rank::R7, Suit::D),
        Self::new(Rank::R7, Suit::C),
        Self::new(Rank::R8, Suit::S),
        Self::new(Rank::R8, Suit::H),
        Self::new(Rank::R8, Suit::D),
        Self::new(Rank::R8, Suit::C),
        Self::new(Rank::R9, Suit::S),
        Self::new(Rank::R9, Suit::H),
        Self::new(Rank::R9, Suit::D),
        Self::new(Rank::R9, Suit::C),
        Self::new(Rank::RT, Suit::S),
        Self::new(Rank::RT, Suit::H),
        Self::new(Rank::RT, Suit::D),
        Self::new(Rank::RT, Suit::C),
        Self::new(Rank::RJ, Suit::S),
        Self::new(Rank::RJ, Suit::H),
        Self::new(Rank::RJ, Suit::D),
        Self::new(Rank::RJ, Suit::C),
        Self::new(Rank::RQ, Suit::S),
        Self::new(Rank::RQ, Suit::H),
        Self::new(Rank::RQ, Suit::D),
        Self::new(Rank::RQ, Suit::C),
        Self::new(Rank::RK, Suit::S),
        Self::new(Rank::RK, Suit::H),
        Self::new(Rank::RK, Suit::D),
        Self::new(Rank::RK, Suit::C),
        Self::new(Rank::RA, Suit::S),
        Self::new(Rank::RA, Suit::H),
        Self::new(Rank::RA, Suit::D),
        Self::new(Rank::RA, Suit::C),
    ];

    /// Creates a new card with the specified rank and suit.
    ///
    /// # Examples
    ///
    /// ```
    /// use open_pql::{Card, Rank, Suit};
    ///
    /// let ace_spades = Card::new(Rank::RA, Suit::S);
    /// ```
    #[must_use]
    #[inline]
    pub const fn new(r: Rank, s: Suit) -> Self {
        Self { rank: r, suit: s }
    }

    /// Creates a card from rank and suit indices.
    #[must_use]
    #[inline]
    pub(crate) fn from_indices(r: RankIdx, s: SuitIdx) -> Self {
        Self {
            rank: r.to_rank(),
            suit: s.to_suit(),
        }
    }

    /// Converts the card to a single u8 representation [xxSSRRRR].
    pub(crate) const fn to_u8(self) -> u8 {
        const SHIFT_SUIT: u8 = 4;
        (self.rank as u8) | ((self.suit as u8) << SHIFT_SUIT)
    }

    /// Creates a card from a u8 [xxSSRRRR].
    pub(crate) fn from_u8(v: u8) -> Self {
        const SHIFT_SUIT: u8 = 4;
        Self::from_indices(
            RankIdx::new(v & 0b1111),
            SuitIdx::new(v >> SHIFT_SUIT),
        )
    }
}

/// Default implementation for Card.
///
/// Returns the Two of Spades as the default card.
impl Default for Card {
    fn default() -> Self {
        Self::new(Rank::R2, Suit::S)
    }
}

/// Debug implementation for Card.
///
/// Uses the Display format for debug output.
impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

/// `FromStr` implementation for Card.
///
/// Parses a card from string format like "As" (Ace of Spades).
/// Whitespace is ignored during parsing.
///
/// # Errors
///
/// Returns `ParseError::InvalidCard` if the string cannot be parsed as a valid card.
///
/// # Examples
///
/// ```
/// use open_pql::Card;
///
/// let card: Card = "As".parse().unwrap();
/// let card_with_spaces: Card = " A s ".parse().unwrap();
/// ```
impl FromStr for Card {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cs = s.chars().filter(|c| !c.is_whitespace());

        if let Some(c) = cs.next()
            && let Ok(r) = Rank::try_from(c)
            && let Some(c) = cs.next()
            && let Ok(s) = Suit::try_from(c)
            && cs.next().is_none()
        {
            return Ok(Self::new(r, s));
        }

        Err(ParseError::InvalidCard(s.into()))
    }
}

impl<T> From<T> for Rank
where
    Card: From<T>,
{
    fn from(v: T) -> Self {
        Card::from(v).rank
    }
}

impl<T> From<T> for Suit
where
    Card: From<T>,
{
    fn from(v: T) -> Self {
        Card::from(v).suit
    }
}

#[cfg(any(test, feature = "benchmark"))]
impl Card {
    /// Creates a card from a tuple of characters representing rank and suit.
    ///
    /// # Panics
    ///
    /// Panics if the characters cannot be converted to valid rank and suit.
    /// This is intended to be used in tests and benchmarks only.
    ///
    /// # Examples
    ///
    /// ```
    /// use open_pql::Card;
    ///
    /// let card = Card::from_tuple(('A', 's'));
    /// ```
    pub fn from_tuple((r, s): (char, char)) -> Self {
        Self::new(r.try_into().unwrap(), s.try_into().unwrap())
    }

    /// Creates a vector of cards from a string representation.
    ///
    /// Parses pairs of characters as rank-suit combinations.
    ///
    /// # Examples
    ///
    /// ```
    /// use open_pql::Card;
    ///
    /// let cards = Card::new_vec("As Kh Qd");
    /// assert_eq!(cards.len(), 3);
    /// ```
    pub fn new_vec(s: &str) -> Vec<Self> {
        use itertools::Itertools;

        s.chars()
            .filter(|c| !c.is_whitespace())
            .tuples()
            .map(Self::from_tuple)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    impl Arbitrary for Card {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            *g.choose(&Self::ARR_ALL).unwrap()
        }
    }

    #[quickcheck]
    fn test_rank_suit(r: Rank, s: Suit) {
        let card = Card::new(r, s);

        assert_eq!(r, card.rank);
        assert_eq!(s, card.suit);
    }

    #[test]
    fn test_default() {
        assert_eq!(Card::new(Rank::R2, Suit::S), Card::default());
        assert_eq!(card!("2s"), Card::default());
    }

    #[quickcheck]
    fn test_hash(c1: Card) {
        use std::hash::DefaultHasher;

        let c2 = c1;

        let mut h1 = DefaultHasher::new();
        let mut h2 = DefaultHasher::new();

        c1.hash(&mut h1);
        c2.hash(&mut h2);

        assert_eq!(h1.finish(), h2.finish());
    }

    #[quickcheck]
    fn test_into_rank_and_suit(c: Card) {
        let r: Rank = c.into();
        let s: Suit = c.into();

        assert_eq!(r, c.rank);
        assert_eq!(s, c.suit);
    }

    #[test]
    fn test_from_str() {
        let c = |s| Ok(cards!(s)[0]);

        assert_eq!(c("2s"), "2s".parse());
        assert_eq!(c("2s"), " 2 S ".parse());
        assert_eq!(
            Err(ParseError::InvalidCard("2s?".to_owned())),
            "2s?".parse::<Card>()
        );
        assert!("".parse::<Card>().is_err());
        assert!("?".parse::<Card>().is_err());
        assert!("2".parse::<Card>().is_err());
        assert!("2k".parse::<Card>().is_err());
    }

    #[quickcheck]
    fn test_to_string(c: Card) {
        assert_eq!(format!("{}{}", c.rank, c.suit), c.to_string());
    }

    #[quickcheck]
    fn test_to_u8(c: Card) {
        assert_eq!(c, Card::from_u8(c.to_u8()));
    }

    #[test]
    fn test_macro() {
        let cards = cards!("As Kh");
        assert_eq!(
            cards,
            vec![Card::new(Rank::RA, Suit::S), Card::new(Rank::RK, Suit::H),]
        );

        let flop = flop!("2s 3s 4s");
        assert_eq!(
            flop,
            (
                Card::new(Rank::R2, Suit::S),
                Card::new(Rank::R3, Suit::S),
                Card::new(Rank::R4, Suit::S),
            )
                .into()
        );

        let board = board!("As Kh 3s 5h 6c");
        assert_eq!(
            board,
            (
                Card::new(Rank::RA, Suit::S),
                Card::new(Rank::RK, Suit::H),
                Card::new(Rank::R3, Suit::S),
                Card::new(Rank::R5, Suit::H),
                Card::new(Rank::R6, Suit::C),
            )
                .into(),
        );
    }

    #[test]
    fn test_ord() {
        let mut sorted = Card::ARR_ALL.to_vec();
        sorted.reverse();
        sorted.sort_unstable();

        assert_eq!(sorted, Card::ARR_ALL);
    }
}
