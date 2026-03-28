//! Player state and action validation utilities.
//!
//! [`AnnotatedAction`] -> bool

use crate::tree::{
    AnnotatedAction, AnnotatedActionKind, PlayerIdx, current_bet,
    current_round, functions::filter_player_action, minimum_raise, num_players,
    player_committed,
};

/// Checks whether all players have folded except for one
pub fn all_folded(history: &[AnnotatedAction]) -> bool {
    fn inner(acc: PlayerIdx, history: &[AnnotatedAction]) -> PlayerIdx {
        match history {
            [] => acc,
            [AnnotatedAction::Join(_, _), tail @ ..] => inner(acc + 1, tail),
            [
                AnnotatedAction::Act(_, AnnotatedActionKind::Fold, _),
                tail @ ..,
            ] => inner(acc - 1, tail),
            [_, tail @ ..] => inner(acc, tail),
        }
    }

    inner(0, history) <= 1
}

/// Checks whether player has folded given full history
pub fn player_folded(hero_id: PlayerIdx, history: &[AnnotatedAction]) -> bool {
    filter_player_action(hero_id, history, |kind| {
        matches!(kind, AnnotatedActionKind::Fold)
    })
}

/// Checks whether player has shoved given full history
pub fn player_shoved(hero_id: PlayerIdx, history: &[AnnotatedAction]) -> bool {
    filter_player_action(hero_id, history, |kind| {
        matches!(
            kind,
            AnnotatedActionKind::ShoveCall
                | AnnotatedActionKind::ShoveBet
                | AnnotatedActionKind::ShoveRaise
        )
    })
}

/// checks whether a player has made an action at the current round
pub fn player_acted(hero_id: PlayerIdx, round: &[AnnotatedAction]) -> bool {
    filter_player_action(hero_id, round, |_| true)
}

/// checks whether a player can fold/check/call at the current round
pub fn player_can_act(hero_id: PlayerIdx, history: &[AnnotatedAction]) -> bool {
    !player_folded(hero_id, history)
        && !player_shoved(hero_id, history)
        && (!player_acted(hero_id, current_round(history))
            || player_committed(hero_id, history) != current_bet(history))
}

/// checks whether a player can raise at the current round
pub fn player_can_raise(
    hero_id: PlayerIdx,
    history: &[AnnotatedAction],
) -> bool {
    !player_acted(hero_id, current_round(history))
        || current_bet(history)
            .saturating_sub(player_committed(hero_id, history))
            >= minimum_raise(history)
}

pub fn active_player_status(history: &[AnnotatedAction]) -> Vec<bool> {
    (0..num_players(history))
        .map(|i| !player_folded(i, history))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_can_raise_47_1() {
        const UTG: PlayerIdx = 2;
        let h = actions!([500,125,500,200], 50/100
          -> turn
          -> sb bet 100
          -> bb shoveraise 125
          -> utg call 125
          -> btn shoveraise 200
          -> sb call 100
        );

        assert_eq!(minimum_raise(&h), 100);
        assert_eq!(minimum_raise(&h[..h.len() - 1]), 100);
        assert_eq!(minimum_raise(&h[..h.len() - 2]), 100);
        assert_eq!(minimum_raise(&h[..h.len() - 3]), 100);
        assert_eq!(minimum_raise(&h[..h.len() - 4]), 100);

        assert!(!player_can_raise(UTG, &h));
    }

    #[test]
    fn test_can_raise_47_1b() {
        const UTG: PlayerIdx = 2;

        let h = actions!([500,125,500,200], 50/100
          -> turn
          -> sb bet 100
          -> bb shoveraise 125
          -> utg call 125
          -> btn shoveraise 200
          -> sb raise 300
        );

        assert!(player_can_raise(UTG, &h));
    }

    #[test]
    fn test_can_raise_47_2() {
        const BTN: PlayerIdx = 5;

        let h = actions!([1000,500,650,800,1000,1000], 50/100
          -> flop
          -> sb bet 300
          -> bb shoveraise 500
          -> utg shoveraise 650
          -> hj shoveraise 800
          -> co call 800
        );

        assert!(player_can_raise(BTN, &h));

        assert_eq!(minimum_raise(&h), 300);
        assert_eq!(minimum_raise(&h[..h.len() - 1]), 300);
        assert_eq!(minimum_raise(&h[..h.len() - 2]), 300);
        assert_eq!(minimum_raise(&h[..h.len() - 3]), 300);
        assert_eq!(minimum_raise(&h[..h.len() - 4]), 300);
    }

    #[test]
    fn test_can_raise_47_3a() {
        const BB: PlayerIdx = 1;
        const UTG: PlayerIdx = 2;

        let h = actions!([7500, 20000, 20000, 20000], 2000/4000
          -> preflop
          -> utg call 4000
          -> btn fold
          -> sb shoveraise 7500
          -> bb call 7500
        );

        assert!(!player_can_raise(UTG, &h));
        assert!(player_can_raise(BB, &h[..h.len() - 1]));

        assert_eq!(minimum_raise(&h), 4000);
        assert_eq!(minimum_raise(&h[..h.len() - 1]), 4000);
        assert_eq!(minimum_raise(&h[..h.len() - 2]), 4000);
        assert_eq!(minimum_raise(&h[..h.len() - 3]), 4000);
    }

    #[test]
    fn test_can_raise_47_3b() {
        const UTG: PlayerIdx = 2;

        let h = actions!([7500, 20000, 20000, 20000], 2000/4000
          -> preflop
          -> utg call 4000
          -> btn fold
          -> sb shoveraise 7500
          -> bb raise 11500
        );

        assert!(player_can_raise(UTG, &h));

        assert_eq!(minimum_raise(&h), 4000);
        assert_eq!(minimum_raise(&h[..h.len() - 1]), 4000);
        assert_eq!(minimum_raise(&h[..h.len() - 2]), 4000);
        assert_eq!(minimum_raise(&h[..h.len() - 3]), 4000);
    }

    #[test]
    fn test_active_player_status() {
        let h = actions!([7500, 20000, 20000, 20000], 2000/4000
          -> preflop
          -> utg call 4000
          -> btn fold
          -> sb shoveraise 7500
          -> bb raise 11500
        );

        assert_eq!(active_player_status(&h), [true, true, true, false]);
    }
}
