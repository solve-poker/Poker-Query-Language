//! helper structs for determine relevent suits
//!
//! * preflop, Holdem/Shortdeck -> only need to distinguish suited/offsuit
//! * preflop, Omaha -> either single suited or double suited
//! * flop -> at most 3 potential drawing suits
//! * turn -> at most 2 drawing suits
//! * river -> at most 1 flush suit

use std::{fmt, str::FromStr};

use crate::{Board, Card, HandN, ParseError, Rank, Street, Suit, Suit4};

const N_HOLDEM: usize = 2;
const N_OMAHA: usize = 4;

/// A single card in a canonical hand.
///
/// `suit` is `Some(s)` when the suit matters for flush potential,
/// `None` when the suit is irrelevant.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CanonicalCard {
    pub rank: Rank,
    pub suit: Option<Suit>,
}

impl FromStr for CanonicalCard {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn to_suit(c: char) -> Result<Option<Suit>, ParseError> {
            if c == '*' {
                Ok(None)
            } else {
                Ok(Some(Suit::try_from(c)?))
            }
        }

        let mut cs = s.chars().filter(|c| !c.is_whitespace());

        if let Some(c) = cs.next()
            && let Ok(rank) = Rank::try_from(c)
            && let Some(c) = cs.next()
            && let Ok(suit) = to_suit(c)
            && cs.next().is_none()
        {
            return Ok(Self { rank, suit });
        }

        Err(ParseError::InvalidCard(s.into()))
    }
}

impl From<Card> for CanonicalCard {
    fn from(card: Card) -> Self {
        Self {
            rank: card.rank,
            suit: Some(card.suit),
        }
    }
}

impl fmt::Debug for CanonicalCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit.map_or('*', Suit::to_char))
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for CanonicalCard {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{self:?}"))
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for CanonicalCard {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;

        let text = String::deserialize(deserializer)?;

        match text.parse() {
            Ok(c) => Ok(c),
            Err(e) => Err(Error::custom(e.to_string())),
        }
    }
}

/// Canonical N-card hand.
///
/// Cards sorted (same order as [`HandN`]) with suit encoded per card as
/// `Option<Suit>`: `Some(s)` when flush-relevant, `None` otherwise.
/// Suited preflop hands normalise to `Suit::S` (e.g. `AhKh` == `AdKd`).
#[derive(
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    derive_more::From,
    derive_more::Into,
)]
pub struct CanonicalHand(pub(crate) Vec<CanonicalCard>);

impl fmt::Debug for CanonicalHand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for card in &self.0 {
            write!(f, "{card:?}")?;
        }
        Ok(())
    }
}

impl CanonicalHand {
    /// Derive the canonical hand given the current board.
    ///
    /// A card's suit is relevant when the total number of cards of that suit
    /// (board + hand + remaining board cards) can reach a flush.  With an
    /// empty board this delegates to preflop logic.
    pub fn new(board: Board, hand: &[Card]) -> Self {
        match Street::from(board) {
            Street::Preflop => Self::preflop(&sort(hand)),
            _ => Self::postflop(board, &sort(hand)),
        }
    }

    fn preflop(hand: &[Card]) -> Self {
        match hand {
            [_, _] => preflop2(hand),
            [_, _, _, _] => preflop4(hand),
            _ => unreachable!(),
        }
    }

    fn postflop(board: Board, hand: &[Card]) -> Self {
        let suits = board.flush_suits();

        hand.iter()
            .copied()
            .map(|c| CanonicalCard {
                rank: c.rank,
                suit: if suits.contains_suit(c.suit) {
                    Some(c.suit)
                } else {
                    None
                },
            })
            .collect::<Vec<_>>()
            .into()
    }
}

