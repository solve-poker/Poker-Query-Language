use super::{Loc, LocInfo, Spanned, str};

/// String literal borrowed from the source with its surrounding quotes stripped.
#[derive(Clone, PartialEq, Eq, derive_more::From, derive_more::Debug)]
#[debug("{:?}", self.inner)]
pub struct Str<'i> {
    /// Literal content without the surrounding quotes.
    pub inner: &'i str,
    /// Source span including the quotes.
    pub loc: (Loc, Loc),
}

impl Spanned for Str<'_> {
    fn loc(&self) -> LocInfo {
        self.loc
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    fn assert_str(src: &str, expected: &str) {
        let loc_start = 0;
        let loc_end = src.len();
        assert_eq!(parse_str(src), Ok((expected, (loc_start, loc_end)).into()));
    }

    #[test]
    fn test_str() {
        assert_str(r#""str""#, "str");
        assert_str(r#""""#, "");

        assert_str("'str'", "str");
        assert_str("''", "");

        assert_str("'one two three'", "one two three");
    }

    #[test]
    fn test_dbg() {
        assert_eq!(
            format!("{:?}", Str::from(("content", (0, 1)))),
            "\"content\""
        );
    }

    #[test]
    fn test_loc() {
        let s = Str::from(("x", (1, 4)));
        assert_eq!(s.loc(), (1, 4));
    }
}
