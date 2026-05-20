//! Suit-isomorphic canonical form of a complete five-card board.

use crate::{
    Board, Card, IsomorphicCard, IsomorphicFlop, card::equiv::TurnTexture,
};

/// The flop cards followed by the turn and river cards.
type River = [Card; Board::N_RIVER];

/// Canonical suit-isomorphic representative of a complete board.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct IsomorphicRiver {
    /// The three flop cards, suit-relabeled and sorted.
    pub flop: [IsomorphicCard; Board::N_FLOP],
    /// The turn card, relabeled with the same suit map.
    pub turn: IsomorphicCard,
    /// The river card, relabeled with the same suit map.
    pub river: IsomorphicCard,
}

impl IsomorphicRiver {
    /// Builds the canonical representative from rank-sorted flop cards, a turn
    /// card, and a river card.
    #[must_use]
    const fn from_river([f0, f1, f2, t, r]: River) -> Self {
        let map = TurnTexture::from_turn([f0, f1, f2, t]).to_suit_map();

        Self {
            flop: IsomorphicFlop::from_3_cards(
                map.iso_card(f0),
                map.iso_card(f1),
                map.iso_card(f2),
            )
            .0,
            turn: map.iso_card(t),
            river: map.iso_card(r),
        }
    }

    /// Builds the canonical representative from a complete board.
    /// # Panics
    /// Board must have a flop, a turn, and a river.
    #[must_use]
    pub const fn from_board(board: Board) -> Self {
        match (board.flop, board.turn, board.river) {
            (Some(f), Some(t), Some(r)) => {
                Self::from_river([f.0[0], f.0[1], f.0[2], t, r])
            }
            _ => {
                panic!("IsomorphicRiver requires a flop, turn, and river card")
            }
        }
    }
}

impl Board {
    /// Returns the canonical suit-isomorphic representative of this complete board.
    #[must_use]
    pub const fn to_isomorphic_river(self) -> IsomorphicRiver {
        IsomorphicRiver::from_board(self)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::*;

    fn iso_river(s: &str) -> IsomorphicRiver {
        let cs = isocards!(s);

        IsomorphicRiver {
            flop: [cs[0], cs[1], cs[2]],
            turn: cs[3],
            river: cs[4],
        }
    }

    fn assert_iso_river(s: &str) {
        let (lhs, rhs) = s.split_once("->").unwrap();
        assert_eq!(
            board!(lhs).to_isomorphic_river(),
            iso_river(rhs),
            "{:?}->{rhs}; but got {:?}",
            board!(lhs),
            board!(lhs).to_isomorphic_river()
        );
    }

    pub fn gen_iso_river() -> FxHashSet<IsomorphicRiver> {
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
                    set.insert(IsomorphicRiver::new(flop, turn, river));
                }
            }
        }
        set
    }

    #[test]
    fn test_monotone() {
        // flop + turn + river all one suit.
        assert_iso_river("AsKsQs Js Ts -> QxKxAxJxTx");
        assert_iso_river("AhKhQh Jh Th -> QxKxAxJxTx");
    }

    #[test]
    fn test_flush_completes_on_river() {
        // two-tone flop, turn off-suit, river completes the flush.
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
        // monotone flop, both later cards off-suit: the flop suit is relevant.
        assert_iso_river("AsKsQs Jh Td -> QxKxAxJnTn");
    }

    #[test]
    fn test_turn_and_river_not_interchangeable() {
        let a = board!("AsKhQd Jc Ts").to_isomorphic_river();
        let b = board!("AsKhQd Ts Jc").to_isomorphic_river();

        assert_ne!(a, b);
    }

    #[test]
    fn test_iso_river() {
        assert_eq!(gen_iso_river().len(), 223_884);
    }

    impl IsomorphicRiver {
        #[must_use]
        pub const fn new(flop: Flop, turn: Card, river: Card) -> Self {
            let [f0, f1, f2] = flop.0;
            Self::from_river([f0, f1, f2, turn, river])
        }
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
            IsomorphicRiver::new(flop, turn, river),
            IsomorphicRiver::new(permuted_flop, remap(turn), remap(river)),
        );
    }
}
