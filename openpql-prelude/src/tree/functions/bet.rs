use crate::{
    Street,
    tree::{
        AnnotatedAction, AnnotatedActionKind, Chip, PlayerIdx, current_round,
        current_street, num_players,
    },
};

/// Returns a player's starting stack.
#[must_use]
pub fn player_initial_stack(
    hero_id: PlayerIdx,
    history: &[AnnotatedAction],
) -> Chip {
    match history {
        [AnnotatedAction::Join(pid, stack), ..] if *pid == hero_id => *stack,
        [_, tail @ ..] => player_initial_stack(hero_id, tail),
        [] => 0,
    }
}

/// Returns a player's stack after pot contributions.
#[must_use]
pub fn player_remaining_stack(
    hero_id: PlayerIdx,
    history: &[AnnotatedAction],
) -> Chip {
    player_initial_stack(hero_id, history) - pot_contribution(hero_id, history)
}

/// Returns the all-in amount: remaining stack plus current-round commitment.
#[must_use]
pub fn player_shove_amount(
    hero_id: PlayerIdx,
    history: &[AnnotatedAction],
) -> Chip {
    player_remaining_stack(hero_id, history)
        + player_committed(hero_id, history)
}

/// Returns a player's total chips contributed across all rounds.
#[must_use]
pub fn pot_contribution(
    hero_id: PlayerIdx,
    history: &[AnnotatedAction],
) -> Chip {
    fn inner(
        current: Chip,
        previous: Chip,
        hero_id: PlayerIdx,
        history: &[AnnotatedAction],
    ) -> Chip {
        match history {
            [] => current + previous,
            [AnnotatedAction::Chance(_), tail @ ..] => {
                inner(0, current + previous, hero_id, tail)
            }
            [
                AnnotatedAction::Act(pid, _, bet)
                | AnnotatedAction::Post(pid, bet),
                tail @ ..,
            ] if *pid == hero_id => {
                inner(current.max(*bet), previous, hero_id, tail)
            }
            [_, tail @ ..] => inner(current, previous, hero_id, tail),
        }
    }

    inner(0, 0, hero_id, history)
}

/// Returns a player's chips committed in the current round.
#[must_use]
pub fn player_committed(
    hero_id: PlayerIdx,
    history: &[AnnotatedAction],
) -> Chip {
    match history {
        [] | [.., AnnotatedAction::Chance(_)] => 0,
        [
            ..,
            AnnotatedAction::Act(pid, _, bet) | AnnotatedAction::Post(pid, bet),
        ] if *pid == hero_id => *bet,
        [init @ .., _] => player_committed(hero_id, init),
    }
}

/// Returns the largest bet in the current round.
#[must_use]
pub fn current_bet(history: &[AnnotatedAction]) -> Chip {
    fn inner(acc: Chip, history: &[AnnotatedAction]) -> Chip {
        match history {
            [] | [.., AnnotatedAction::Chance(_)] => acc,
            [
                init @ ..,
                AnnotatedAction::Act(_, _, bet) | AnnotatedAction::Post(_, bet),
            ] => inner(acc.max(*bet), init),
            [init @ .., _] => inner(acc, init),
        }
    }

    inner(0, history)
}

/// Returns the minimum legal raise increment.
#[must_use]
pub fn minimum_raise(history: &[AnnotatedAction]) -> Chip {
    fn inner(acc: Chip, history: &[AnnotatedAction]) -> Chip {
        match history {
            [] | [.., AnnotatedAction::Chance(_)] => acc,
            [
                ..,
                AnnotatedAction::Act(
                    _,
                    AnnotatedActionKind::Bet | AnnotatedActionKind::ShoveBet,
                    bet,
                )
                | AnnotatedAction::Post(_, bet),
            ] => acc.max(*bet),
            [
                init @ ..,
                AnnotatedAction::Act(
                    _,
                    AnnotatedActionKind::Raise
                    | AnnotatedActionKind::ShoveRaise,
                    bet,
                ),
            ] => inner(acc.max(bet.saturating_sub(current_bet(init))), init),
            [init @ .., _] => inner(acc, init),
        }
    }

    inner(0, history)
}

/// Returns the bet level of the current round in poker "N-bet" terms.
///
/// Preflop the big blind is the opening bet, so an open is a 2-bet, a
/// re-raise a 3-bet, and so on; a limped pot stays at 1. Postflop counting
/// starts from the first bet (1-bet), so a checked-through round is 0.
#[must_use]
pub fn num_of_raises(history: &[AnnotatedAction]) -> usize {
    let blind = usize::from(current_street(history) == Some(Street::Preflop));

    blind
        + super::filter_count(
            0,
            current_round(history),
            &|a: &AnnotatedAction| {
                matches!(
                    a,
                    AnnotatedAction::Act(
                        _,
                        AnnotatedActionKind::Bet
                            | AnnotatedActionKind::Raise
                            | AnnotatedActionKind::ShoveBet
                            | AnnotatedActionKind::ShoveRaise,
                        _,
                    )
                )
            },
        )
}

