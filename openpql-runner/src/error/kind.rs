use super::*;

#[derive(Debug, Clone, PartialEq, Eq, derive_more::From)]
pub enum PQLErrorKind {
    // Parsing
    SyntaxError(SyntaxError),
    ParseError(ParseError),
    RangeError(RangeError),

    // Name resolution
    UnrecognizedFunction,
    UnrecognizedIdentifier,

    // Type and arity checks
    TypeError {
        given: PQLType,
        expected: PQLType,
    },
    WrongNumberOfArguments {
        given: usize,
        expected: usize,
    },
    #[from(skip)]
    ArithmeticOperationUnsupported {
        lhs: PQLType,
        rhs: PQLType,
    },
    #[from(skip)]
    ComparisonOperationUnsupported {
        lhs: PQLType,
        rhs: PQLType,
    },
    #[from(skip)]
    LogicalOperationUnsupported {
        lhs: PQLType,
        rhs: PQLType,
    },

    // Input validation
    ExceededMaximumPlayers(usize),
    InvalidPlayer,
    InvalidDeadcards,
    InvalidCardCount,

    // Execution
    Internal(InternalError),
    Runtime(RuntimeError),
    Vm(VmError),
}
