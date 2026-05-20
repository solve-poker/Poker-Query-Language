//! Suit-isomorphic canonical form of a flop-and-turn board.

use crate::{
    Board, Card, IsomorphicCard, IsomorphicFlop, Suit, SuitMap,
    card::equiv::FlopTexture,
};

/// The flop cards followed by the turn card.
type Turn = [Card; Board::N_TURN];

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
    pub(super) const fn from_turn([f0, f1, f2, t]: Turn) -> Self {
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
pub struct IsomorphicTurn {
    /// The three flop cards, suit-relabeled and sorted.
    pub flop: [IsomorphicCard; Board::N_FLOP],
    /// The turn card, relabeled with the same suit map.
    pub turn: IsomorphicCard,
}

impl IsomorphicTurn {
    /// Builds the canonical representative from rank-sorted flop cards and a
    /// turn card.
    #[must_use]
    const fn from_turn([f0, f1, f2, t]: Turn) -> Self {
        let map = TurnTexture::from_turn([f0, f1, f2, t]).to_suit_map();

        Self {
            flop: IsomorphicFlop::from_3_cards(
                map.iso_card(f0),
                map.iso_card(f1),
                map.iso_card(f2),
            )
            .0,
            turn: map.iso_card(t),
        }
    }

    /// Builds the canonical representative from a board's flop and turn.
    /// # Panics
    /// Board must have valid flop and turn
    #[must_use]
    pub const fn from_board(board: Board) -> Self {
        match (board.flop, board.turn) {
            (Some(f), Some(t)) => Self::from_turn([f.0[0], f.0[1], f.0[2], t]),
            _ => panic!("IsomorphicTurn requires a flop and a turn card"),
        }
    }
}

impl Board {
    /// Returns the canonical suit-isomorphic representative of this flop-and-turn board.
    #[must_use]
    pub const fn to_isomorphic_turn(self) -> IsomorphicTurn {
        IsomorphicTurn::from_board(self)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::*;

    fn iso_turn(s: &str) -> IsomorphicTurn {
        let cs = isocards!(s);

        IsomorphicTurn {
            flop: [cs[0], cs[1], cs[2]],
            turn: cs[3],
        }
    }

    fn assert_iso_turn(s: &str) {
        let (lhs, rhs) = s.split_once("->").unwrap();
        assert_eq!(
            board!(lhs).to_isomorphic_turn(),
            iso_turn(rhs),
            "{:?}->{rhs}; but got {:?}",
            board!(lhs),
            board!(lhs).to_isomorphic_turn()
        );
    }

    pub fn gen_iso_turn() -> FxHashSet<IsomorphicTurn> {
        let mut set = FxHashSet::default();
        for flop in Flop::iter_all::<true>() {
            for &turn in Card::all::<true>() {
                if !flop.contains_card(turn) {
                    set.insert(IsomorphicTurn::new(flop, turn));
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
        // flop + turn all one suit.
        assert_iso_turn("AsKsQs Js -> QxKxAxJx");
        assert_iso_turn("AhKhQh Jh -> QxKxAxJx");
    }

    #[test]
    fn test_flop_monotone_turn_offsuit() {
        // monotone flop, off-suit turn -> the lone suit is irrelevant.
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
    fn test_flop_and_turn_not_interchangeable() {
        let a = board!("AsKhQd Jc").to_isomorphic_turn();
        let b = board!("AsKhJc Qd").to_isomorphic_turn();

        assert_ne!(a, b);
    }

    impl IsomorphicTurn {
        #[must_use]
        pub const fn new(flop: Flop, turn: Card) -> Self {
            let [f0, f1, f2] = flop.0;
            Self::from_turn([f0, f1, f2, turn])
        }
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
            IsomorphicTurn::new(flop, turn),
            IsomorphicTurn::new(permuted_flop, remap(turn)),
        );
    }
}
