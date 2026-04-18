#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InternalError {
    StackUnderflow,
    NonNumericStackValue,
    UnexpectedTypeCast,
    UnexpectedComparison,
}
