use super::*;

#[derive(Debug, Eq, PartialEq, From)]
pub struct FnCall<'i> {
    pub name: Ident<'i>,
    pub args: Vec<Expr<'i>>,

    pub loc: (Loc, Loc),
}

#[cfg(test)]
mod tests {
    use super::{super::super::parser::*, *};

    fn f(s: &str) -> FnCall<'_> {
        FnCallParser::new().parse(s).unwrap()
    }

    #[test]
    fn test_expr() {
        let obj = f("equity(hero, river)");

        assert_eq!(obj.name, ("equity", (0, 6)).into());
        assert_eq!(obj.args.len(), 2);
        assert_eq!(obj.args[0], Expr::Ident(("hero", (7, 11)).into()));
        assert_eq!(obj.args[1], Expr::Ident(("river", (13, 18)).into()));
        assert_eq!(obj.loc, (0, 19));
    }
}
