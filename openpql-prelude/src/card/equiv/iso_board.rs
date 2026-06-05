use std::fmt;

use crate::{
    Board, Card, FlushingSuit, HandN, IsomorphicCard, Rank, Rank16, Suit,
    Suit4, SuitMap,
    card::{
        Rank16Inner,
        equiv::{
            isomorphic_flop::IsomorphicFlop,
            isomorphic_river::IsomorphicRiver,
            isomorphic_turn::IsomorphicTurn,
            util::{n_flush_suits, place_card},
        },
    },
};

/// Canonical suit-isomorphic representative of a board at any street.
#[cfg_attr(feature = "speedy", derive(speedy::Readable, speedy::Writable))] // LCOV_EXCL_LINE
#[derive(
    Copy,
    Clone,
    derive_more::Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Default,
)]
#[debug("IsomorphicBoard<{}>", self)]
pub struct IsomorphicBoard {
    /// The flop cards, suit-relabeled and sorted, or `None` preflop.
    pub flop: Option<[IsomorphicCard; Board::N_FLOP]>,
    /// The turn card, relabeled with the same suit map.
    pub turn: Option<IsomorphicCard>,
    /// The river card, relabeled with the same suit map.
    pub river: Option<IsomorphicCard>,
}

impl Board {
    /// Canonical suit-isomorphic form (for GTO) of `board` and the [`SuitMap`] that produced it.
    pub const fn to_isomorphic(self) -> (IsomorphicBoard, SuitMap) {
        match (self.flop, self.turn, self.river) {
            (Some(flop), None, _) => {
                let (flop, map) = IsomorphicFlop::from_flop(flop);

                (IsomorphicBoard::flop(flop.0), map)
            }
            (Some(flop), Some(turn), None) => {
                let (turn, map) = IsomorphicTurn::from_turn(flop, turn);

                (IsomorphicBoard::flop(turn.flop).with_turn(turn.turn), map)
            }
            (Some(flop), Some(turn), Some(river)) => {
                let (river, map) =
                    IsomorphicRiver::from_river(flop, turn, river);

                (
                    IsomorphicBoard::flop(river.flop)
                        .with_turn(river.turn)
                        .with_river(river.river),
                    map,
                )
            }
            _ => (IsomorphicBoard::EMPTY, SuitMap::new()),
        }
    }
}

impl IsomorphicBoard {
    /// The preflop board: no cards.
    const EMPTY: Self = Self {
        flop: None,
        turn: None,
        river: None,
    };

    /// Builds a flop-only board from relabeled flop cards.
    pub const fn flop(flop: [IsomorphicCard; Board::N_FLOP]) -> Self {
        Self {
            flop: Some(flop),
            ..Self::EMPTY
        }
    }

    /// Adds a relabeled turn card.
    #[must_use]
    pub const fn with_turn(self, turn: IsomorphicCard) -> Self {
        Self {
            flop: self.flop,
            turn: Some(turn),
            river: self.river,
        }
    }

    /// Adds a relabeled river card.
    #[must_use]
    pub const fn with_river(self, river: IsomorphicCard) -> Self {
        Self {
            flop: self.flop,
            turn: self.turn,
            river: Some(river),
        }
    }

    /// Returns the set of ranks present on the board.
    pub const fn ranks(self) -> Rank16 {
        let mut res = Rank16(0);

        if let Some([f0, f1, f2]) = self.flop {
            res.set(f0.rank);
            res.set(f1.rank);
            res.set(f2.rank);
        }

        if let Some(turn) = self.turn {
            res.set(turn.rank);
        }

        if let Some(river) = self.river {
            res.set(river.rank);
        }

        res
    }

