use super::*;

#[derive(Debug, Clone, PartialEq, Eq, From)]
pub struct Num<'i> {
    pub inner: &'i str,
    pub loc: (Loc, Loc),
    pub is_float: bool,
}

#[cfg(test)]
mod tests {
    use super::{super::super::parser::*, *};

    fn i(s: &str) -> Num<'_> {
        NumParser::new().parse(s).unwrap()
    }

    #[test]
    fn test_num() {
        assert_eq!(i("0"), ("0", (0, 1), false).into());
        assert_eq!(i("-1"), ("-1", (0, 2), false).into());

        assert_eq!(i("-1.5"), ("-1.5", (0, 4), true).into());
        assert_eq!(i("-.5"), ("-.5", (0, 3), true).into());
        assert_eq!(i(".5"), (".5", (0, 2), true).into());
    }
}
