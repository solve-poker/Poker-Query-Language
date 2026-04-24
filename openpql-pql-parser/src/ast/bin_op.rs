/// Binary operator in an expression.
#[derive(Debug, Clone, Copy, Eq, PartialEq, derive_more::From)]
pub enum BinOp {
    /// Addition `+`.
    Add,
    /// Subtraction `-`.
    Sub,
    /// Multiplication `*`.
    Mul,
    /// Division `/`.
    Div,
    /// Equality `=`.
    Eq,
    /// Greater-or-equal `>=`.
    Ge,
    /// Greater-than `>`.
    Gt,
    /// Less-or-equal `<=`.
    Le,
    /// Less-than `<`.
    Lt,
    /// Logical conjunction `and`.
    And,
    /// Logical disjunction `or`.
    Or,
}