    /// Returns the board ranks strictly above `rank`.
    pub const fn ranks_above(self, rank: Rank) -> Rank16 {
        let mask = !((1 << (rank as Rank16Inner + 1)) - 1);
        Rank16(self.ranks().0 & mask)
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

    let river_suit = match river.suit {
        FlushingSuit::N => {
            let n_flush = n_flush_suits(&[flop[0], flop[1], flop[2], turn]);
            let taken = taken_by_rank(river.rank, [c0, c1, c2, c3]);

            match (
                n_flush,
                taken.contains_suit(Suit::S),
                taken.contains_suit(Suit::H),
                taken.contains_suit(Suit::D),
            ) {
                (0, false, _, _) => Suit::S,
                (0..=1, _, false, _) => Suit::H,
                (0..=2, _, _, false) => Suit::D,
                _ => Suit::C,
            }
        }
        _ => to_suit(river.suit),
    };

    [c0, c1, c2, c3, Card::new(river.rank, river_suit)]
}

#[inline]
const fn taken_by_rank(rank: Rank, cs: [Card; 4]) -> Suit4 {
    let mut res = Suit4(0);
    if rank.const_eq(cs[0].rank) {
        res.set(cs[0].suit);
    }
    if rank.const_eq(cs[1].rank) {
        res.set(cs[1].suit);
    }
    if rank.const_eq(cs[2].rank) {
        res.set(cs[2].suit);
    }
    if rank.const_eq(cs[3].rank) {
        res.set(cs[3].suit);
    }
    res
}

#[inline]
const fn to_suit(s: FlushingSuit) -> Suit {
    match s {
        FlushingSuit::X => Suit::S,
        FlushingSuit::Y => Suit::H,
        FlushingSuit::N => Suit::D, // LCOV_EXCL_LINE — place_flop only sees X/Y/Z
        FlushingSuit::Z => Suit::C,
    }
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
        let (got, _) = board!(lhs).to_isomorphic();
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
        let (iso, _) = board!(s).to_isomorphic();
        let (back, _) = iso.to_board().to_isomorphic();

        assert_eq!(back, iso, "{s}: {iso} -> {} -> {back}", iso.to_board());
    }

    #[test]
    fn test_place_river_suit_c_arm() {
        assert_roundtrip("AsAhAd Kh Ac");
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
        let (iso, _) = board.to_isomorphic();
        let (back, _) = iso.to_board().to_isomorphic();

        assert_eq!(
            board.to_card64().count(),
            iso.to_board().to_card64().count()
        );
        assert_eq!(back, iso);
    }

    #[test]
    fn test_preflop_is_empty() {
        let (got, map) = Board::default().to_isomorphic();
        assert_eq!(got, IsomorphicBoard::default());
        assert_eq!(map.0, SuitMap::new().0);
    }

    #[test]
    fn test_ranks_above() {
        let (iso, _) = board!("AsKhQd Jc Ts").to_isomorphic();

        assert_eq!(iso.ranks_above(Rank::RA), Rank16::default());
        assert_eq!(iso.ranks_above(Rank::RK), r16!("A"));
        assert_eq!(iso.ranks_above(Rank::RQ), r16!("KA"));
        assert_eq!(iso.ranks_above(Rank::RT), r16!("JQKA"));
        assert_eq!(iso.ranks_above(Rank::R9), r16!("TJQKA"));
        assert_eq!(iso.ranks_above(Rank::R2), r16!("TJQKA"));
    }

    #[test]
    fn test_place_river() {
        assert_eq!(
            board!("AsKhQd Jc Js")
                .to_isomorphic()
                .0
                .to_board()
                .to_card64()
                .count(),
            5
        );

        assert_eq!(
            board!("AsKhQd Jc Qc")
                .to_isomorphic()
                .0
                .to_board()
                .to_card64()
                .count(),
            5
        );
    }

    #[test]
    fn test_ranks_above_empty() {
        let iso = IsomorphicBoard::default();
        assert_eq!(iso.ranks_above(Rank::R2), Rank16::default());
        assert_eq!(iso.ranks_above(Rank::RA), Rank16::default());
    }
}
