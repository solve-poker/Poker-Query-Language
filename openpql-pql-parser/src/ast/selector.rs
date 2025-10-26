use super::{Error, Expr, Ident, ResultE, SelectorKind, String};

#[derive(Clone, PartialEq, derive_more::Debug)]
#[debug("{:?}({:?}){}", self.kind, self.expr, _alias_to_str(self.alias.as_ref()))]
pub struct Selector<'i> {
    pub kind: SelectorKind,
    pub expr: Expr<'i>,
    pub alias: Option<Ident<'i>>,
}

fn _alias_to_str(alias: Option<&Ident>) -> String {
    alias.map_or_else(String::default, |id| format!(" as {id:?}"))
}

impl<'i> Selector<'i> {
    pub fn new(
        kind: &Ident<'i>,
        expr: Expr<'i>,
        alias: Option<Ident<'i>>,
    ) -> ResultE<'i, Self> {
        let kind = match kind.inner.to_ascii_lowercase().as_str() {
            "avg" => SelectorKind::Avg,
            "count" => SelectorKind::Count,
            "max" => SelectorKind::Max,
            "min" => SelectorKind::Min,
            _ => return Err(Error::UnrecognizedSelector(kind.loc).into()),
        };

        Ok(Self { kind, expr, alias })
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use crate::*;

    fn assert_selector(src: &str, expected: &str) {
        assert_eq!(format!("{:?}", parse_selector(src).unwrap()), expected);
    }

    #[test]
    fn test_selector() {
        assert_selector("avg(equity(hero, river))", "avg(equity(hero,river))");
        assert_selector(
            "avg(equity(hero, river)) as s1",
            "avg(equity(hero,river)) as s1",
        );

        assert_selector("count(_)", "count(_)");
        assert_selector("max(_)", "max(_)");
        assert_selector("min(_)", "min(_)");
    }

    fn assert_err(src: &str, expected: Error) {
        assert_eq!(parse_selector(src).unwrap_err(), expected);
    }

    #[test]
    fn test_selector_err() {
        let src = "invalid(_)";
        assert_err(src, Error::UnrecognizedSelector(loc(src, "invalid")));
    }
}
