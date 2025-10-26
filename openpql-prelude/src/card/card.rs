use super::{
    Card64, CardCount, Display, FromStr, Hash, ParseError, Rank, Suit,
};

#[macro_export]
macro_rules! card {
    ($s:expr) => {
        $s.parse::<$crate::Card>().unwrap()
    };
}

#[macro_export]
macro_rules! cards {
    ($s:expr) => {{
        let s: &str = $s;
        let mut cards = Vec::new();
        let mut chars = s.chars().filter(|c| !c.is_whitespace());
        while let (Some(r), Some(s)) = (chars.next(), chars.next()) {
            cards.push($crate::card![format!("{r}{s}")]);
        }
        cards
    }};
}

/// Playing card representation.
///
/// Represents a single playing card with a rank and suit, with macros for convenient creation.
#[derive(Clone, Copy, Debug, Display, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[display("{rank}{suit}")]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    /// Total number of cards in a standard deck
    pub const N_CARDS: CardCount = Suit::N_SUITS * Rank::N_RANKS;
    /// Total number of cards in a short deck
    pub const N_CARDS_SD: CardCount = Suit::N_SUITS * Rank::N_RANKS_SD;

    const ARR_ALL: [Self; Self::N_CARDS as usize] = sealed::all_cards();
    const ARR_ALL_SD: [Self; Self::N_CARDS_SD as usize] =
        sealed::all_cards_sd();

    /// Creates a new card with the specified rank and suit.
    #[must_use]
    #[inline]
    pub const fn new(r: Rank, s: Suit) -> Self {
        Self { rank: r, suit: s }
    }

    /// Returns a slice of all cards
    #[inline]
    pub const fn all<const SD: bool>() -> &'static [Self] {
        const {
            if SD {
                &Self::ARR_ALL_SD
            } else {
                &Self::ARR_ALL
            }
        }
    }

    #[inline]
    pub(crate) const fn to_c64(self) -> Card64 {
        let mut res = Card64::EMPTY;
        res.set(self);

        res
    }

    #[inline]
    pub(crate) const fn eq(self, other: Self) -> bool {
        self.rank.eq(other.rank) && self.suit.eq(other.suit)
    }
}

// compiler-time functions
#[cfg_attr(coverage_nightly, coverage(off))]
mod sealed {
    use super::{Card, CardCount, Rank, Suit};

    const fn mk_card(r: CardCount, s: CardCount) -> Card {
        Card::new(Rank::all::<false>()[r as usize], Suit::ARR_ALL[s as usize])
    }

    pub(super) const fn all_cards() -> [Card; Card::N_CARDS as usize] {
        let mut res = [mk_card(0, 0); Card::N_CARDS as usize];
        let mut i = 0;

        while i < Card::N_CARDS {
            res[i as usize] = mk_card(i / Suit::N_SUITS, i % Suit::N_SUITS);

            i += 1;
        }

        res
    }

    pub(super) const fn all_cards_sd() -> [Card; Card::N_CARDS_SD as usize] {
        const SHIFT: CardCount = 4;
        let mut res = [mk_card(0, 0); Card::N_CARDS_SD as usize];
        let mut i = 0;

        while i < Card::N_CARDS_SD {
            res[i as usize] =
                mk_card(i / Suit::N_SUITS + SHIFT, i % Suit::N_SUITS);

            i += 1;
        }

        res
    }
}

impl Default for Card {
    fn default() -> Self {
        Self::new(Rank::R2, Suit::S)
    }
}

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

#[cfg(any(test, feature = "quickcheck"))]
impl quickcheck::Arbitrary for Card {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        *g.choose(&Self::ARR_ALL).unwrap()
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_all(cards: CardN<3>) {
        for c in cards {
            if c.rank >= Rank::R6 {
                assert!(Card::all::<true>().contains(&c));
            }

            assert!(Card::all::<false>().contains(&c));
        }
    }

    #[test]
    fn test_default() {
        assert_eq!(card!("2s"), Card::default());
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
    fn test_macro(cs: CardN<3>) {
        assert_eq!(cs[0], card!(&cs[0].to_string()));
        let v = cs.as_slice().to_vec();
        let s: String = v.iter().map(ToString::to_string).collect();
        assert_eq!(v, cards!(&s));
    }

    #[test]
    fn test_ord() {
        let mut sorted = Card::ARR_ALL.to_vec();
        sorted.sort_unstable();

        assert_eq!(sorted, Card::ARR_ALL);
    }
}