/// Derive the canonical hand with no board (preflop), 2-card holdem.
///
/// All cards are treated as suited when they share the same suit (normalised
/// to `Suit::S`); otherwise all suits are marked irrelevant.
fn preflop2(hand: &[Card]) -> CanonicalHand {
    let (c1, c2) = (hand[0], hand[1]);
    if c1.suit == c2.suit {
        CanonicalHand(vec![
            CanonicalCard {
                rank: c1.rank,
                suit: Some(Suit::S),
            },
            CanonicalCard {
                rank: c2.rank,
                suit: Some(Suit::S),
            },
        ])
    } else {
        CanonicalHand(vec![
            CanonicalCard {
                rank: c1.rank,
                suit: Some(Suit::S),
            },
            CanonicalCard {
                rank: c2.rank,
                suit: Some(Suit::H),
            },
        ])
    }
}

// TODO: refactor this
fn preflop4(hand: &[Card]) -> CanonicalHand {
    const N_SUITED: usize = 2;
    let (c1, c2, c3, c4) = (hand[0], hand[1], hand[2], hand[3]);

    let mut res: [CanonicalCard; N_OMAHA] =
        [c1, c2, c3, c4].map(|c| CanonicalCard {
            rank: c.rank,
            suit: None,
        });

    let suits = Suit::ARR_ALL
        .into_iter()
        .filter(|&suit| count_suit(suit, hand) >= N_SUITED)
        .collect::<Suit4>();

    if suits.count() == 1 {
        for i in 0..N_OMAHA {
            if suits.contains_suit(hand[i].suit) {
                res[i].suit = Some(Suit::S);
            }
        }
    } else if suits.count() == 2 {
        if c1.suit == c2.suit {
            res = [
                CanonicalCard {
                    rank: c1.rank,
                    suit: Some(Suit::S),
                },
                CanonicalCard {
                    rank: c2.rank,
                    suit: Some(Suit::S),
                },
                CanonicalCard {
                    rank: c3.rank,
                    suit: Some(Suit::H),
                },
                CanonicalCard {
                    rank: c4.rank,
                    suit: Some(Suit::H),
                },
            ];
        } else if c1.suit == c3.suit {
            res = [
                CanonicalCard {
                    rank: c1.rank,
                    suit: Some(Suit::S),
                },
                CanonicalCard {
                    rank: c2.rank,
                    suit: Some(Suit::H),
                },
                CanonicalCard {
                    rank: c3.rank,
                    suit: Some(Suit::S),
                },
                CanonicalCard {
                    rank: c4.rank,
                    suit: Some(Suit::H),
                },
            ];
        } else {
            res = [
                CanonicalCard {
                    rank: c1.rank,
                    suit: Some(Suit::S),
                },
                CanonicalCard {
                    rank: c2.rank,
                    suit: Some(Suit::H),
                },
                CanonicalCard {
                    rank: c3.rank,
                    suit: Some(Suit::H),
                },
                CanonicalCard {
                    rank: c4.rank,
                    suit: Some(Suit::S),
                },
            ];
        }
    }

    CanonicalHand(res.to_vec())
}

fn count_suit(suit: Suit, cs: &[Card]) -> usize {
    cs.iter().filter(|c| c.suit == suit).count()
}

fn sort(hand: &[Card]) -> Vec<Card> {
    match hand {
        [_, _] => HandN::<N_HOLDEM>::from_slice(hand).to_vec(),
        [_, _, _, _] => HandN::<N_OMAHA>::from_slice(hand).to_vec(),
        _ => unimplemented!("CanonicalHand only supports hand of 2 or 4 cards"),
    }
}

#[cfg(test)]
mod tests {

    use std::array;

    use quickcheck::TestResult;

    use super::*;
    use crate::{Card64, CardN, cards};

    fn assert_canonical_hand(hand: CanonicalHand, expected: &str) {
        assert_eq!(format!("{hand:?}"), expected);
    }

    #[test]
    fn test_from_str() {
        fn parse(s: &str) -> Result<CanonicalCard, ParseError> {
            s.parse()
        }

        assert_eq!(parse("2s"), Ok(cards!("2s")[0].into()));
        assert_eq!(parse("2 S "), Ok(cards!("2s")[0].into()));
        assert!("".parse::<Card>().is_err());
        assert!("?".parse::<Card>().is_err());
        assert!("2".parse::<Card>().is_err());
        assert!("2k".parse::<Card>().is_err());
        assert_eq!(
            parse("2s?"),
            Err(ParseError::InvalidCard("2s?".to_owned())),
        );
        assert!(parse("").is_err());
        assert!(parse("?").is_err());
        assert!(parse("2").is_err());
        assert!(parse("2k").is_err());
    }

