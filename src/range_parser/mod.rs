use std::convert::From;

use ast::Expr;
pub use error::Error;
use error::{LalrError, ResultE};
use lalrpop_util::{lalrpop_mod, lexer::Token, ParseError};

use crate::{Loc, LocInfo};

pub mod ast;
mod error;

lalrpop_mod!(
    #[allow(clippy::empty_line_after_outer_attr)]
    #[allow(clippy::iter_nth_zero)]
    #[allow(clippy::nursery)]
    #[allow(clippy::pedantic)]
    #[allow(clippy::restriction)]
    #[allow(clippy::useless_conversion)]
    parser,
    "/range_parser/range.rs"
);

type Expected = Vec<String>;

pub fn parse(src: &str) -> Result<Box<Expr>, Error> {
    Ok(parser::ExprParser::new().parse(src)?)
}

#[cfg_attr(coverage_nightly, coverage(off))]
#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn parse_card(src: &str) -> ResultE<ast::RangeCard> {
        parser::RangeCardParser::new().parse(src)
    }

    pub fn parse_list(src: &str) -> ResultE<ast::List> {
        parser::ListParser::new().parse(src)
    }

    pub fn parse_span(src: &str) -> ResultE<ast::Span> {
        parser::SpanParser::new().parse(src)
    }

    pub fn parse_term(src: &str) -> ResultE<ast::Term> {
        parser::TermParser::new().parse(src)
    }

    fn assert_str_in(vec: &[String], val: &str) {
        assert!(vec.contains(&format!("\"{val}\"")));
    }

    #[test]
    fn test_error_invalid_token() {
        assert_eq!(Error::InvalidToken((0, 1)), parse("?").unwrap_err());
    }

    #[test]
    fn test_error_unrecognized_eof() {
        let res = parse("[").unwrap_err();

        if let Error::UnrecognizedEof(loc, expected) = res {
            assert_eq!(loc, (1, 2));
            assert_eq!(expected.len(), 3);

            assert_str_in(&expected, "Suit");
            assert_str_in(&expected, "Rank");
            assert_str_in(&expected, "RankSuit");
        } else {
            panic!("Expected: UnrecognizedEof. Got: {res:?}")
        }
    }

    #[test]
    fn test_error_unrecognized_token() {
        let res = parse("[,").unwrap_err();

        if let Error::UnrecognizedToken(loc, expected) = res {
            assert_eq!(loc, (1, 2));
            assert_eq!(expected.len(), 3);

            assert_str_in(&expected, "Suit");
            assert_str_in(&expected, "Rank");
            assert_str_in(&expected, "RankSuit");
        } else {
            panic!("Expected: UnrecognizedToken. Got: {res:?}")
        }
    }

    #[test]
    fn test_error() {
        let err = LalrError::ExtraToken {
            token: (0, Token(0, "a"), 1),
        };

        assert_eq!(Error::ExtraToken((0, 1)), err.into());

        let err = LalrError::User {
            error: Error::InvalidRank((0, 1)),
        };

        assert_eq!(Error::InvalidRank((0, 1)), err.into());
    }
}
