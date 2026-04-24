/// Unary operator in an expression.
#[derive(Debug, Clone, Copy, Eq, PartialEq, derive_more::From)]
pub enum UnaryOp {
    /// Logical negation `not`.
    Not,
}
