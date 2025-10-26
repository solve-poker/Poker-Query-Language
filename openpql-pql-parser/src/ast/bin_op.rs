#[derive(Debug, Clone, Copy, Eq, PartialEq, derive_more::From)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Ge,
    Gt,
    Le,
    Lt,
}
