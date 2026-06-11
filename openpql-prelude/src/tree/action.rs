use std::str::FromStr;

use crate::tree::{Chip, TreeParseError};

/// A game-tree action, either a chance event or a player bet.
#[derive(
    Clone,
    Copy,
    derive_more::Display,
    derive_more::Debug,
    PartialEq,
    Eq,
    derive_more::From,
    Hash,
    PartialOrd,
    Ord,
)]
pub enum Action {
    /// Dealing of community cards.
    #[display("Chance")]
    #[debug("C")]
    Chance,
    /// Player bet of the wrapped chip amount.
    #[display("Bet({_0})")]
    #[debug("{_0}")]
    PlayerBet(Chip),
}

const fn is_chance_str(s: &str) -> bool {
    s.eq_ignore_ascii_case("c") || s.eq_ignore_ascii_case("chance")
}

impl FromStr for Action {
    type Err = TreeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if is_chance_str(s) {
            return Ok(Self::Chance);
        }

        if let Ok(chip) = s.parse::<Chip>() {
            return Ok(Self::PlayerBet(chip));
        }

        Err(TreeParseError::InvalidAction(s.into()))
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Action {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Chance => serializer.serialize_str("C"),
            Self::PlayerBet(v) => serializer.serialize_u16(*v),
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Action {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        #[derive(serde::Deserialize)]
        #[serde(untagged)]
        enum Helper {
            Str(String),
            Num(Chip),
        }

        match Helper::deserialize(deserializer)? {
            Helper::Str(s) if is_chance_str(&s) => Ok(Self::Chance),
            Helper::Str(s) => Err(Error::custom(format!("unknown action: {s}"))),
            Helper::Num(v) => Ok(Self::PlayerBet(v)),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_ordering() {
        let chance = Action::Chance;
        let bet1 = Action::PlayerBet(100);
        let bet2 = Action::PlayerBet(200);

        assert!(chance < bet1);
        assert!(bet1 < bet2);
    }

    #[test]
    fn test_from_chip() {
        let chip = 150;
        let action: Action = chip.into();

        assert_eq!(action, Action::PlayerBet(chip));
    }

    #[test]
    fn test_debug() {
        let actions = [Action::Chance, Action::PlayerBet(1), Action::PlayerBet(100)];
        let expected = ["C", "1", "100"];

        for i in 0..actions.len() {
            assert_eq!(format!("{:?}", actions[i]), expected[i]);
        }
    }

    #[test]
    fn test_display() {
        assert_eq!(Action::Chance.to_string(), "Chance");
        assert_eq!(Action::PlayerBet(99).to_string(), "Bet(99)");
    }

    #[test]
    fn test_from_str_chance() {
        assert_eq!(Action::from_str("c"), Ok(Action::Chance));
        assert_eq!(Action::from_str("C"), Ok(Action::Chance));
        assert_eq!(Action::from_str("chance"), Ok(Action::Chance));
        assert_eq!(Action::from_str("Chance"), Ok(Action::Chance));
        assert_eq!(Action::from_str(" c "), Ok(Action::Chance));
    }

    #[test]
    fn test_from_str_player_bet() {
        assert_eq!(Action::from_str("0"), Ok(Action::PlayerBet(0)));
        assert_eq!(Action::from_str("100"), Ok(Action::PlayerBet(100)));
        assert_eq!(Action::from_str(" 42 "), Ok(Action::PlayerBet(42)));
    }

    #[test]
    fn test_from_str_invalid() {
        assert_eq!(
            Action::from_str("bogus"),
            Err(TreeParseError::InvalidAction("bogus".into()))
        );
    }
}

#[cfg(all(test, feature = "serde"))]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests_serde {
    use super::*;
    use crate::*;

    #[test]
    fn test_action_chance_ser_de() {
        assert_tokens(&Action::Chance, &[Token::Str("C")]);
    }

    #[test]
    fn test_action_player_bet_ser_de() {
        assert_tokens(&Action::PlayerBet(100), &[Token::U16(100)]);
    }

    #[test]
    fn test_action_unknown_string_err() {
        assert_de_tokens_error::<Action>(&[Token::Str("bogus")], "unknown action: bogus");
    }
}
