use std::num::ParseIntError;

use derive_more::derive::{Display, From};

use crate::{Loc, LocInfo, PQLType, pql_parser, range_parser};

mod parse_error;

pub use parse_error::ParseError;

type SelectorName = String;
type OpSymbol = String;

type NumGiven = usize;
type NumExpected = NumGiven;

type GotType = PQLType;
type ExpectedType = GotType;

#[derive(Debug, Clone, PartialEq, Eq, From, Display)]
pub enum PQLError {
    TooManyVariables,
    #[display("UnknownIdent")]
    UnknownIdent(LocInfo),
    #[display("UnrecognizedGame")]
    UnrecognizedGame(LocInfo),
    #[display("UnrecognizedFunction")]
    UnrecognizedFunction(LocInfo),
    #[display("WrongNumberOfArguments. expected {_2} got {_1}")]
    WrongNumberOfArguments(LocInfo, NumGiven, NumExpected),
    #[display("ParseIntError {_1}")]
    ParseIntError(LocInfo, ParseIntError),
    #[from]
    #[display("TypeError {_1}")]
    TypeError(LocInfo, TypeError),
    #[from]
    #[display("RuntimeError {_0}")]
    RuntimeError(RuntimeError),
    #[from]
    #[display("InvalidPQL {_0:?}")]
    InvalidPQL(pql_parser::Error),
    #[from]
    #[display("InvalidRange")]
    InvalidRange(Loc, range_parser::Error),
    #[from]
    Internal(InternalError),
}

// Error for
// * calling evaluate::to_rank as API
// * Vm at runtime
#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum RuntimeError {
    ToRankParseFailed(String),
    ToCardParseFailed(String),
    RateHiHandExpectedFiveCards(String),
    RateHiHandParseFailed(String),
    AddFailed,
    CmpFailed,
    MinMaxNotDefined,
    DefaultNotDefined,
}

// Vm Compile-time Error
#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum TypeError {
    #[display("BinOpError {_1} {_0} {_2}")]
    BinOpError(OpSymbol, PQLType, PQLType),
    #[display("InvalidIdent")]
    InvalidIdent(PQLType),
    #[display("SelectorDoesNotSupport {_0} {_1}")]
    SelectorDoesNotSupport(SelectorName, GotType),
    #[display("expected {_0}")]
    TypeMismatch(ExpectedType),
}

// Error that should not happen
#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum InternalError {
    BrokenStack,
    CannotAddValue,
    ThreadJoinFailed,
    InvalidVmValue,
}

impl From<&PQLError> for Option<LocInfo> {
    fn from(e: &PQLError) -> Self {
        match e {
            PQLError::Internal(_)
            | PQLError::TooManyVariables
            | PQLError::RuntimeError(_) => None,

            PQLError::UnknownIdent(loc)
            | PQLError::ParseIntError(loc, _)
            | PQLError::UnrecognizedGame(loc)
            | PQLError::WrongNumberOfArguments(loc, _, _)
            | PQLError::UnrecognizedFunction(loc)
            | PQLError::TypeError(loc, _) => Some(*loc),

            PQLError::InvalidPQL(e) => Some(e.into()),

            PQLError::InvalidRange(offset, e) => {
                let (a, b): LocInfo = e.into();

                Some((a + offset + 1, b + offset + 1))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loc() {
        let opt: Option<LocInfo> =
            (&PQLError::Internal(InternalError::BrokenStack)).into();
        assert!(opt.is_none());

        let opt: Option<LocInfo> = (&PQLError::TooManyVariables).into();
        assert!(opt.is_none());
    }
}
