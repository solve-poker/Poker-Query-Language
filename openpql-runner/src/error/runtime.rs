use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeError {
    AddOverflow,
    SubOverflow,
    MulOverflow,
    InvalidHand,
    RequiresFiveCards,
    IntegerRequired,
    ValueRetrievalFailed(PQLType),
}
