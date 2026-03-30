//! helper structs for determine relevent suits
//!
//! * preflop, Holdem/Shortdeck -> only need to distinguish suited/offsuit
//! * preflop, Omaha -> either single suited or double suited
//! * flop -> at most 3 potential drawing suits
//! * turn -> at most 2 drawing suits
//! * river -> at most 1 flush suit

use std::{array, fmt};

use crate::{Board, Card, HandN, Rank, Street, Suit, Suit4};

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

impl fmt::Debug for CanonicalCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit.map_or('*', Suit::to_char))
    }
}

/// Canonical N-card hand.
///
/// Cards sorted (same order as [`HandN`]) with suit encoded per card as
/// `Option<Suit>`: `Some(s)` when flush-relevant, `None` otherwise.
/// Suited preflop hands normalise to `Suit::S` (e.g. `AhKh` == `AdKd`).
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CanonicalHand<const N: usize>(pub(crate) [CanonicalCard; N]);

impl<const N: usize> fmt::Debug for CanonicalHand<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for card in &self.0 {
            write!(f, "{card:?}")?;
        }
        Ok(())
    }
}

impl<const N: usize> CanonicalHand<N> {
    /// Derive the canonical hand given the current board.
    ///
    /// A card's suit is relevant when the total number of cards of that suit
    /// (board + hand + remaining board cards) can reach a flush.  With an
    /// empty board this delegates to preflop logic.
    pub fn new(board: Board, hand: HandN<N>) -> Self {
        match Street::from(board) {
            Street::Preflop => Self::preflop(hand),
            _ => Self::postflop(board, hand),
        }
    }

    fn preflop(hand: HandN<N>) -> Self {
        fn cast<const N: usize, const M: usize>(hand: HandN<N>) -> HandN<M> {
            HandN::<M>::from_slice(hand.as_slice())
        }

        let cards: Vec<CanonicalCard> = if N == N_HOLDEM {
            preflop2(cast(hand)).0.to_vec()
        } else if N == N_OMAHA {
            preflop4(cast(hand)).0.to_vec()
        } else {
            unimplemented!()
        };

        let arr: [CanonicalCard; N] =
            cards.try_into().unwrap_or_else(|_| unreachable!());
        Self(arr)
    }

    fn postflop(board: Board, hand: HandN<N>) -> Self {
        let suits = board.flush_suits();
        Self(array::from_fn(|i| {
            let c = hand[i];
            CanonicalCard {
                rank: c.rank,
                suit: if suits.contains_suit(c.suit) {
                    Some(c.suit)
                } else {
                    None
                },
            }
        }))
    }
}

/// Derive the canonical hand with no board (preflop), 2-card holdem.
///
/// All cards are treated as suited when they share the same suit (normalised
/// to `Suit::S`); otherwise all suits are marked irrelevant.
fn preflop2(hand: HandN<N_HOLDEM>) -> CanonicalHand<N_HOLDEM> {
    let [c1, c2] = hand.0;
    if c1.suit == c2.suit {
        CanonicalHand([
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
        CanonicalHand([
            CanonicalCard {
                rank: c1.rank,
                suit: None,
            },
            CanonicalCard {
                rank: c2.rank,
                suit: None,
            },
        ])
    }
}

// TODO: refactor this
fn preflop4(hand: HandN<N_OMAHA>) -> CanonicalHand<N_OMAHA> {
    const N_SUITED: usize = 2;
    let mut res: [CanonicalCard; N_OMAHA] = hand.0.map(|c| CanonicalCard {
        rank: c.rank,
        suit: None,
    });

    let suits = Suit::ARR_ALL
        .into_iter()
        .filter(|&suit| count_suit(suit, &hand.0) >= N_SUITED)
        .collect::<Suit4>();

    if suits.count() == 1 {
        for i in 0..N_OMAHA {
            if suits.contains_suit(hand[i].suit) {
                res[i].suit = Some(Suit::S);
            }
        }
    } else if suits.count() == 2 {
        let [c1, c2, c3, c4] = hand.0;
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

    CanonicalHand(res)
}

fn count_suit(suit: Suit, cs: &[Card]) -> usize {
    cs.iter().filter(|c| c.suit == suit).count()
}

#[cfg(test)]
mod tests {

    use std::array;

    use quickcheck::TestResult;

    use super::*;
    use crate::{Card64, CardN, cards};

    fn assert_canonical_hand<const N: usize>(
        hand: CanonicalHand<N>,
        expected: &str,
    ) {
        assert_eq!(format!("{hand:?}"), expected);
    }

    fn new_hand<const N: usize>(cs: &str) -> HandN<N> {
        HandN::from_slice(cards!(cs).as_slice())
    }

    #[test]
    fn test_canonical_hand_preflop() {
        let pf = Board::default();
        assert_canonical_hand(
            CanonicalHand::<2>::new(pf, new_hand("AdKd")),
            "KsAs",
        );
        assert_canonical_hand(
            CanonicalHand::<2>::new(pf, new_hand::<2>("AsKd")),
            "K*A*",
        );
        assert_canonical_hand(
            CanonicalHand::<4>::new(pf, new_hand::<4>("AsKs7h2h")),
            "2s7sKhAh",
        );
        assert_canonical_hand(
            CanonicalHand::<4>::new(pf, new_hand::<4>("AdKh7h2d")),
            "2s7hKhAs",
        );
        assert_canonical_hand(
            CanonicalHand::<4>::new(pf, new_hand::<4>("AdKd7d2h")),
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
        let hand = CanonicalHand::<N_OMAHA>::new(board, sorted_hand);
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
    fn test_eq(
        cs: CardN<4>,
        suit_relevant: [bool; 4],
        new_suit: Suit,
        new_rank: Rank,
        idx: usize,
    ) {
        fn assert_eq_check(l: &CanonicalHand<4>, r: &CanonicalHand<4>) {
            assert_eq!(l == r, format!("{l:?}") == format!("{r:?}"));
        }

        let hand = HandN::<4>::from_slice(cs.as_slice());
        let left = CanonicalHand(array::from_fn(|i| CanonicalCard {
            rank: hand[i].rank,
            suit: if suit_relevant[i] {
                Some(hand[i].suit)
            } else {
                None
            },
        }));
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
