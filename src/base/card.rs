use super::*;

#[cfg(any(test, feature = "benchmark"))]
#[macro_export]
macro_rules! cards {
    ($s:expr) => {
        $crate::Card::new_vec($s)
    };
}

#[cfg(any(test, feature = "benchmark"))]
#[macro_export]
macro_rules! flop {
    ($s:expr) => {
        $crate::Flop::from(
            <[$crate::Card; 3]>::try_from($crate::Card::new_vec($s)).unwrap(),
        )
    };
}

#[cfg(any(test, feature = "benchmark"))]
#[macro_export]
macro_rules! board {
    ($s:expr) => {
        $crate::Board::from(
            $crate::Card::new_vec($s).as_ref() as &[$crate::Card]
        )
    };
}

/// Single Card
#[derive(Copy, Clone, PartialEq, Eq, Display)]
#[display("{r}{s}")]
pub struct Card {
    pub r: Rank,
    pub s: Suit,
}

impl Hash for Card {
    fn hash<H: Hasher>(&self, state: &mut H) {
        ((self.s as u8) * N_RANKS + (self.r as u8)).hash(state);
    }
}

impl Card {
    pub(crate) const C_2S: Self = Self::new(Rank::R2, Suit::S);
    pub(crate) const C_3S: Self = Self::new(Rank::R3, Suit::S);
    pub(crate) const C_4S: Self = Self::new(Rank::R4, Suit::S);
    pub(crate) const C_5S: Self = Self::new(Rank::R5, Suit::S);
    pub(crate) const C_6S: Self = Self::new(Rank::R6, Suit::S);
    pub(crate) const C_7S: Self = Self::new(Rank::R7, Suit::S);
    pub(crate) const C_8S: Self = Self::new(Rank::R8, Suit::S);
    pub(crate) const C_9S: Self = Self::new(Rank::R9, Suit::S);
    pub(crate) const C_TS: Self = Self::new(Rank::RT, Suit::S);
    pub(crate) const C_JS: Self = Self::new(Rank::RJ, Suit::S);
    pub(crate) const C_QS: Self = Self::new(Rank::RQ, Suit::S);
    pub(crate) const C_KS: Self = Self::new(Rank::RK, Suit::S);
    pub(crate) const C_AS: Self = Self::new(Rank::RA, Suit::S);

    pub(crate) const C_2H: Self = Self::new(Rank::R2, Suit::H);
    pub(crate) const C_3H: Self = Self::new(Rank::R3, Suit::H);
    pub(crate) const C_4H: Self = Self::new(Rank::R4, Suit::H);
    pub(crate) const C_5H: Self = Self::new(Rank::R5, Suit::H);
    pub(crate) const C_6H: Self = Self::new(Rank::R6, Suit::H);
    pub(crate) const C_7H: Self = Self::new(Rank::R7, Suit::H);
    pub(crate) const C_8H: Self = Self::new(Rank::R8, Suit::H);
    pub(crate) const C_9H: Self = Self::new(Rank::R9, Suit::H);
    pub(crate) const C_TH: Self = Self::new(Rank::RT, Suit::H);
    pub(crate) const C_JH: Self = Self::new(Rank::RJ, Suit::H);
    pub(crate) const C_QH: Self = Self::new(Rank::RQ, Suit::H);
    pub(crate) const C_KH: Self = Self::new(Rank::RK, Suit::H);
    pub(crate) const C_AH: Self = Self::new(Rank::RA, Suit::H);

    pub(crate) const C_2D: Self = Self::new(Rank::R2, Suit::D);
    pub(crate) const C_3D: Self = Self::new(Rank::R3, Suit::D);
    pub(crate) const C_4D: Self = Self::new(Rank::R4, Suit::D);
    pub(crate) const C_5D: Self = Self::new(Rank::R5, Suit::D);
    pub(crate) const C_6D: Self = Self::new(Rank::R6, Suit::D);
    pub(crate) const C_7D: Self = Self::new(Rank::R7, Suit::D);
    pub(crate) const C_8D: Self = Self::new(Rank::R8, Suit::D);
    pub(crate) const C_9D: Self = Self::new(Rank::R9, Suit::D);
    pub(crate) const C_TD: Self = Self::new(Rank::RT, Suit::D);
    pub(crate) const C_JD: Self = Self::new(Rank::RJ, Suit::D);
    pub(crate) const C_QD: Self = Self::new(Rank::RQ, Suit::D);
    pub(crate) const C_KD: Self = Self::new(Rank::RK, Suit::D);
    pub(crate) const C_AD: Self = Self::new(Rank::RA, Suit::D);

