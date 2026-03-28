use derive_more::{Display, FromStr};

/// Represents the type of action a player can take in a poker hand.
/// Used to classify parsed player actions in the game tree.
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
