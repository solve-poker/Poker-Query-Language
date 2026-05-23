//! Suit-isomorphic canonical form of a flop-and-turn board.

use crate::{
    Board, Card, Flop, IsomorphicCard, Suit, SuitMap,
    card::equiv::isomorphic_flop::{FlopTexture, IsomorphicFlop},
};

/// Flush-relevant suit pattern across a flop and turn.
#[derive(Clone, Copy)]
pub(super) enum TurnTexture {
    /// No suit has enough cards to threaten a flush.
    NoFlush,
    /// One suit has a live flush draw.
    FlushDraw(Suit),
    /// Two suits each have a live flush draw.
    DblFlushDraw(Suit, Suit),
}

impl TurnTexture {
    /// Classifies the texture of three sorted flop cards plus the turn.
    #[inline]
    pub(super) const fn from_turn(
        f0: Card,
        f1: Card,
        f2: Card,
        t: Card,
    ) -> Self {
        let t = t.suit;

        match FlopTexture::from_sorted(f0, f1, f2) {
            FlopTexture::Monotone(s) => Self::FlushDraw(s),
            FlopTexture::Twotone(f, s) if t.eq(s) => Self::DblFlushDraw(f, s),
            FlopTexture::Twotone(f, _) => Self::FlushDraw(f),
            FlopTexture::Rainbow(s0, _, _) if s0.eq(t) => Self::FlushDraw(s0),
            FlopTexture::Rainbow(_, s1, _) if s1.eq(t) => Self::FlushDraw(s1),
            FlopTexture::Rainbow(_, _, s2) if s2.eq(t) => Self::FlushDraw(s2),
            FlopTexture::Rainbow(_, _, _) => Self::NoFlush,
        }
    }

    /// Builds the [`SuitMap`] that relabels this texture's suits canonically.
    #[inline]
    pub(super) const fn to_suit_map(self) -> SuitMap {
        match self {
            Self::NoFlush => SuitMap::map0(),
            Self::FlushDraw(s) => SuitMap::map1(s),
            Self::DblFlushDraw(flop, turn) => SuitMap::map2(flop, turn),
        }
    }
}

/// Canonical suit-isomorphic representative of a flop-and-turn board.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub(super) struct IsomorphicTurn {
    /// The three flop cards, suit-relabeled and sorted.
    pub(super) flop: [IsomorphicCard; Board::N_FLOP],
    /// The turn card, relabeled with the same suit map.
    pub(super) turn: IsomorphicCard,
}

impl IsomorphicTurn {
    /// Canonical representative of `flop` plus `turn` and the [`SuitMap`] that produced it.
    #[inline]
    pub(super) const fn from_turn(flop: Flop, turn: Card) -> (Self, SuitMap) {
        let [f0, f1, f2] = flop.0;
        let map = TurnTexture::from_turn(f0, f1, f2, turn).to_suit_map();

        (
            Self {
                flop: IsomorphicFlop::relabel(f0, f1, f2, map).0,
                turn: map.iso_card(turn),
            },
            map,
        )
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::*;

    fn iso_turn(s: &str) -> IsomorphicTurn {
        let cs = isocards!(s);

        IsomorphicTurn {
            flop: [cs[0], cs[1], cs[2]],
            turn: cs[3],
        }
    }

    fn from_str(s: &str) -> IsomorphicTurn {
        let b = board!(s);

        IsomorphicTurn::from_turn(b.flop.unwrap(), b.turn.unwrap()).0
    }

    fn assert_iso_turn(s: &str) {
        let (lhs, rhs) = s.split_once("->").unwrap();

        assert_eq!(from_str(lhs), iso_turn(rhs), "{lhs}->{rhs}");
    }

    fn gen_iso_turn() -> FxHashSet<IsomorphicTurn> {
        let mut set = FxHashSet::default();
        for flop in Flop::iter_all::<true>() {
            for &turn in Card::all::<true>() {
                if !flop.contains_card(turn) {
                    set.insert(IsomorphicTurn::from_turn(flop, turn).0);
                }
            }
        }
        set
    }

    #[test]
    fn test_iso_turn() {
        assert_eq!(gen_iso_turn().len(), 13_761);
    }

    #[test]
    fn test_monotone() {
        assert_iso_turn("AsKsQs Js -> QxKxAxJx");
        assert_iso_turn("AhKhQh Jh -> QxKxAxJx");
    }

    #[test]
    fn test_flop_monotone_turn_offsuit() {
        assert_iso_turn("AsKsQs Jh -> QxKxAxJn");
        assert_iso_turn("AsKsQs Jd -> QxKxAxJn");
    }

    #[test]
    fn test_rainbow() {
        assert_iso_turn("AsKhQd Jc -> QnKnAnJn");
        assert_iso_turn("AcKdJh Ts -> JnKnAnTn");
    }

    #[test]
    fn test_two_two() {
        assert_iso_turn("AsKsQh Jh -> QyKxAxJy");
        assert_iso_turn("AhKhQs Js -> QyKxAxJy");
    }

    #[test]
    fn test_twotone() {
        assert_iso_turn("AcKcQs Jh -> QnKxAxJn");
    }

    #[test]
    fn test_flop_and_turn_not_interchangeable() {
        assert_ne!(from_str("AsKhQd Jc"), from_str("AsKhJc Qd"));
    }

    #[quickcheck]
    fn test_suit_permutation_invariant(cards: CardN<4>, a: Suit, b: Suit) {
        let swap = |s: Suit| {
            if s == a {
                b
            } else if s == b {
                a
            } else {
                s
            }
        };
        let remap = |c: Card| Card::new(c.rank, swap(c.suit));

        let cs = cards.as_slice();
        let flop = Flop::from_slice(&cs[0..3]);
        let turn = cs[3];

        let permuted: Vec<Card> = cs[0..3].iter().copied().map(remap).collect();
        let permuted_flop = Flop::from_slice(&permuted);

        assert_eq!(
            IsomorphicTurn::from_turn(flop, turn).0,
            IsomorphicTurn::from_turn(permuted_flop, remap(turn)).0,
        );
    }
}
