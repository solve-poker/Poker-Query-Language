use super::{Expected, Loc, LocInfo, ParseError, Token};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    InvalidToken(LocInfo),
    UnrecognizedEof(LocInfo, Expected),
    UnrecognizedToken(LocInfo, Expected),
    ExtraToken(LocInfo),

    TooManyCardsInRange(LocInfo),
    NumberOfRanksMismatchInSpan(LocInfo),
    RankDistanceMismatchInSpan(LocInfo),
    SuitMismatchInSpan(LocInfo),
    InvalidSpan(LocInfo),
    InvalidList(LocInfo),
    InvalidRank(LocInfo),
    InvalidSuit(LocInfo),
}

pub type LalrError<'input> = ParseError<Loc, Token<'input>, Error>;

pub type ResultE<'input, T> = Result<T, LalrError<'input>>;

impl<'input> From<LalrError<'input>> for Error {
    fn from(err: LalrError<'input>) -> Self {
        match err {
            ParseError::InvalidToken { location: l } => {
                Self::InvalidToken((l, l + 1))
            }

            ParseError::UnrecognizedEof {
                location: l,
                expected: v,
            } => Self::UnrecognizedEof((l, l + 1), v),

            ParseError::UnrecognizedToken {
                token: t,
                expected: v,
            } => Self::UnrecognizedToken((t.0, t.2), v),

            ParseError::ExtraToken { token: t } => Self::ExtraToken((t.0, t.2)),

            ParseError::User { error } => error,
        }
    }
}

impl From<&Error> for LocInfo {
    fn from(e: &Error) -> Self {
        match e {
            Error::InvalidToken(loc)
            | Error::UnrecognizedEof(loc, _)
            | Error::UnrecognizedToken(loc, _)
            | Error::ExtraToken(loc)
            | Error::TooManyCardsInRange(loc)
            | Error::NumberOfRanksMismatchInSpan(loc)
            | Error::RankDistanceMismatchInSpan(loc)
            | Error::SuitMismatchInSpan(loc)
            | Error::InvalidSpan(loc)
            | Error::InvalidList(loc)
            | Error::InvalidRank(loc)
            | Error::InvalidSuit(loc) => *loc,
        }
    }
}
