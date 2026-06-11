use super::{Expected, Loc, LocInfo, ParseError, Token};

/// Parse failure produced while parsing a range expression.
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

    /// Range lists more cards than allowed.
    TooManyCardsInRange(LocInfo),
    /// Span endpoints disagree on rank count.
    NumberOfRanksMismatchInSpan(LocInfo),
    /// Span endpoints disagree on rank distance.
    RankDistanceMismatchInSpan(LocInfo),
    /// Span endpoints disagree on suit.
    SuitMismatchInSpan(LocInfo),
    /// Span is otherwise invalid.
    InvalidSpan(LocInfo),
    /// List is otherwise invalid.
    InvalidList(LocInfo),
    /// Rank is out of range for the deck.
    InvalidRank(LocInfo),
    /// Suit is invalid.
    InvalidSuit(LocInfo),
}

/// LALRPOP parse error specialized for this grammar.
pub type LalrError<'input> = ParseError<Loc, Token<'input>, Error>;

/// Result alias for parser entry points.
pub type ResultE<'input, T> = Result<T, LalrError<'input>>;

impl<'input> From<LalrError<'input>> for Error {
    fn from(err: LalrError<'input>) -> Self {
        match err {
            ParseError::InvalidToken { location: l } => Self::InvalidToken((l, l + 1)),

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
