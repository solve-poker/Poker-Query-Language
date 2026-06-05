use crate::{
    Board, HandN, IsomorphicHand, SuitMap,
    card::equiv::{
        IsomorphicRiverEv, IsomorphicTurnEv, isomorphic_flop::IsomorphicFlop,
    },
};

impl Board {
    /// Canonical suit-isomorphic form (for ev) of `board` and the [`SuitMap`] that produced it.
    pub const fn to_isomorphic_ev(self) -> (IsomorphicHand, SuitMap) {
        match (self.flop, self.turn, self.river) {
            (Some(flop), None, _) => {
                let (flop, map) = IsomorphicFlop::from_flop(flop);

                (IsomorphicHand::from_arr(flop.0), map)
            }
            (Some(HandN([f0, f1, f2])), Some(turn), None) => {
                let (turn, map) =
                    IsomorphicTurnEv::from_cards(&[f0, f1, f2, turn]);

                (IsomorphicHand::from_arr(turn.0), map)
            }
            (Some(HandN([f0, f1, f2])), Some(turn), Some(river)) => {
                let (river, map) =
                    IsomorphicRiverEv::from_cards(&[f0, f1, f2, turn, river]);

                (IsomorphicHand::from_arr(river.0), map)
            }
            _ => (IsomorphicHand::new(), SuitMap::new()),
        }
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use crate::*;

    #[cfg(feature = "rayon")]
    #[test]
    #[ignore = "slow"]
    fn test_iso_board_ev() {
        use rayon::prelude::*;

        type IsoSets = (
            FxHashSet<IsomorphicHand>,
            FxHashSet<IsomorphicHand>,
            FxHashSet<IsomorphicHand>,
        );

        fn merge_sets(mut a: IsoSets, b: IsoSets) -> IsoSets {
            a.0.extend(b.0);
            a.1.extend(b.1);
            a.2.extend(b.2);

            a
        }

        fn empty_sets() -> IsoSets {
            (
                FxHashSet::default(),
                FxHashSet::default(),
                FxHashSet::default(),
            )
        }

        fn collect_flop(
            (mut set_flop, mut set_turn, mut set_river): IsoSets,
            flop: Flop,
        ) -> IsoSets {
            let board = Board::from(flop);
            let (iso_flop, _) = board.to_isomorphic_ev();
            let _ = set_flop.insert(iso_flop);

            for turn in (!board.to_card64()).iter() {
                let board = board.with_turn(turn);

                let (iso_turn, _) = board.to_isomorphic_ev();
                let _ = set_turn.insert(iso_turn);

                for river in (!board.to_card64()).iter() {
                    let board = board.with_river(river);

                    let (iso_river, _) = board.to_isomorphic_ev();
                    let _ = set_river.insert(iso_river);
                }
            }

            (set_flop, set_turn, set_river)
        }

        let (set_flop, set_turn, set_river) = Flop::iter_all::<false>()
            .par_bridge()
            .fold(empty_sets, collect_flop)
            .reduce(empty_sets, merge_sets);

        assert_eq!(set_flop.len(), 1755);
        assert_eq!(set_turn.len(), 16718);
        assert_eq!(set_river.len(), 42783);
    }

    #[test]
    fn test_iso_board_ev_preflop() {
        let _ = Board::new().to_isomorphic_ev();
    }
}
