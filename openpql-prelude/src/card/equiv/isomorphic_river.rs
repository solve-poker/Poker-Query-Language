//! Suit-isomorphic canonical form of a complete five-card board.

use crate::{
    Board, Card, Flop, IsomorphicCard, SuitMap,
    card::equiv::isomorphic_turn::IsomorphicTurn,
};

/// Canonical suit-isomorphic representative of a complete board.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub(super) struct IsomorphicRiver {
    /// The three flop cards, suit-relabeled and sorted.
    pub(super) flop: [IsomorphicCard; Board::N_FLOP],
    /// The turn card, relabeled with the same suit map.
    pub(super) turn: IsomorphicCard,
    /// The river card, relabeled with the same suit map.
    pub(super) river: IsomorphicCard,
}

impl IsomorphicRiver {
    /// Canonical representative of a complete board and the [`SuitMap`] that produced it.
    #[inline]
    pub(super) const fn from_river(
        flop: Flop,
        turn: Card,
        river: Card,
    ) -> (Self, SuitMap) {
        let (turn, map) = IsomorphicTurn::from_turn(flop, turn);

        (
            Self {
                flop: turn.flop,
                turn: turn.turn,
                river: map.iso_card(river),
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

    fn iso_river(s: &str) -> IsomorphicRiver {
        let cs = isocards!(s);

        IsomorphicRiver {
            flop: [cs[0], cs[1], cs[2]],
            turn: cs[3],
            river: cs[4],
        }
    }

    fn from_str(s: &str) -> IsomorphicRiver {
        let b = board!(s);

        IsomorphicRiver::from_river(
            b.flop.unwrap(),
            b.turn.unwrap(),
            b.river.unwrap(),
        )
        .0
    }

    fn assert_iso_river(s: &str) {
        let (lhs, rhs) = s.split_once("->").unwrap();

        assert_eq!(from_str(lhs), iso_river(rhs), "{lhs}->{rhs}");
    }

    fn gen_iso_river() -> FxHashSet<IsomorphicRiver> {
        let mut set = FxHashSet::default();
        for flop in Flop::iter_all::<true>() {
            for &turn in Card::all::<true>() {
                if flop.contains_card(turn) {
                    continue;
                }
                for &river in Card::all::<true>() {
                    if flop.contains_card(river) || river == turn {
                        continue;
                    }
                    set.insert(
                        IsomorphicRiver::from_river(flop, turn, river).0,
                    );
                }
            }
        }
        set
    }

    #[test]
    fn test_monotone() {
        assert_iso_river("AsKsQs Js Ts -> QxKxAxJxTx");
        assert_iso_river("AhKhQh Jh Th -> QxKxAxJxTx");
    }

    #[test]
    fn test_flush_completes_on_river() {
        assert_iso_river("AsKsQh Jd Ts -> QnKxAxJnTx");
        assert_iso_river("AhKhQs Jd Th -> QnKxAxJnTx");
    }

    #[test]
    fn test_no_flush() {
        assert_iso_river("AsKhQd Jc Ts -> QnKnAnJnTn");
        assert_iso_river("AsKsQh Jd Tc -> QnKxAxJnTn");
    }

    #[test]
    fn test_flush_on_flop_only() {
        assert_iso_river("AsKsQs Jh Td -> QxKxAxJnTn");
    }

    #[test]
    fn test_turn_and_river_not_interchangeable() {
        assert_ne!(from_str("AsKhQd Jc Ts"), from_str("AsKhQd Ts Jc"));
    }

    #[test]
    #[ignore = "slow"]
    fn test_iso_river() {
        assert_eq!(gen_iso_river().len(), 223_884);
    }

    #[quickcheck]
    fn test_suit_permutation_invariant(cards: CardN<5>, a: Suit, b: Suit) {
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
        let river = cs[4];

        let permuted: Vec<Card> = cs[0..3].iter().copied().map(remap).collect();
        let permuted_flop = Flop::from_slice(&permuted);

        assert_eq!(
            IsomorphicRiver::from_river(flop, turn, river).0,
            IsomorphicRiver::from_river(
                permuted_flop,
                remap(turn),
                remap(river)
            )
            .0,
        );
    }
}
