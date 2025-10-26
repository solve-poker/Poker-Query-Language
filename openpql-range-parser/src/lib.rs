#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#![cfg_attr(test, allow(clippy::needless_pass_by_value))]
#![cfg_attr(test, allow(clippy::wildcard_imports))]

use std::{
    convert::From, marker::PhantomData, ops, string::ToString, sync::LazyLock,
};

use derive_more::Display;
use itertools::Itertools;
use lalrpop_util::{ParseError, lalrpop_mod, lexer::Token};
use openpql_prelude::*;
use smallvec::{Array, SmallVec};

pub mod ast;
mod checker;
mod error;

pub use checker::{BoardRangeChecker, RangeChecker};
pub use error::Error;
use error::{LalrError, ResultE};

lalrpop_mod!(
    #[allow(clippy::empty_line_after_outer_attr)]
    #[allow(clippy::iter_nth_zero)]
    #[allow(clippy::nursery)]
    #[allow(clippy::pedantic)]
    #[allow(clippy::restriction)]
    #[allow(clippy::useless_conversion)]
    parser,
    "/range.rs"
);

type Idx = u8;
pub type Loc = usize;
pub type LocInfo = (Loc, Loc);
type Expected = Vec<String>;

pub fn parse_expr(
    is_shortdeck: bool,
    src: &str,
) -> Result<Box<ast::Expr>, Error> {
    Ok(parser::ExprParser::new().parse(is_shortdeck, src)?)
}

#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

#[cfg(test)]
pub use tests::*;

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
pub mod tests {
    pub use std::{cmp, fmt};

    pub use openpql_prelude::s4;
    pub use quickcheck::{Arbitrary, TestResult};

    pub use super::*;

    pub fn parse_card(src: &str) -> ResultE<'_, ast::RangeCard> {
        parser::RangeCardParser::new().parse(false, src)
    }

    pub fn parse_list(src: &str) -> ResultE<'_, ast::List> {
        parser::ListParser::new().parse(false, src)
    }

    pub fn parse_span(src: &str) -> ResultE<'_, ast::Span> {
        parser::SpanParser::new().parse(false, src)
    }

    pub fn parse_term(src: &str) -> ResultE<'_, ast::Term> {
        parser::TermParser::new().parse(false, src)
    }

    pub fn parse_card_sd(src: &str) -> ResultE<'_, ast::RangeCard> {
        parser::RangeCardParser::new().parse(true, src)
    }

    pub(crate) fn assert_range_card(src: &str, expected: &str) {
        let range_card = parse_card(src).unwrap();

        assert_eq!(range_card.to_string(), expected);
    }

    pub(crate) fn assert_err<T: fmt::Debug + cmp::PartialEq>(
        res: ResultE<'_, T>,
        expected: Error,
    ) {
        assert_eq!(res, Err(expected.into()));
    }

    fn assert_str_in(vec: &[String], val: &str) {
        assert!(vec.contains(&format!("\"{val}\"")), "{val} not in {vec:?}");
    }

    #[test]
    fn test_error_invalid_token() {
        assert_eq!(
            Error::InvalidToken((0, 1)),
            parse_expr(false, "?").unwrap_err()
        );
    }

    #[test]
    fn test_error_unrecognized_eof() {
        let res = parse_expr(false, "[").unwrap_err();

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
        let res = parse_expr(false, "[,").unwrap_err();

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

    #[test]
    fn test_from_error_to_loc() {
        let err = Error::InvalidToken((5, 10));
        assert_eq!(LocInfo::from(&err), (5, 10));

        let err = Error::UnrecognizedEof((3, 7), vec![]);
        assert_eq!(LocInfo::from(&err), (3, 7));

        let err = Error::UnrecognizedToken((1, 4), vec![]);
        assert_eq!(LocInfo::from(&err), (1, 4));

        let err = Error::ExtraToken((2, 6));
        assert_eq!(LocInfo::from(&err), (2, 6));

        let err = Error::TooManyCardsInRange((10, 15));
        assert_eq!(LocInfo::from(&err), (10, 15));

        let err = Error::NumberOfRanksMismatchInSpan((8, 12));
        assert_eq!(LocInfo::from(&err), (8, 12));

        let err = Error::RankDistanceMismatchInSpan((4, 9));
        assert_eq!(LocInfo::from(&err), (4, 9));

        let err = Error::SuitMismatchInSpan((6, 11));
        assert_eq!(LocInfo::from(&err), (6, 11));

        let err = Error::InvalidSpan((7, 13));
        assert_eq!(LocInfo::from(&err), (7, 13));

        let err = Error::InvalidList((9, 14));
        assert_eq!(LocInfo::from(&err), (9, 14));

        let err = Error::InvalidRank((0, 1));
        assert_eq!(LocInfo::from(&err), (0, 1));

        let err = Error::InvalidSuit((11, 16));
        assert_eq!(LocInfo::from(&err), (11, 16));
    }
}
