use derive_more::Display;

use crate::tree::Chip;

/// Simple Action type for building poker game history.
/// Note: Dealt cards are treated as separate from the game history.
#[derive(
    Clone,
    Copy,
    Display,
    derive_more::Debug,
    PartialEq,
    Eq,
    derive_more::From,
    Hash,
    PartialOrd,
    Ord,
)]
pub enum Action {
    #[display("Chance")]
    #[debug("C")]
    Chance,
    #[display("Bet({_0})")]
    #[debug("{_0}")]
    PlayerBet(Chip),
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
        let actions =
            [Action::Chance, Action::PlayerBet(1), Action::PlayerBet(100)];
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
}
