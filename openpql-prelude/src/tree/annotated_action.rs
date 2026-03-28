use crate::{
    Street,
    tree::{Action, AnnotatedActionKind, Chip, PlayerIdx, idx_prev, to_pid},
};

#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    derive_more::From,
    derive_more::Debug,
)]
pub enum AnnotatedAction {
    #[from(skip)]
    #[debug("P{_0}: {_1}")]
    Join(PlayerIdx, Chip),
    #[debug("P{_0} Post({_1})")]
    Post(PlayerIdx, Chip),
    #[debug("{_0}")]
    Chance(Street),
    #[debug("P{_0} {_1}({_2})")]
    Act(PlayerIdx, AnnotatedActionKind, Chip),
}

impl AnnotatedAction {
    //// TODO: allin post
    //// Players: [SB,BB,UTG,UTG+1,CO,BTN]; [BB,BTN/SB]
    pub fn new_prefix(
        stacks: &[Chip],
        sb: Chip,
        bb: Chip,
        ante: Chip,
    ) -> Vec<Self> {
        fn post_ante_non_blind(
            n: PlayerIdx,
            idx_bb: PlayerIdx,
            ante: Chip,
        ) -> impl Iterator<Item = AnnotatedAction> {
            let n_ante = if ante > 0 { n - 2 } else { 0 };

            (idx_bb + 1..)
                .take(n_ante as usize)
                .map(move |pid| AnnotatedAction::Post(pid % n, ante))
        }

        fn post_ante_blind(
            n: PlayerIdx,
            idx_bb: PlayerIdx,
            ante: Chip,
            sb: Chip,
            bb: Chip,
        ) -> impl Iterator<Item = AnnotatedAction> {
            [
                AnnotatedAction::Post(idx_prev(n, idx_bb), sb + ante),
                AnnotatedAction::Post(idx_bb, bb + ante),
            ]
            .into_iter()
        }

        let n_players = to_pid(stacks.len());
        let idx_bb = PlayerIdx::from(n_players != 2);

        stacks
            .iter()
            .copied()
            .enumerate()
            .map(|(i, stack)| Self::Join(to_pid(i), stack))
            .chain([Self::default()])
            .chain(post_ante_non_blind(n_players, idx_bb, ante))
            .chain(post_ante_blind(n_players, idx_bb, ante, sb, bb))
            .collect()
    }

    pub const fn to_action(&self) -> Option<Action> {
        match self {
            Self::Chance(_) => Some(Action::Chance),
            Self::Act(_, _, bet) => Some(Action::PlayerBet(*bet)),
            _ => None,
        }
    }
}

impl Default for AnnotatedAction {
    fn default() -> Self {
        Self::Chance(Street::Preflop)
    }
}

#[macro_export]
macro_rules! actions {
    (
        $stacks:expr,
        $sb:literal/$bb:literal
        $(-> $player:ident $($action:ident $($amount:literal)?)?)*
    ) => {{
        let mut _res = $crate::tree::AnnotatedAction::new_prefix($stacks.as_slice(), $sb, $bb, 0);
        let _n = $stacks.len();
        $(
          _res.extend($crate::action![_n, $player $(, $action $(, $amount)?)?]);
        )*
        _res
    }};
}

#[macro_export]
macro_rules! action {
    (@player $n:expr, $player:ident) => {{
        let name = stringify!($player);

        let names = match $n {
            2 => vec!["bb", "btn"],
            3 => vec!["sb", "bb", "btn"],
            4 => vec!["sb", "bb", "utg", "btn"],
            5 => vec!["sb", "bb", "utg", "co", "btn"],
            6 => vec!["sb", "bb", "utg", "hj", "co", "btn"],
            _ => unimplemented!(),
        };

        $crate::PlayerIdx::try_from(names.iter().position(|n| *n == name).unwrap()).unwrap()
    }};

    ($_:expr, preflop) => {
        None as Option<AnnotatedAction>
    };
    ($_:expr, $player:ident) => {
        Some(AnnotatedAction::Chance(stringify!($player).parse().unwrap()))
    };
    ($n:expr, $player:ident, $act:ident) => {
        action!($n, $player, $act, 0)
    };
    ($n:expr, $player:ident, $act:ident, $bet:literal) => {
        Some(AnnotatedAction::Act(action!(@player $n, $player), stringify!($act).parse().unwrap(), $bet))
    };
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::tree::{tests::*, *};

    impl AnnotatedAction {
        pub const fn player_idx(self) -> Option<PlayerIdx> {
            match self {
                Self::Chance(_) => None,
                Self::Post(idx, _)
                | Self::Join(idx, _)
                | Self::Act(idx, _, _) => Some(idx),
            }
        }
    }

    #[test]
    fn test_new_prefix() {
        assert_eq!(
            AnnotatedAction::new_prefix(&[30, 30, 30], 1, 2, 5),
            [
                AnnotatedAction::Join(0, 30),
                AnnotatedAction::Join(1, 30),
                AnnotatedAction::Join(2, 30),
                PREFLOP,
                AnnotatedAction::Post(2, 5),
                AnnotatedAction::Post(0, 6),
                AnnotatedAction::Post(1, 7),
            ]
        );

        assert_eq!(
            AnnotatedAction::new_prefix(&[30, 30], 1, 2, 5),
            [
                AnnotatedAction::Join(0, 30),
                AnnotatedAction::Join(1, 30),
                PREFLOP,
                AnnotatedAction::Post(1, 6),
                AnnotatedAction::Post(0, 7),
            ]
        );
    }

    #[test]
    fn test_to_action() {
        assert_eq!(AnnotatedAction::Join(0, 100).to_action(), None);
        assert_eq!(AnnotatedAction::Post(0, 10).to_action(), None);
        assert_eq!(
            AnnotatedAction::Chance(Street::Flop).to_action(),
            Some(Action::Chance)
        );
        assert_eq!(
            AnnotatedAction::Act(0, AnnotatedActionKind::Bet, 20).to_action(),
            Some(Action::PlayerBet(20))
        );
    }

    #[test]
    fn test_macro() {
        assert_eq!(
            actions!([30, 30, 30], 1/2
              -> preflop -> btn call 2 -> sb call 2 -> bb check 2
              -> flop -> sb bet 5
            ),
            [
                AnnotatedAction::Join(0, 30),
                AnnotatedAction::Join(1, 30),
                AnnotatedAction::Join(2, 30),
                PREFLOP,
                AnnotatedAction::Post(0, 1),
                AnnotatedAction::Post(1, 2),
                AnnotatedAction::Act(2, AnnotatedActionKind::Call, 2),
                AnnotatedAction::Act(0, AnnotatedActionKind::Call, 2),
                AnnotatedAction::Act(1, AnnotatedActionKind::Check, 2),
                FLOP,
                AnnotatedAction::Act(0, AnnotatedActionKind::Bet, 5),
            ]
        );
    }

    #[test]
    fn test_default() {
        assert_eq!(AnnotatedAction::default(), PREFLOP);
    }
}
