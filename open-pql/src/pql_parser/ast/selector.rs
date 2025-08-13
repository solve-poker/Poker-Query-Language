use super::*;

#[derive(PartialEq, Eq, Debug, Display)]
pub enum SelectorKind {
    Avg,
    Count,
    Max,
    Min,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Selector<'i> {
    pub kind: SelectorKind,
    pub expr: Expr<'i>,
    pub alias: Option<Ident<'i>>,
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
mod tests {
    use super::{super::super::parser::*, *};

    fn s(s: &str) -> Selector<'_> {
        SelectorParser::new().parse(s).unwrap()
    }

    fn e(s: &str) -> Error {
        SelectorParser::new().parse(s).unwrap_err().into()
    }

    #[test]
    fn test_selector() {
        let obj1 = s("avg(equity(hero, river))");
        let obj2 = s("avg(equity(hero, river)) as s1");

        assert_eq!(obj1.kind, obj2.kind);
        assert_eq!(obj1.expr, obj2.expr);

        assert_eq!(obj1.kind, SelectorKind::Avg);
        assert![matches!(obj1.expr, Expr::FnCall(_))];
        assert_eq!(obj1.alias, None);

        assert_eq!(obj2.alias, Some(("s1", (28, 30)).into()));
    }

    #[test]
    fn test_selector_err() {
        assert_eq!(e("invalid(_)"), Error::UnrecognizedSelector((0, 7)));
    }
}
