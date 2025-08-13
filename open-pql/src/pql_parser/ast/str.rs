use super::*;

#[derive(Debug, Clone, PartialEq, Eq, From)]
pub struct Str<'i> {
    pub inner: &'i str,
    pub loc: (Loc, Loc),
}

#[cfg(test)]
mod tests {
    use super::{super::super::parser::*, *};

    fn s(s: &str) -> Str<'_> {
        StrParser::new().parse(s).unwrap()
    }

    #[test]
    fn test_str() {
        assert_eq!(s(r#""str""#), ("str", (0, 5)).into());
        assert_eq!(s(r#""""#), ("", (0, 2)).into());

        assert_eq!(s("'str'"), ("str", (0, 5)).into());
        assert_eq!(s("''"), ("", (0, 2)).into());

        assert_eq!(s("'one two three'"), ("one two three", (0, 15)).into());
    }
}
