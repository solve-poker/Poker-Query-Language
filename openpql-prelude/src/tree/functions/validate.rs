use std::cmp;

use crate::tree::{
    AnnotatedAction, AnnotatedActionKind, Chip, GameTreeError, PlayerIdx,
    current_bet, minimum_raise, next_to_act, player_can_raise,
    player_committed, player_shove_amount,
};

pub fn validate_bet(
    history: &[AnnotatedAction],
    bet: Chip,
) -> Result<AnnotatedAction, GameTreeError> {
    next_to_act(history).map_or(Err(GameTreeError::ActionNotAllowed), |pid| {
        validate_player_bet(history, pid, bet)
    })
}

fn validate_player_bet(
    history: &[AnnotatedAction],
    hero_id: PlayerIdx,
    bet: Chip,
) -> Result<AnnotatedAction, GameTreeError> {
    use cmp::Ordering::{Equal, Greater, Less};
    let facing_bet = current_bet(history);

    let kind = match bet.cmp(&facing_bet) {
        Less => validate_bet_lt(facing_bet, history, hero_id, bet),
        Equal => validate_bet_eq(facing_bet, history, hero_id, bet),
        Greater => validate_bet_gt(facing_bet, history, hero_id, bet),
    };

    kind.map(|kind| AnnotatedAction::from((hero_id, kind, bet)))
}

fn validate_bet_lt(
    facing_bet: Chip,
    history: &[AnnotatedAction],
    hero_id: PlayerIdx,
    bet: Chip,
) -> Result<AnnotatedActionKind, GameTreeError> {
    let shove = validate_bet_le_shove(history, hero_id, bet)?;
    if bet == shove {
        Ok(AnnotatedActionKind::ShoveCall)
    } else if bet == 0 && player_committed(hero_id, history) < facing_bet {
        Ok(AnnotatedActionKind::Fold)
    } else {
        Err(GameTreeError::BetAmountInvalid(
            hero_id, bet, facing_bet, shove,
        ))
    }
}

fn validate_bet_eq(
    facing_bet: Chip,
    history: &[AnnotatedAction],
    hero_id: PlayerIdx,
    bet: Chip,
) -> Result<AnnotatedActionKind, GameTreeError> {
    let shove = validate_bet_le_shove(history, hero_id, bet)?;
    if bet == shove {
        Ok(AnnotatedActionKind::ShoveCall)
    } else if player_committed(hero_id, history) == facing_bet {
        Ok(AnnotatedActionKind::Check)
    } else {
        Ok(AnnotatedActionKind::Call)
    }
}

// TODO: min bet
fn validate_bet_gt(
    facing_bet: Chip,
    history: &[AnnotatedAction],
    hero_id: PlayerIdx,
    bet: Chip,
) -> Result<AnnotatedActionKind, GameTreeError> {
    let shove = validate_bet_le_shove(history, hero_id, bet)?;

    if facing_bet == 0 {
        if bet < shove {
            Ok(AnnotatedActionKind::Bet)
        } else {
            Ok(AnnotatedActionKind::ShoveBet)
        }
    } else if player_can_raise(hero_id, history) {
        if bet.saturating_sub(facing_bet) >= minimum_raise(history) {
            if bet < shove {
                Ok(AnnotatedActionKind::Raise)
            } else {
                Ok(AnnotatedActionKind::ShoveRaise)
            }
        } else {
            Err(GameTreeError::RaiseTooSmall(hero_id, bet))
        }
    } else {
        Err(GameTreeError::RaiseNotAllowed(hero_id))
    }
}

fn validate_bet_le_shove(
    history: &[AnnotatedAction],
    hero_id: PlayerIdx,
    bet: Chip,
) -> Result<Chip, GameTreeError> {
    let shove = player_shove_amount(hero_id, history);

    if bet > shove {
        Err(GameTreeError::BetExceedsStack(hero_id, bet, shove))
    } else {
        Ok(shove)
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::*;

    fn assert_validate(mut h: Vec<AnnotatedAction>) {
        while let Some(action) = h.pop() {
            if let AnnotatedAction::Act(_, _, bet) = action {
                assert_eq!(
                    validate_bet(&h, bet),
                    Ok(action),
                    "{h:?} {action:?}"
                );
            }
        }
    }

    #[test]
    fn test_validate_bet() {
        assert_validate(actions!([100, 100, 100], 1/2
          -> preflop -> btn call 2 -> sb call 2 -> bb check 2
          -> flop
            -> sb bet 5
            -> bb raise 20
            -> btn raise 50
            -> sb call 50
            -> bb call 50
        ));

        assert_validate(actions!([100, 100, 100], 1/2
          -> preflop -> btn call 2 -> sb call 2 -> bb check 2
          -> flop
            -> sb shovebet 98
            -> bb shovecall 98
            -> btn fold
        ));

        assert_validate(actions!([100, 50], 1/2
          -> preflop -> btn call 2 -> bb check 2
          -> flop
            -> bb bet 10
            -> btn raise 30
            -> bb shoveraise 98
            -> btn shovecall 48
        ));
    }

    fn to_bet(a: AnnotatedAction) -> Chip {
        match a {
            AnnotatedAction::Act(_, _, bet) => bet,
            _ => unreachable!(),
        }
    }

    fn assert_err(mut h: Vec<AnnotatedAction>, err: GameTreeError) {
        let action = h.pop().unwrap();
        assert_eq!(validate_bet(&h, to_bet(action)), Err(err));
    }

    #[test]
    fn test_invalid_bet() {
        const BB: PlayerIdx = 0;
        const BTN: PlayerIdx = 1;
        assert_err(
            actions!([10, 10], 1/2 -> preflop -> btn call 2 -> bb fold),
            GameTreeError::BetAmountInvalid(BB, 0, 2, 10),
        );

        assert_err(
            actions!([10, 10], 1/2 -> flop -> bb shovebet 100),
            GameTreeError::BetExceedsStack(BB, 100, 8),
        );

        assert_err(
            actions!([10, 10], 1/2 -> preflop -> btn shoveraise 100),
            GameTreeError::BetExceedsStack(BTN, 100, 10),
        );

        assert_err(
            actions!([10, 5], 1/2
                -> preflop -> btn call 2 -> bb check 2
                -> flop -> bb shovebet 8 -> btn shovecall 8
            ),
            GameTreeError::BetExceedsStack(BTN, 8, 3),
        );

        assert_err(
            actions!([10, 5], 1/2
                -> preflop -> btn call 2 -> bb check 2
                -> flop -> bb shovebet 8 -> btn shovecall 5
            ),
            GameTreeError::BetExceedsStack(BTN, 5, 3),
        );
    }

    #[test]
    fn test_raise_err() {
        let btn: PlayerIdx = 1;
        assert_err(
            actions!([100, 100], 1/2
                -> preflop -> btn call 2 -> bb check 2
                -> flop -> bb bet 10 -> btn raise 19
            ),
            GameTreeError::RaiseTooSmall(btn, 19),
        );

        let sb: PlayerIdx = 0;
        assert_err(
            actions!([200, 100, 200], 1/2
                -> preflop -> btn call 2 -> sb call 2 -> bb check 2
                -> flop -> sb bet 80 -> bb shoveraise 100 -> btn call 100
                  -> sb shoveraise 198
            ),
            GameTreeError::RaiseNotAllowed(sb),
        );
    }
}
