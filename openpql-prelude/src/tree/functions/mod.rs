use std::ops;

mod bet;
mod player_index;
mod predicate;
mod street;
mod validate;

pub use bet::*;
pub use player_index::*;
pub use predicate::*;
pub use street::*;
pub use validate::*;

use crate::tree::{AnnotatedAction, AnnotatedActionKind, PlayerIdx};

/// Returns all actions after last chance action
pub fn current_round(history: &[AnnotatedAction]) -> &[AnnotatedAction] {
    fn inner<'a>(
        acc: &'a [AnnotatedAction],
        actions: &'a [AnnotatedAction],
    ) -> &'a [AnnotatedAction] {
        match actions {
            [AnnotatedAction::Chance(_), tail @ ..] => inner(tail, tail),
            [_, tail @ ..] => inner(acc, tail),
            [] => acc,
        }
    }

    inner(history, history)
}

fn filter_player_action<F>(
    hero_id: PlayerIdx,
    history: &[AnnotatedAction],
    predicate: F,
) -> bool
where
    F: Fn(&AnnotatedActionKind) -> bool,
{
    match history {
        [] => false,
        [AnnotatedAction::Act(pid, kind, _), ..]
            if *pid == hero_id && predicate(kind) =>
        {
            true
        }
        [_, tail @ ..] => filter_player_action(hero_id, tail, predicate),
    }
}

fn filter_count<T, F>(acc: T, actions: &[AnnotatedAction], predicate: &F) -> T
where
    T: ops::Add<Output = T> + From<bool>,
    F: Fn(&AnnotatedAction) -> bool,
{
    match actions {
        [] => acc,
        [head, tail @ ..] => {
            filter_count(acc + T::from(predicate(head)), tail, predicate)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        tree::{tests::*, *},
        *,
    };

    #[test]
    fn test_current_round() {
        let bet =
            |i: PlayerIdx, b: Chip| (i, AnnotatedActionKind::Bet, b).into();
        assert_eq!(current_round(&[bet(0, 10)]), [bet(0, 10)]);
        assert_eq!(current_round(&[bet(0, 10), FLOP]), []);
        assert_eq!(
            current_round(&[bet(0, 10), FLOP, bet(0, 20), RIVR, bet(0, 30)]),
            [bet(0, 30)]
        );
    }
}
