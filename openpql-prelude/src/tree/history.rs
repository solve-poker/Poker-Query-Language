use core::{borrow, fmt};

use derive_more::{Deref, DerefMut};

use crate::tree::{Action, AnnotatedAction};

#[macro_export]
macro_rules! history {
  ($($token:tt),* $(,)?) => {
      $crate::tree::History::from(vec![$($crate::history!(@convert $token)),*])
  };
  (@convert c) => { $crate::tree::Action::Chance };
  (@convert $chip:expr) => { $crate::tree::Action::from($chip) };
}

/// A sequence of actions representing a path through the game tree.
#[derive(
    Clone, Default, PartialEq, Eq, Hash, derive_more::From, Deref, DerefMut,
)]
#[repr(transparent)]
pub struct History(Vec<Action>);

impl History {
    /// Returns the parent history by excluding the last action.
    ///
    /// If the history is empty, returns an empty slice. Otherwise, returns
    /// all actions except the most recent one.
    ///
    /// # Returns
    /// A slice containing all actions except the last, or an empty slice if empty.
    pub fn parent(&self) -> &[Action] {
        match self.0.split_last() {
            Some((_, parent)) => parent,
            None => &[],
        }
    }

    /// Creates a new history by appending an action to the current sequence.
    ///
    /// This method clones the current history and adds the specified action,
    /// returning a new `History` instance without modifying the original.
    ///
    /// # Arguments
    /// * `action` - The action to append to the history
    ///
    /// # Returns
    /// A new `History` containing all previous actions plus the new action.
    #[must_use]
    pub fn with_action(&self, action: Action) -> Self {
        let mut new = self.clone();
        new.push(action);
        new
    }

    #[must_use]
    pub fn with_parsed_action(&self, action: AnnotatedAction) -> Self {
        let mut new = self.clone();
        if let Some(a) = action.to_action() {
            new.push(a);
        }
        new
    }

    /// Creates a root history starting with a chance action.
    ///
    /// The root represents the beginning of the game tree, where cards are dealt
    /// (represented by `Action::Chance`) before any player actions occur.
    ///
    /// # Returns
    /// A new `History` containing only the initial chance action.
    pub fn root() -> Self {
        Self(vec![Action::Chance])
    }
}

impl borrow::Borrow<[Action]> for History {
    fn borrow(&self) -> &[Action] {
        &self.0
    }
}

impl From<&[Action]> for History {
    fn from(actions: &[Action]) -> Self {
        Self(actions.to_vec())
    }
}

impl FromIterator<Action> for History {
    fn from_iter<I: IntoIterator<Item = Action>>(iter: I) -> Self {
        let mut res = Self::default();
        res.0.extend(iter);
        res
    }
}

impl From<&[AnnotatedAction]> for History {
    fn from(actions: &[AnnotatedAction]) -> Self {
        actions
            .iter()
            .filter_map(AnnotatedAction::to_action)
            .collect()
    }
}

impl fmt::Debug for History {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cs = self.0.iter().map(|a| format!("{a:?}")).collect::<Vec<_>>();

        write!(f, "<{}>", cs.join("-"))
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for History {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        fn history_str(history: &History) -> String {
            format!("{history:?}").replace('<', "").replace('>', "")
        }

        serializer.serialize_str(&history_str(self))
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for History {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let text = String::deserialize(deserializer)?;

        let mut actions = vec![];

        for s in text.split('-') {
            use crate::tree::Chip;

            if s == "c" || s == "C" {
                actions.push(Action::Chance);
            } else if let Ok(amount) = s.parse::<Chip>() {
                actions.push(Action::PlayerBet(amount));
            } else {
                unimplemented!();
            }
        }

        Ok(Self::from(actions))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::{AnnotatedActionKind, tests::*};

    #[test]
    fn test_parent() {
        assert_eq!(history!(c, 10, 20).parent(), history!(c, 10).as_slice());
        assert_eq!(history!(c).parent(), []);
        assert_eq!(history!().parent(), []);
    }

    #[test]
    fn test_with_action() {
        assert_eq!(
            history!(c, 10).with_action(Action::Chance),
            history!(c, 10, c)
        );
    }

    #[test]
    fn test_with_parsed_action() {
        assert_eq!(
            history!(c, 10).with_parsed_action(PREFLOP),
            history!(c, 10, c)
        );

        assert_eq!(
            history!(c, 10).with_parsed_action(AnnotatedAction::Join(0, 1)),
            history!(c, 10)
        );
    }

    #[test]
    fn test_root() {
        assert_eq!(History::root(), vec![Action::Chance].into());
    }

    #[test]
    fn test_borrow() {
        use borrow::Borrow;

        let h = history!(c, 10);
        let borrowed: &[Action] = h.borrow();

        assert_eq!(borrowed.len(), 2);
        assert_eq!(borrowed[0], Action::Chance);
        assert_eq!(borrowed[1], Action::PlayerBet(10));
    }

    #[test]
    fn test_from_actions() {
        let v =
            vec![Action::Chance, Action::PlayerBet(10), Action::PlayerBet(20)];

        assert_eq!(history!(c, 10, 20), History::from(v));
    }

    #[test]
    fn test_from_parsed_actions() {
        let v = [
            AnnotatedAction::Join(0, 100),
            AnnotatedAction::Join(1, 100),
            PREFLOP,
            AnnotatedAction::Act(0, AnnotatedActionKind::Bet, 10),
            AnnotatedAction::Act(1, AnnotatedActionKind::Raise, 20),
        ];

        assert_eq!(history!(c, 10, 20), History::from(v.as_slice()));
    }

    #[test]
    fn test_macro() {
        let expected = History::from(
            [Action::Chance, Action::PlayerBet(10), Action::PlayerBet(20)]
                .as_slice(),
        );

        assert_eq!(expected, history!(c, 10, 20));
    }

    #[test]
    fn test_debug() {
        let history = History::from(
            [Action::Chance, Action::PlayerBet(1), Action::PlayerBet(100)]
                .as_slice(),
        );

        assert_eq!(format!("{history:?}"), "<C-1-100>");
    }
}

#[cfg(all(test, feature = "serde"))]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests_serde {
    use super::*;
    use crate::*;

    #[test]
    fn test_history_ser_deser() {
        let h = History::from(vec![
            Action::Chance,
            Action::PlayerBet(100),
            Action::PlayerBet(200),
        ]);

        assert_tokens(&h, &[Token::Str("C-100-200")]);
    }
}
