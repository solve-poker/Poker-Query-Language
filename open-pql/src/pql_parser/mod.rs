use std::{collections::hash_map::Entry, string::String};

use derive_more::derive::{Display, From};
use lalrpop_util::{ParseError, lalrpop_mod, lexer::Token};
use rustc_hash::{FxHashMap, FxHashSet};

use crate::{Loc, LocInfo};

pub mod ast;
mod error;

pub use error::Error;
use error::{ResultE, user_err};

type Expected = Vec<String>;

lalrpop_mod!(
    #[allow(clippy::empty_line_after_outer_attr)]
    #[allow(clippy::iter_nth_zero)]
    #[allow(clippy::nursery)]
    #[allow(clippy::pedantic)]
    #[allow(clippy::restriction)]
    #[allow(clippy::useless_conversion)]
    pub(crate) parser,
    "/pql_parser/pql.rs"
);

pub fn parse(src: &str) -> Result<Vec<ast::Stmt<'_>>, Error> {
    Ok(parser::PQLParser::new().parse(src)?)
}

pub(super) fn strip_str(s: &str) -> &str {
    &s[1..s.len() - 1]
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_error_invalid_token() {
        assert_eq!(Error::InvalidToken((0, 1)), parse("?").unwrap_err());
    }

    #[test]
    fn test_error_unrecognized_eof() {
        let res = parse("select").unwrap_err();

        if let Error::UnrecognizedEof(loc, expected) = res {
            assert_eq!(loc, (6, 7));
            assert_eq!(expected.len(), 1);
        } else {
            panic!("Expected: UnrecognizedEof. Got: {res:?}")
        }
    }

    #[test]
    fn test_error_unrecognized_token() {
        let res = parse("select ()").unwrap_err();

        if let Error::UnrecognizedToken(loc, expected) = res {
            assert_eq!(loc, (7, 8));
            assert_eq!(expected.len(), 1);
        } else {
            panic!("Expected: UnrecognizedToken. Got: {res:?}")
        }
    }
}
