use std::fmt;

use crate::{
    Board, Card, FlushingSuit, HandN, IsomorphicCard, Suit, SuitMap,
    card::equiv::{
        isomorphic_flop::IsomorphicFlop, isomorphic_river::IsomorphicRiver,
        isomorphic_turn::IsomorphicTurn,
    },
};

/// Canonical suit-isomorphic representative of a board at any street.
#[cfg_attr(feature = "speedy", derive(speedy::Readable, speedy::Writable))]
#[derive(Copy, Clone, derive_more::Debug, PartialEq, Eq, Hash, Default)]
#[debug("IsomorphicBoard<{}>", self)]
pub struct IsomorphicBoard {
    /// The flop cards, suit-relabeled and sorted, or `None` preflop.
    pub flop: Option<[IsomorphicCard; Board::N_FLOP]>,
    /// The turn card, relabeled with the same suit map.
    pub turn: Option<IsomorphicCard>,
    /// The river card, relabeled with the same suit map.
    pub river: Option<IsomorphicCard>,
}

impl IsomorphicBoard {
    /// Canonical suit-isomorphic form of `board` and the [`SuitMap`] that produced it.
    pub const fn to_isomorphic(board: Board) -> (Self, SuitMap) {
        match (board.flop, board.turn, board.river) {
            (Some(flop), None, _) => {
                let (flop, map) = IsomorphicFlop::from_flop(flop);

                (Self::flop(flop.0), map)
            }
            (Some(flop), Some(turn), None) => {
                let (turn, map) = IsomorphicTurn::from_turn(flop, turn);

                (Self::flop(turn.flop).with_turn(turn.turn), map)
            }
            (Some(flop), Some(turn), Some(river)) => {
                let (river, map) =
                    IsomorphicRiver::from_river(flop, turn, river);

                (
                    Self::flop(river.flop)
                        .with_turn(river.turn)
                        .with_river(river.river),
                    map,
                )
            }
            _ => (Self::EMPTY, SuitMap::new()),
        }
    }

    /// The preflop board: no cards.
    const EMPTY: Self = Self {
        flop: None,
        turn: None,
        river: None,
    };

    /// Builds a flop-only board from relabeled flop cards.
    const fn flop(flop: [IsomorphicCard; Board::N_FLOP]) -> Self {
        Self {
            flop: Some(flop),
            ..Self::EMPTY
        }
    }

    /// Adds a relabeled turn card.
    const fn with_turn(mut self, turn: IsomorphicCard) -> Self {
        self.turn = Some(turn);
        self
    }

    /// Adds a relabeled river card.
    const fn with_river(mut self, river: IsomorphicCard) -> Self {
        self.river = Some(river);
        self
    }

    /// Iterates the relabeled cards in board order.
    pub fn iter(&self) -> impl Iterator<Item = IsomorphicCard> + '_ {
        self.flop
            .iter()
            .flat_map(|flop| flop.iter().copied())
            .chain(self.turn)
            .chain(self.river)
    }

    /// Convert to a `Board` with concrete suits
    pub const fn to_board(&self) -> Board {
        match (self.flop, self.turn, self.river) {
            (Some(f), None, _) => create_board(&place_flop(f)),
            (Some(f), Some(t), None) => create_board(&place_turn(f, t)),
            (Some(f), Some(t), Some(r)) => create_board(&place_river(f, t, r)),
            _ => Board::new(),
        }
    }
}

#[inline]
const fn create_board(cs: &[Card]) -> Board {
    match cs.len() {
        Board::N_FLOP => Board {
            flop: Some(HandN([cs[0], cs[1], cs[2]])),
            turn: None,
            river: None,
        },

        Board::N_TURN => Board {
            flop: Some(HandN([cs[0], cs[1], cs[2]])),
            turn: Some(cs[3]),
            river: None,
        },

        _ => Board {
            flop: Some(HandN([cs[0], cs[1], cs[2]])),
            turn: Some(cs[3]),
            river: Some(cs[4]),
        },
    }
}

#[inline]
const fn place_flop(
    flop: [IsomorphicCard; Board::N_FLOP],
) -> [Card; Board::N_FLOP] {
    let [f0, f1, f2] = flop;

    [
        Card::new(f0.rank, to_suit(f0.suit)),
        Card::new(f1.rank, to_suit(f1.suit)),
        Card::new(f2.rank, to_suit(f2.suit)),
    ]
}

#[inline]
const fn place_turn(
    flop: [IsomorphicCard; Board::N_FLOP],
    turn: IsomorphicCard,
) -> [Card; Board::N_TURN] {
    let [f0, f1, f2] = flop;
    let k = n_flush_suits(&[f0, f1, f2, turn]);

    let (c0, k) = place_card(f0, k);
    let (c1, k) = place_card(f1, k);
    let (c2, k) = place_card(f2, k);
    let (c3, _) = place_card(turn, k);

    [c0, c1, c2, c3]
}