    #[test]
    fn test_eq() {
        fn cc(s: &str) -> CanonicalCard {
            s.parse().unwrap()
        }
        assert_eq!(cc("2s"), cc("2s"));
        assert_ne!(cc("2s"), cc("2h"));
        assert_eq!(cc("2*"), cc("2*"));
    }

    #[test]
    fn test_canonical_hand_preflop() {
        let pf = Board::default();
        assert_canonical_hand(CanonicalHand::new(pf, &cards!("AdKd")), "KsAs");
        assert_canonical_hand(CanonicalHand::new(pf, &cards!("AsKd")), "KsAh");
        assert_canonical_hand(
            CanonicalHand::new(pf, &cards!("AsKs7h2h")),
            "2s7sKhAh",
        );
        assert_canonical_hand(
            CanonicalHand::new(pf, &cards!("AdKh7h2d")),
            "2s7hKhAs",
        );
        assert_canonical_hand(
            CanonicalHand::new(pf, &cards!("AdKd7d2h")),
            "2*7sKsAs",
        );
    }

    #[quickcheck]
    fn test_canonical_hand_postflop(
        board: Board,
        cs: CardN<N_OMAHA>,
    ) -> TestResult {
        if board.is_empty()
            || !(Card64::from(board) & Card64::from(cs.as_slice())).is_empty()
        {
            return TestResult::discard();
        }

        let sorted_hand = HandN::<N_OMAHA>::from_slice(cs.as_slice());
        let hand = CanonicalHand::new(board, sorted_hand.as_slice());
        let suits = board.flush_suits();

        for i in 0..N_OMAHA {
            assert_eq!(
                hand.0[i].suit.is_some(),
                suits.contains_suit(sorted_hand[i].suit)
            );
        }

        TestResult::from_bool(true)
    }

    #[quickcheck]
    fn test_hand_eq(
        cs: CardN<4>,
        suit_relevant: [bool; 4],
        new_suit: Suit,
        new_rank: Rank,
        idx: usize,
    ) {
        fn assert_eq_check(l: &CanonicalHand, r: &CanonicalHand) {
            assert_eq!(l == r, format!("{l:?}") == format!("{r:?}"));
        }

        let hand = HandN::<4>::from_slice(cs.as_slice());
        let left = CanonicalHand(
            array::from_fn::<_, 4, _>(|i| CanonicalCard {
                rank: hand[i].rank,
                suit: if suit_relevant[i] {
                    Some(hand[i].suit)
                } else {
                    None
                },
            })
            .to_vec(),
        );
        let idx = idx % 4;

        // 1) alter suit at random position
        let mut right = left.clone();
        right.0[idx].suit = right.0[idx].suit.map(|_| new_suit);
        assert_eq_check(&left, &right);

        // 2) alter rank at random position
        let mut right = left.clone();
        right.0[idx].rank = new_rank;
        assert_eq_check(&left, &right);

        // 3) flip suit relevance at random position
        let mut right = left.clone();
        right.0[idx].suit = if right.0[idx].suit.is_some() {
            None
        } else {
            Some(new_suit)
        };
        assert_eq_check(&left, &right);
    }
}

#[cfg(all(test, feature = "serde"))]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests_serde {
    use super::*;
    use crate::*;

    #[test]
    fn test_canonical_card_with_suit_ser_de() {
        assert_tokens(
            &CanonicalCard {
                rank: Rank::RA,
                suit: Some(Suit::S),
            },
            &[Token::Str("As")],
        );
    }

    #[test]
    fn test_canonical_card_without_suit_ser_de() {
        assert_tokens(
            &CanonicalCard {
                rank: Rank::R2,
                suit: None,
            },
            &[Token::Str("2*")],
        );
    }
}
