#![allow(clippy::wildcard_imports)]
#![cfg_attr(test, allow(clippy::needless_pass_by_value))]

#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use std::{
    cmp::Ordering,
    fmt,
    hash::Hasher,
    ops::{Add, AddAssign, *},
    str::FromStr,
};

use derive_more::derive::{AsMut, AsRef, Display, From, Into};
#[cfg(test)]
use quickcheck::{Arbitrary, TestResult};
use tailcall::tailcall;

mod base;
mod concurrency;
mod error;
mod helper;
mod pql_type;
mod range_checker;
mod runner;

pub mod constants;
pub mod functions;
pub mod pql_parser;
pub mod prim;
pub mod range_parser;
pub mod vm;

pub use base::*;
pub use error::*;
#[allow(unused)]
pub use helper::*;
pub use pql_type::*;
use prim::eval;
use range_checker::CachedChecker;
pub use runner::*;

pub type Loc = usize;
pub type LocInfo = (Loc, Loc);

pub type HandRatingInt = i16;

pub const fn eval_holdem7(c: Card64) -> PQLHiRating {
    PQLHiRating::new(eval::holdem7::eval(c.to_u64()))
}

pub const fn eval_shortdeck7(c: Card64) -> PQLHiRating {
    PQLHiRating::new(eval::shortdeck7::eval(c.to_u64()))
}

pub const fn eval_omaha9(player: Card64, board: Card64) -> PQLHiRating {
    PQLHiRating::new(eval::omaha9::eval(player.to_u64(), board.to_u64()))
}

#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, From, Into, AsRef, AsMut,
)]
pub struct DeadCards(Card64);

impl From<&[Card]> for DeadCards {
    fn from(cs: &[Card]) -> Self {
        Card64::from(cs).into()
    }
}

#[cfg(test)]
pub use crate::tests::{CardN, *};

#[cfg(test)]
pub mod tests {
    use std::fmt::{self};

    use derive_more::derive::{From, Index, Into, IntoIterator};
    use regex::Regex;
    use rustc_hash::FxHashSet;

    use super::*;

    #[derive(Clone, Index, From, Into, IntoIterator)]
    pub struct CardN<const N: usize, const SHORT: bool = false>([Card; N]);

    impl<const N: usize, const S: bool> Arbitrary for CardN<N, S> {
        fn arbitrary(_g: &mut quickcheck::Gen) -> Self {
            let mut rng = fastrand::Rng::new();

            let v = if S {
                rng.choose_multiple(Card::ARR_ALL_SHORT.into_iter(), N)
            } else {
                rng.choose_multiple(Card::ARR_ALL.into_iter(), N)
            };

            <[_; N]>::try_from(v).unwrap().into()
        }
    }

    impl<const N: usize, const S: bool> fmt::Debug for CardN<N, S> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let s = format!("{:?}", self.0);

            let replaced =
                Regex::new(r"(Card|,|\[|\])").unwrap().replace_all(&s, "");

            f.write_str(&format!("cards!(\"{replaced}\")"))
        }
    }

    impl<const N: usize, const S: bool> AsRef<[Card]> for CardN<N, S> {
        fn as_ref(&self) -> &[Card] {
            &self.0
        }
    }

    impl<const N: usize, const S: bool> From<CardN<N, S>> for Vec<Card> {
        fn from(cards: CardN<N, S>) -> Self {
            cards.0.into()
        }
    }

    impl<const N: usize, const S: bool> From<CardN<N, S>> for Card64 {
        fn from(cards: CardN<N, S>) -> Self {
            cards.as_ref().into()
        }
    }

    #[allow(clippy::fallible_impl_from)]
    impl<const X: usize, const Y: usize, const Z: usize, const S: bool>
        From<CardN<Z, S>> for (CardN<X, S>, CardN<Y, S>)
    {
        fn from(cards: CardN<Z, S>) -> Self {
            assert!(X + Y <= Z, "Not enough cards {Z} < {X} + {Y} = {}", X + Y);

            let mut x: [_; X] = [Card::default(); X];
            let mut y: [_; Y] = [Card::default(); Y];

            for i in 0..X {
                x[i] = cards[i];
            }

            for j in 0..Y {
                y[j] = cards[X + j];
            }

            (x.into(), y.into())
        }
    }

    #[derive(Debug, Clone)]
    pub struct HandBoardGame {
        pub hand: Vec<Card>,
        pub another_hand: Vec<Card>,
        pub board: Board,
        pub game: PQLGame,
        pub dead: DeadCards,
        pub street: PQLStreet,
    }

    impl Arbitrary for HandBoardGame {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            const N: usize = 4 + 4 + 5 + 3;

            let game = PQLGame::arbitrary(g);
            let n_dead = *g.choose(&[0, 1, 2, 3]).unwrap();

            let (mut cs, n): (Vec<_>, _) = match game {
                PQLGame::Holdem => (CardN::<N>::arbitrary(g).into(), 2),
                PQLGame::Omaha => (CardN::<N>::arbitrary(g).into(), 4),
                PQLGame::ShortDeck => {
                    (CardN::<N, false>::arbitrary(g).into(), 2)
                }
            };

            let hand = cs.drain(0..n).collect();
            let another_hand = cs.drain(0..n).collect();
            let board: Vec<_> = cs.drain(0..5).collect();
            let dead: Vec<_> = cs.drain(0..n_dead).collect();

            Self {
                hand,
                another_hand,
                board: (&board as &[_]).into(),
                game,
                dead: (&dead as &[_]).into(),
                street: PQLStreet::arbitrary(g),
            }
        }
    }

    #[quickcheck]
    fn test_cards_n_destruct(cards: CardN<10>) -> TestResult {
        let (c4, c5): (CardN<4>, CardN<5>) = cards.clone().into();

        TestResult::from_bool(
            c4[0] == cards[0]
                && c4[1] == cards[1]
                && c4[2] == cards[2]
                && c4[3] == cards[3]
                && c5[0] == cards[4]
                && c5[1] == cards[5]
                && c5[2] == cards[6]
                && c5[3] == cards[7]
                && c5[4] == cards[8],
        )
    }

    #[quickcheck]
    fn test_cards_n_distinct(cards: CardN<10>) -> TestResult {
        let v: FxHashSet<_> = cards.into_iter().collect();

        TestResult::from_bool(v.len() == 10)
    }
}
