use derive_more::{Display, FromStr};

/// Represents the type of action a player can take in a poker hand.
/// Used to classify parsed player actions in the game tree.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Debug, Clone, Copy, Display, PartialOrd, Ord, PartialEq, Eq, FromStr,
)]
pub enum AnnotatedActionKind {
    Fold,
    Check,
    Call,
    Bet,
    Raise,
    ShoveCall,
    ShoveBet,
    ShoveRaise,
}

#[cfg(all(test, feature = "serde"))]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests_serde {
    use super::*;
    use crate::*;

    fn assert_action_kind(kind: AnnotatedActionKind, s: &'static str) {
        assert_tokens(
            &kind,
            &[Token::UnitVariant {
                name: "AnnotatedActionKind",
                variant: s,
            }],
        );
    }

    #[test]
    fn test_annotated_action_kind_ser_de() {
        assert_action_kind(AnnotatedActionKind::Fold, "Fold");
        assert_action_kind(AnnotatedActionKind::Check, "Check");
        assert_action_kind(AnnotatedActionKind::Call, "Call");
        assert_action_kind(AnnotatedActionKind::Bet, "Bet");
        assert_action_kind(AnnotatedActionKind::Raise, "Raise");
        assert_action_kind(AnnotatedActionKind::ShoveCall, "ShoveCall");
        assert_action_kind(AnnotatedActionKind::ShoveBet, "ShoveBet");
        assert_action_kind(AnnotatedActionKind::ShoveRaise, "ShoveRaise");
    }
}
