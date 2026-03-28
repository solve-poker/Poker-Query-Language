//! Player index utilities for action history analysis.
//!
//! [`AnnotatedAction`] -> `PlayerIdx`
//! (`PlayerIdx`, [`AnnotatedAction`]) -> `PlayerIdx`

use std::iter;

use crate::tree::{
    AnnotatedAction, PlayerIdx, all_folded, current_round,
    functions::filter_count, player_can_act,
};

pub const fn idx_next(n: PlayerIdx, i: PlayerIdx) -> PlayerIdx {
    (i + 1) % n
}

pub const fn idx_prev(n: PlayerIdx, i: PlayerIdx) -> PlayerIdx {
    (i + n - 1) % n
}

/// Count players from actions
pub fn num_players(history: &[AnnotatedAction]) -> PlayerIdx {
    filter_count(0, history, &|a: &AnnotatedAction| {
        matches!(a, AnnotatedAction::Join(_, _))
    })
}

/// Returns the player sit left to hero
pub fn index_next(
    hero_id: PlayerIdx,
    history: &[AnnotatedAction],
) -> PlayerIdx {
    idx_next(num_players(history), hero_id)
}

/// Returns the last acted player of current round (including posting actions)
pub fn last_acted(history: &[AnnotatedAction]) -> Option<PlayerIdx> {
    match current_round(history) {
        [
            ..,
            AnnotatedAction::Post(pid, _) | AnnotatedAction::Act(pid, _, _),
        ] => Some(*pid),
        _ => None,
    }
}

/// Returns the next player to act at current round
pub fn players_after(
    hero_id: PlayerIdx,
    history: &[AnnotatedAction],
) -> impl Iterator<Item = PlayerIdx> {
    iter::successors(Some(index_next(hero_id, history)), |&cur| {
        Some(index_next(cur, history))
    })
    .take_while(move |cur| *cur != hero_id)
}

// TODO: refactor; remove iter
/// Returns the next player to act at current round
pub fn next_to_act(history: &[AnnotatedAction]) -> Option<PlayerIdx> {
    if all_folded(history) {
        None
    } else {
        players_after(
            last_acted(history)
                .unwrap_or_else(|| idx_prev(num_players(history), 0)),
            history,
        )
        .find(|cur| player_can_act(*cur, history))
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::{tree::tests::*, *};

    fn mk_game(n: usize) -> Vec<AnnotatedAction> {
        vec![AnnotatedAction::Join(0, 0); n]
    }

    #[test]
    fn test_num_players() {
        assert_eq!(num_players(&mk_game(5)), 5);
    }

    #[test]
    fn test_players_after() {
        let h = vec![AnnotatedAction::Join(0, 0); 5];
        assert_eq!(players_after(2, &h).collect::<Vec<_>>(), [3, 4, 0, 1]);
        assert_eq!(players_after(0, &h).collect::<Vec<_>>(), [1, 2, 3, 4]);
    }

    fn assert_next_to_act(mut h: Vec<AnnotatedAction>) {
        while let Some(action) = h.pop() {
            if matches!(action, AnnotatedAction::Post(_, _) | PREFLOP) {
                break;
            }

            assert_eq!(
                next_to_act(&h),
                action.player_idx(),
                "{h:?} {action:?}"
            );
        }
    }

    #[test]
    fn test_next_to_act() {
        assert_next_to_act(actions!([100, 100, 100], 1/2
          -> preflop -> btn call 2 -> sb call 2 -> bb check
          -> flop
            -> sb bet 5
            -> bb raise 20
            -> btn raise 50
            -> sb call 50
            -> bb call 50
        ));

        assert_next_to_act(actions!([100, 100, 100], 1/2
          -> preflop -> btn call 2 -> sb fold -> bb check
          -> flop
            -> bb check
            -> btn check
        ));
    }

    #[test]
    fn test_next_to_act_none() {
        let h =
            actions!([100, 100, 100], 1/2 -> preflop -> btn fold -> sb fold);

        assert!(next_to_act(&h).is_none());
    }
}
