use super::{Expr, Ident, Loc, String};

#[derive(Clone, PartialEq, derive_more::From, derive_more::Debug)]
#[debug("{:?}({})", self.name, _to_str(&self.args))]
pub struct FnCall<'i> {
    pub name: Ident<'i>,
    pub args: Vec<Expr<'i>>,

    pub loc: (Loc, Loc),
}

fn _to_str(elems: &[Expr<'_>]) -> String {
    elems
        .iter()
        .map(|e| format!("{e:?}"))
        .collect::<Vec<_>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_fn_call() {
        let src = "equity(hero, river)";
        let fncall = parse_fn_call(src).unwrap();

        let ident = |id| Ident::from((id, loc(src, id)));

        assert_eq!(fncall.name, ident("equity"));
        assert_eq!(fncall.args.len(), 2);
        assert_eq!(fncall.args[0], ident("hero").into());
        assert_eq!(fncall.args[1], ident("river").into());
        assert_eq!(fncall.loc, (0, src.len()));
    }

    #[test]
    fn test_debug() {
        let fncall = parse_fn_call("equity(hero, 1.23)").unwrap();

        assert_eq!(format!("{fncall:?}"), "equity(hero,1.23)");
    }
}