/// Player who made the last bet or raise of the round, if any.
#[must_use]
fn aggressor(round: &[AnnotatedAction]) -> Option<PlayerIdx> {
    match round {
        [] => None,
        [
            ..,
            AnnotatedAction::Act(
                pid,
                AnnotatedActionKind::Bet
                | AnnotatedActionKind::Raise
                | AnnotatedActionKind::ShoveBet
                | AnnotatedActionKind::ShoveRaise,
                _,
            ),
        ] => Some(*pid),
        [init @ .., _] => aggressor(init),
    }
}

/// Returns `true` if the last action is a donk bet.
#[must_use]
pub fn is_donk_bet(history: &[AnnotatedAction]) -> bool {
    let [
        init @ ..,
        AnnotatedAction::Act(
            pid,
            AnnotatedActionKind::Bet | AnnotatedActionKind::ShoveBet,
            _,
        ),
    ] = history
    else {
        return false;
    };

    let last_chance = init
        .iter()
        .rposition(|a| matches!(a, AnnotatedAction::Chance(_)))
        .unwrap_or(init.len());

    aggressor(current_round(&init[..last_chance])).is_some_and(|a| a != *pid)
}

/// Returns per-player total contributions indexed by seat.
#[must_use]
pub fn total_pot(history: &[AnnotatedAction]) -> Vec<Chip> {
    (0..num_players(history))
        .map(|i| pot_contribution(i, history))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_initial_stack() {
        let h = actions!([6000; 4], 100 / 200);

        assert_eq!(player_initial_stack(0, &h), 6000);
        assert_eq!(player_initial_stack(1, &h), 6000);
        assert_eq!(player_initial_stack(2, &h), 6000);
        assert_eq!(player_initial_stack(3, &h), 6000);
        assert_eq!(player_initial_stack(4, &h), 0);
    }

    #[test]
    fn test_remaining_stack() {
        let h = actions!([6000; 4], 100 / 200);

        assert_eq!(player_remaining_stack(0, &h), 5900);
        assert_eq!(player_remaining_stack(1, &h), 5800);
        assert_eq!(player_remaining_stack(2, &h), 6000);
        assert_eq!(player_remaining_stack(3, &h), 6000);
        assert_eq!(player_remaining_stack(4, &h), 0);
    }

    #[test]
    fn test_shove_amount() {
        let h = actions!([6000; 4], 100 / 200);

        assert_eq!(player_shove_amount(0, &h), 6000);
        assert_eq!(player_shove_amount(1, &h), 6000);
        assert_eq!(player_shove_amount(2, &h), 6000);
        assert_eq!(player_shove_amount(3, &h), 6000);
    }

    #[test]
    fn test_pot_contribution() {
        let h = actions!([100; 3], 1/2
            -> preflop
            -> btn raise 50
            -> sb call 50
            -> bb call 50
            -> flop
            -> sb bet 20
        );

        assert_eq!(pot_contribution(0, &h), 70);
        assert_eq!(pot_contribution(1, &h), 50);
        assert_eq!(pot_contribution(2, &h), 50);
    }

    #[test]
    fn test_player_committed() {
        let h = actions!([100; 3], 1/2
            -> preflop
            -> btn raise 50
            -> sb call 50
            -> bb call 50
            -> flop
            -> sb bet 20
        );

        assert_eq!(player_committed(0, &h), 20);
        assert_eq!(player_committed(1, &h), 0);
        assert_eq!(player_committed(2, &h), 0);
    }

    #[test]
    fn test_player_committed_blinds() {
        let h = actions!([100; 3], 1 / 2);

        assert_eq!(player_committed(0, &h), 1);
        assert_eq!(player_committed(1, &h), 2);
        assert_eq!(player_committed(2, &h), 0);
    }

    #[test]
    fn test_current_bet() {
        assert_eq!(current_bet(&[AnnotatedAction::Join(0, 10)]), 0); // unreachable
    }

    #[test]
    fn test_min_raise() {
        let h = actions!([100; 3], 1 / 2 -> flop);

        assert_eq!(minimum_raise(&h), 0);
    }

    #[test]
    fn test_min_raise_43_1() {
        let h = actions!([6000; 4], 100/200
          -> preflop
          -> utg bet 600
          -> btn raise 1600
          -> sb raise 3600
          -> bb raise 5600
        );

        assert_eq!(minimum_raise(&h[..h.len() - 1]), 2000);
        assert_eq!(minimum_raise(&h[..h.len() - 2]), 1000);
        assert_eq!(minimum_raise(&h[..h.len() - 3]), 600);
    }

    #[test]
    fn test_min_raise_43_2() {
        let h = actions!([150, 300, 300], 50/100
          -> preflop
          -> btn shoveraise 150
          -> sb raise 250
        );

        assert_eq!(minimum_raise(&h), 100);
        assert_eq!(minimum_raise(&h[..h.len() - 1]), 100);
        assert_eq!(minimum_raise(&h[..h.len() - 2]), 100);
    }

    #[test]
    fn test_min_raise_43_3() {
        let h = actions!([2000; 3], 100/200
          -> turn
          -> sb bet 300
          -> bb raise 1000
          -> btn raise 1700
        );

        assert_eq!(minimum_raise(&h), 700);
        assert_eq!(minimum_raise(&h[..h.len() - 1]), 700);
        assert_eq!(minimum_raise(&h[..h.len() - 2]), 300);
    }

    #[test]
    fn test_min_raise_43_4a() {
        let h = actions!([1000; 4], 25/50
          -> preflop
          -> utg raise 125
          -> btn raise 200
          -> sb raise 500
          -> bb raise 800
        );

        assert_eq!(minimum_raise(&h), 300);
        assert_eq!(minimum_raise(&h[..h.len() - 1]), 300);
        assert_eq!(minimum_raise(&h[..h.len() - 2]), 75);
        assert_eq!(minimum_raise(&h[..h.len() - 3]), 75);
        assert_eq!(minimum_raise(&h[..h.len() - 4]), 50);
    }

    #[test]
    fn test_min_raise_43_4b() {
        let h = actions!([1000; 4], 25/50
          -> preflop
          -> utg raise 500
          -> btn fold
          -> sb fold
          -> bb raise 950
        );

        assert_eq!(minimum_raise(&h), 450);
        assert_eq!(minimum_raise(&h[..h.len() - 1]), 450);
        assert_eq!(minimum_raise(&h[..h.len() - 2]), 450);
        assert_eq!(minimum_raise(&h[..h.len() - 3]), 450);
    }

    #[test]
    fn test_num_of_raises_preflop() {
        let limp = actions!([6000; 4], 100/200
          -> preflop
          -> utg call 200
        );
        assert_eq!(num_of_raises(&limp), 1);

        let h = actions!([6000; 4], 100/200
          -> preflop
          -> utg raise 600
          -> btn raise 1600
          -> sb raise 3600
          -> bb raise 5600
        );

        assert_eq!(num_of_raises(&h), 5);
        assert_eq!(num_of_raises(&h[..h.len() - 1]), 4);
        assert_eq!(num_of_raises(&h[..h.len() - 2]), 3);
        assert_eq!(num_of_raises(&h[..h.len() - 3]), 2);
    }

    #[test]
    fn test_num_of_raises_postflop() {
        let h = actions!([2000; 3], 100/200
          -> turn
          -> sb bet 300
          -> bb raise 1000
          -> btn shoveraise 1700
        );

        assert_eq!(num_of_raises(&h), 3);
        assert_eq!(num_of_raises(&h[..h.len() - 1]), 2);
        assert_eq!(num_of_raises(&h[..h.len() - 2]), 1);

        let checked = actions!([100; 3], 1 / 2 -> flop);
        assert_eq!(num_of_raises(&checked), 0);
    }

    #[test]
    fn test_donk_bet() {
        let h = actions!([100; 3], 1/2
          -> preflop -> btn raise 20 -> sb call 20 -> bb call 20
          -> flop -> sb check -> bb bet 20
        );

        assert!(is_donk_bet(&h));
        assert!(!is_donk_bet(&h[..h.len() - 1]));
        assert!(!is_donk_bet(&h[..h.len() - 2]));

        assert!(!is_donk_bet(&actions!([100; 3], 1/2
          -> preflop -> btn raise 20 -> sb call 20 -> bb call 20
          -> flop -> sb check -> bb check -> btn check
          -> turn -> sb bet 20
        )));
    }

    #[test]
    fn test_not_donk_bet() {
        let h = actions!([1000; 3], 1/2
          -> preflop
          -> btn raise 6
          -> sb call 6
          -> bb call 6
          -> flop
          -> sb check
          -> bb check
          -> btn bet 10
        );

        assert!(!is_donk_bet(&h));

        let h = actions!([1000; 3], 1/2
          -> preflop
          -> btn call 2
          -> sb call 2
          -> bb check
          -> flop
          -> sb bet 10
        );

        assert!(!is_donk_bet(&h));
    }

    #[test]
    fn test_total_pot() {
        let h = actions!([1000; 4], 25/50
          -> preflop
          -> utg raise 500
          -> btn fold
          -> sb fold
          -> bb raise 950
        );

        assert_eq!(total_pot(&h), [25, 950, 500, 0]);
    }
}