    pub(crate) const C_2C: Self = Self::new(Rank::R2, Suit::C);
    pub(crate) const C_3C: Self = Self::new(Rank::R3, Suit::C);
    pub(crate) const C_4C: Self = Self::new(Rank::R4, Suit::C);
    pub(crate) const C_5C: Self = Self::new(Rank::R5, Suit::C);
    pub(crate) const C_6C: Self = Self::new(Rank::R6, Suit::C);
    pub(crate) const C_7C: Self = Self::new(Rank::R7, Suit::C);
    pub(crate) const C_8C: Self = Self::new(Rank::R8, Suit::C);
    pub(crate) const C_9C: Self = Self::new(Rank::R9, Suit::C);
    pub(crate) const C_TC: Self = Self::new(Rank::RT, Suit::C);
    pub(crate) const C_JC: Self = Self::new(Rank::RJ, Suit::C);
    pub(crate) const C_QC: Self = Self::new(Rank::RQ, Suit::C);
    pub(crate) const C_KC: Self = Self::new(Rank::RK, Suit::C);
    pub(crate) const C_AC: Self = Self::new(Rank::RA, Suit::C);

    /// [ 2s, 3s, ... As, ..., Ah, ..., Ad, ..., Ac ]
    pub const ARR_ALL: [Self; 52] = [
        Self::C_2S,
        Self::C_3S,
        Self::C_4S,
        Self::C_5S,
        Self::C_6S,
        Self::C_7S,
        Self::C_8S,
        Self::C_9S,
        Self::C_TS,
        Self::C_JS,
        Self::C_QS,
        Self::C_KS,
        Self::C_AS,
        Self::C_2H,
        Self::C_3H,
        Self::C_4H,
        Self::C_5H,
        Self::C_6H,
        Self::C_7H,
        Self::C_8H,
        Self::C_9H,
        Self::C_TH,
        Self::C_JH,
        Self::C_QH,
        Self::C_KH,
        Self::C_AH,
        Self::C_2D,
        Self::C_3D,
        Self::C_4D,
        Self::C_5D,
        Self::C_6D,
        Self::C_7D,
        Self::C_8D,
        Self::C_9D,
        Self::C_TD,
        Self::C_JD,
        Self::C_QD,
        Self::C_KD,
        Self::C_AD,
        Self::C_2C,
        Self::C_3C,
        Self::C_4C,
        Self::C_5C,
        Self::C_6C,
        Self::C_7C,
        Self::C_8C,
        Self::C_9C,
        Self::C_TC,
        Self::C_JC,
        Self::C_QC,
        Self::C_KC,
        Self::C_AC,
    ];

    /// [ 7s, 8s, ... As, ..., Ah, ..., Ad, ..., Ac ]
    pub const ARR_ALL_SHORT: [Self; 32] = [
        Self::C_7S,
        Self::C_8S,
        Self::C_9S,
        Self::C_TS,
        Self::C_JS,
        Self::C_QS,
        Self::C_KS,
        Self::C_AS,
        Self::C_7H,
        Self::C_8H,
        Self::C_9H,
        Self::C_TH,
        Self::C_JH,
        Self::C_QH,
        Self::C_KH,
        Self::C_AH,
        Self::C_7D,
        Self::C_8D,
        Self::C_9D,
        Self::C_TD,
        Self::C_JD,
        Self::C_QD,
        Self::C_KD,
        Self::C_AD,
        Self::C_7C,
        Self::C_8C,
        Self::C_9C,
        Self::C_TC,
        Self::C_JC,
        Self::C_QC,
        Self::C_KC,
        Self::C_AC,
    ];

