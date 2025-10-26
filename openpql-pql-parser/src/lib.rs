#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#![cfg_attr(test, allow(clippy::needless_pass_by_value))]
#![cfg_attr(test, allow(clippy::wildcard_imports))]

use std::{collections::hash_map::Entry, fmt, string::String};

use derive_more::Display;
use lalrpop_util::{ParseError, lalrpop_mod, lexer::Token};
use rustc_hash::{FxHashMap, FxHashSet};

pub mod ast;
mod error;

pub use error::Error;
use error::{LalrError, ResultE, user_err};
use parser::{
    ExprParser, FnCallParser, FromClauseParser, IdentParser, NumParser,
    PQLParser, SelectorParser, StrParser,
};

pub fn parse_pql(src: &str) -> Result<Vec<ast::Stmt<'_>>, Error> {
    PQLParser::new().parse(src).map_err(Into::into)
}

// Editor macro is much simpler than rust macro :>

pub fn parse_selector(src: &str) -> Result<ast::Selector<'_>, Error> {
    SelectorParser::new().parse(src).map_err(Into::into)
}

pub fn parse_from_clause(src: &str) -> Result<ast::FromClause<'_>, Error> {
    FromClauseParser::new().parse(src).map_err(Into::into)
}

pub fn parse_expr(src: &str) -> Result<ast::Expr<'_>, Error> {
    ExprParser::new().parse(src).map_err(Into::into)
}

pub fn parse_fn_call(src: &str) -> Result<ast::FnCall<'_>, Error> {
    FnCallParser::new().parse(src).map_err(Into::into)
}

pub fn parse_str(src: &str) -> Result<ast::Str<'_>, Error> {
    StrParser::new().parse(src).map_err(Into::into)
}

pub fn parse_num(src: &str) -> Result<ast::Num, Error> {
    NumParser::new().parse(src).map_err(Into::into)
}

pub fn parse_ident(src: &str) -> Result<ast::Ident<'_>, Error> {
    IdentParser::new().parse(src).map_err(Into::into)
}

type Expected = Vec<String>;

lalrpop_mod!(
    #[allow(clippy::empty_line_after_outer_attr)]
    #[allow(clippy::iter_nth_zero)]
    #[allow(clippy::nursery)]
    #[allow(clippy::pedantic)]
    #[allow(clippy::restriction)]
    #[allow(clippy::useless_conversion)]
    pub(crate) parser,
    "/pql.rs"
);

pub type Loc = usize;
pub type LocInfo = (Loc, Loc);

fn strip_str(s: &str) -> &str {
    &s[1..s.len() - 1]
}

#[cfg(test)]
pub use tests::*;

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
pub mod tests {
    use super::*;

    #[allow(clippy::missing_panics_doc)]
    pub fn loc(full: &str, sub: &str) -> (Loc, Loc) {
        let start = full
            .find(sub)
            .unwrap_or_else(|| panic!("{sub} not in {full}"));
        let end = start + sub.len();
        (start, end)
    }

    #[test]
    fn test_error_invalid_token() {
        assert_eq!(Error::InvalidToken((0, 1)), parse_pql("?").unwrap_err());
    }

    #[test]
    fn test_error_unrecognized_eof() {
        let res = parse_pql("select").unwrap_err();

        if let Error::UnrecognizedEof(loc, expected) = res {
            assert_eq!(loc, (6, 7));
            assert_eq!(expected.len(), 1);
        } else {
            panic!("Expected: UnrecognizedEof. Got: {res:?}")
        }
    }

    #[test]
    fn test_error_unrecognized_token() {
        let res = parse_pql("select ()").unwrap_err();

        if let Error::UnrecognizedToken(loc, expected) = res {
            assert_eq!(loc, (7, 8));
            assert_eq!(expected.len(), 1);
        } else {
            panic!("Expected: UnrecognizedToken. Got: {res:?}")
        }
    }
}
