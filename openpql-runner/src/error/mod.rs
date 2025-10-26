use super::*;

pub type Loc = usize;
pub type LocInfo = (Loc, Loc);
pub type PQLResult<T> = Result<T, PQLError>;

type Given = PQLType;
type Expected = Given;
type GivenN = usize;
type ExpectedN = GivenN;
type Lhs = PQLType;
type Rhs = PQLType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PQLError {
    pub loc: LocInfo,
    pub kind: PQLErrorKind,
}

impl<E> From<(SourceLocation, E)> for PQLError
where
    PQLErrorKind: From<E>,
{
    fn from((loc, kind): (SourceLocation, E)) -> Self {
        Self {
            loc,
            kind: PQLErrorKind::from(kind),
        }
    }
}

type NumPlayers = usize;

#[derive(Debug, Clone, PartialEq, Eq, derive_more::From)]
pub enum PQLErrorKind {
    SyntaxError(SyntaxError),
    ParseError(ParseError),
    RangeError(RangeError),
    Internal(InternalError),
    Runtime(RuntimeError),
    UnrecognizedFunction,
    UnrecognizedIdentifier,
    ExceededMaximumPlayers(NumPlayers),
    InvalidPlayer,
    InvalidDeadcards,
    InvalidCardCount,
    TypeError(Given, Expected),
    WrongNumberOfArguments(GivenN, ExpectedN),
    SamplingFailed,
    #[from(skip)]
    ArithmeticOperationUnsupported(Lhs, Rhs),
    #[from(skip)]
    ComparisonOperationUnsupported(Lhs, Rhs),
    VmErr(VmError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InternalError {
    StackUnderflow,
    NonNumericStackValue,
    UnexpectedTypeCast,
    UnexpectedComparison,
}

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

// used for flow control
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VmError {
    SamplingFailed,
}