    /// Constructs [Card] from [Rank] and [Suit]
    ///
    /// # Examples
    ///
    /// ```
    /// use open_pql::{Card, Rank, Suit};
    ///
    /// let rank: Rank = Rank::RA;
    /// let suit: Suit = Suit::H;
    ///
    /// let card = Card::new(rank, suit);
    ///
    /// assert_eq!(card.r, rank); // Ace
    /// assert_eq!(card.s, suit); // Heart
    /// ```
    #[must_use]
    #[inline]
    pub const fn new(r: Rank, s: Suit) -> Self {
        Self { r, s }
    }
}

impl Default for Card {
    fn default() -> Self {
        Self::C_2S
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else {
            None
        }
    }
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let r = match self.r {
            Rank::R2 => '2',
            Rank::R3 => '3',
            Rank::R4 => '4',
            Rank::R5 => '5',
            Rank::R6 => '6',
            Rank::R7 => '7',
            Rank::R8 => '8',
            Rank::R9 => '9',
            Rank::RT => 'T',
            Rank::RJ => 'J',
            Rank::RQ => 'Q',
            Rank::RK => 'K',
            Rank::RA => 'A',
        };

        let s = match self.s {
            Suit::S => 's',
            Suit::H => 'h',
            Suit::D => 'd',
            Suit::C => 'c',
        };

        f.write_str(&format!("{r}{s}"))
    }
}

impl FromStr for Card {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cs = s.chars().filter(|c| !c.is_whitespace());

        if let Some(c) = cs.next() {
            if let Ok(r) = Rank::try_from(c) {
                if let Some(c) = cs.next() {
                    if let Ok(s) = Suit::try_from(c) {
                        if cs.next().is_none() {
                            return Ok(Self::new(r, s));
                        }
                    }
                }
            }
        }

        Err(ParseError::InvalidCard(s.into()))
    }
}

impl<T> From<T> for Rank
where
    Card: From<T>,
{
    fn from(v: T) -> Self {
        Card::from(v).r
    }
}

impl<T> From<T> for Suit
where
    Card: From<T>,
{
    fn from(v: T) -> Self {
        Card::from(v).s
    }
}

#[cfg(any(test, feature = "benchmark"))]
impl Card {
    /// # Panics
    /// this is intended to be used in tests and benchmarks only.
    pub fn from_tuple((r, s): (char, char)) -> Self {
        Self::new(r.try_into().unwrap(), s.try_into().unwrap())
    }

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

    impl Arbitrary for Card {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            *g.choose(&Self::ARR_ALL).unwrap()
        }
    }

    #[quickcheck]
    fn test_rank_suit(r: Rank, s: Suit) {
        let card = Card::new(r, s);

        assert_eq!(r, card.r);
        assert_eq!(s, card.s);
    }

    #[test]
    fn test_default() {
        assert_eq!(Card::C_2S, Card::default());
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

        assert_eq!(r, c.r);
        assert_eq!(s, c.s);
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
        assert_eq!(format!("{}{}", c.r, c.s), c.to_string());
    }

    #[test]
    fn test_macro() {
        let cards = cards!("As Kh");
        assert_eq!(cards, vec![Card::C_AS, Card::C_KH]);

        let flop = flop!("2s 3s 4s");
        assert_eq!(flop, (Card::C_2S, Card::C_3S, Card::C_4S).into());

        let board = board!("As Kh 3s 5h 6c");
        assert_eq!(
            board,
            (Card::C_AS, Card::C_KH, Card::C_3S, Card::C_5H, Card::C_6C).into(),
        );
    }

    #[test]
    fn test_partial_ord() {
        assert!(Card::C_2S == Card::C_2S);
        assert_eq!(Card::C_2S.partial_cmp(&Card::C_2S), Some(Ordering::Equal));

        assert!(!(Card::C_2S <= Card::C_3S || Card::C_2S >= Card::C_3S));
        assert!(Card::C_2S <= Card::C_2S);
    }
}