#[inline]
const fn place_river(
    flop: [IsomorphicCard; Board::N_FLOP],
    turn: IsomorphicCard,
    river: IsomorphicCard,
) -> [Card; Board::N_RIVER] {
    let [c0, c1, c2, c3] = place_turn(flop, turn);

    [c0, c1, c2, c3, Card::new(river.rank, to_suit(river.suit))]
}

#[inline]
const fn to_suit(s: FlushingSuit) -> Suit {
    match s {
        FlushingSuit::X => Suit::S,
        FlushingSuit::Y => Suit::H,
        FlushingSuit::N => Suit::D,
        FlushingSuit::Z => Suit::C,
    }
}

#[inline]
const fn n_flush_suits(cards: &[IsomorphicCard]) -> usize {
    let (mut x, mut y, mut z) = (false, false, false);
    let mut idx = 0;

    while idx < cards.len() {
        match cards[idx].suit {
            FlushingSuit::X => x = true,
            FlushingSuit::Y => y = true,
            FlushingSuit::Z => z = true,
            FlushingSuit::N => {}
        }
        idx += 1;
    }

    x as usize + y as usize + z as usize
}

#[inline]
const fn place_card(c: IsomorphicCard, next: usize) -> (Card, usize) {
    const fn take(suit: FlushingSuit, next: usize) -> (Suit, usize) {
        match suit {
            FlushingSuit::X => (Suit::S, next),
            FlushingSuit::Y => (Suit::H, next),
            FlushingSuit::Z => (Suit::D, next),
            FlushingSuit::N => (Suit::ARR_ALL[next], next + 1),
        }
    }

    let (s, next) = take(c.suit, next);

    (Card::new(c.rank, s), next)
}

impl fmt::Display for IsomorphicBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.iter().try_for_each(|card| write!(f, "{card}"))
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::*;

    fn assert_iso(s: &str) {
        let (lhs, rhs) = s.split_once("->").unwrap();
        let (got, _) = IsomorphicBoard::to_isomorphic(board!(lhs));
        assert_eq!(
            got.to_string(),
            rhs.replace(char::is_whitespace, ""),
            "{:?}->{rhs}; but got {got}",
            board!(lhs),
        );
    }

    #[test]
    fn test_flop() {
        assert_iso("6s6hAs -> 6x6yAx");
        assert_iso("AsKsQs -> QxKxAx");
        assert_iso("AsKhQd -> QxKyAz");
    }

    #[test]
    fn test_turn() {
        assert_iso("AsKsQs Js -> QxKxAxJx");
        assert_iso("AsKsQs Jh -> QxKxAxJn");
        assert_iso("AsKhQd Jc -> QnKnAnJn");
        assert_iso("AsKsQh Jh -> QyKxAxJy");
    }

    #[test]
    fn test_river() {
        assert_iso("AsKsQs Js Ts -> QxKxAxJxTx");
        assert_iso("AsKsQh Jd Ts -> QnKxAxJnTx");
        assert_iso("AsKhQd Jc Ts -> QnKnAnJnTn");
    }

    /// `to_board` must produce a board that re-isomorphizes to `iso`.
    fn assert_roundtrip(s: &str) {
        let (iso, _) = IsomorphicBoard::to_isomorphic(board!(s));
        let (back, _) = IsomorphicBoard::to_isomorphic(iso.to_board());

        assert_eq!(back, iso, "{s}: {iso} -> {} -> {back}", iso.to_board());
    }

    #[test]
    fn test_to_board_roundtrip() {
        // Flop textures.
        assert_roundtrip("AsKsQs");
        assert_roundtrip("AsKsQh");
        assert_roundtrip("AsKhQd");
        // Turn: flush draw, double draw, and no-flush (all irrelevant).
        assert_roundtrip("AsKsQs Jh");
        assert_roundtrip("AsKsQh Jh");
        assert_roundtrip("AsKhQd Jc");
        // River: flush completes, flush on flop only, and no flush at all.
        assert_roundtrip("AsKsQh Jd Ts");
        assert_roundtrip("AsKsQs Jh Td");
        assert_roundtrip("AsKhQd Jc Ts");
    }

    #[quickcheck]
    fn test_to_board_roundtrip_all(board: Board) {
        let (iso, _) = IsomorphicBoard::to_isomorphic(board);
        let (back, _) = IsomorphicBoard::to_isomorphic(iso.to_board());

        assert_eq!(back, iso);
    }

    #[test]
    fn test_preflop_is_empty() {
        let (got, map) = IsomorphicBoard::to_isomorphic(Board::default());
        assert_eq!(got, IsomorphicBoard::default());
        assert_eq!(map.0, SuitMap::new().0);
    }
}
