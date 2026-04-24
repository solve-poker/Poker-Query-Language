use super::{Expected, Loc, LocInfo, ParseError, Token};

/// Parse failure produced while parsing a PQL source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Token is not recognized by the lexer.
    InvalidToken(LocInfo),
    /// Input ended before the parser expected.
    UnrecognizedEof(LocInfo, Expected),
    /// Token is not valid in this position.
    UnrecognizedToken(LocInfo, Expected),
    /// Trailing token after a complete parse.
    ExtraToken(LocInfo),

    /// Selector name is not one of the supported aggregates.
    UnrecognizedSelector(LocInfo),
    /// `from` clause contains the same key more than once.
    DuplicatedKeyInFrom(LocInfo),
    /// Two selectors share the same alias.
    DuplicatedSelectorName(LocInfo),
    /// Numeric literal is not a valid number.
    InvalidNumericValue(LocInfo),
}

/// LALRPOP parse error specialized for this grammar.
pub type LalrError<'input> = ParseError<Loc, Token<'input>, Error>;

/// Result alias for parser entry points.
pub type ResultE<'input, T> = Result<T, LalrError<'input>>;

pub const fn user_err<'input>(error: Error) -> LalrError<'input> {
    ParseError::User { error }
}

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
            | Error::UnrecognizedSelector(loc)
            | Error::DuplicatedKeyInFrom(loc)
            | Error::DuplicatedSelectorName(loc)
            | Error::InvalidNumericValue(loc) => *loc,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into_loc() {
        let loc = (1, 2);

        assert_eq!(loc, (&Error::InvalidToken(loc)).into());
        assert_eq!(loc, (&Error::UnrecognizedEof(loc, vec![])).into());
        assert_eq!(loc, (&Error::UnrecognizedToken(loc, vec![])).into());
        assert_eq!(loc, (&Error::ExtraToken(loc)).into());
        assert_eq!(loc, (&Error::UnrecognizedSelector(loc)).into());
        assert_eq!(loc, (&Error::DuplicatedKeyInFrom(loc)).into());
        assert_eq!(loc, (&Error::DuplicatedSelectorName(loc)).into());
        assert_eq!(loc, (&Error::InvalidNumericValue(loc)).into());
    }

    #[test]
    fn test_error() {
        let err = LalrError::ExtraToken {
            token: (0, Token(0, "a"), 1),
        };

        assert_eq!(Error::ExtraToken((0, 1)), err.into());
    }
}
