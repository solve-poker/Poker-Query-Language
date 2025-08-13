use derive_more::derive::From;

use super::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, From)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Ge,
    Gt,
    Le,
    Lt,
}

#[derive(Debug, Eq, PartialEq, From)]
pub enum Expr<'i> {
    Ident(Ident<'i>),
    Str(Str<'i>),
    FnCall(FnCall<'i>),
    Int(Num<'i>),
    BinOp(BinOp, Box<Expr<'i>>, Box<Expr<'i>>),
}

impl Expr<'_> {
    pub(crate) const fn loc(&self) -> LocInfo {
        match self {
            Expr::Ident(id) => id.loc,
            Expr::Str(s) => s.loc,
            Expr::FnCall(fncall) => fncall.loc,
            Expr::Int(int) => int.loc,
            Expr::BinOp(_, l, r) => (l.loc().0, r.loc().1),
        }
    }

    pub(crate) fn binop(op: BinOp, l: Self, r: Self) -> Self {
        Self::BinOp(op, Box::new(l), Box::new(r))
    }
}

#[cfg(test)]
mod tests {
    use super::{super::super::parser::*, *};

    fn e(s: &str) -> Expr<'_> {
        ExprParser::new().parse(s).unwrap()
    }

    #[test]
    fn test_cmp() {
        assert![matches!(e("1 = 1"), Expr::BinOp(BinOp::Eq, _, _))];
        assert![matches!(e("1 > 1"), Expr::BinOp(BinOp::Gt, _, _))];
        assert![matches!(e("1 >= 1"), Expr::BinOp(BinOp::Ge, _, _))];
        assert![matches!(e("1 < 1"), Expr::BinOp(BinOp::Lt, _, _))];
        assert![matches!(e("1 <= 1"), Expr::BinOp(BinOp::Le, _, _))];
    }

    #[test]
    fn test_expr() {
        assert![matches!(e("a"), Expr::Ident(_))];
        assert![matches!(e("'a'"), Expr::Str(_))];
        assert![matches!(e("sin(x)"), Expr::FnCall(_))];
    }

    #[test]
    fn test_loc() {
        assert_eq!(e("a").loc(), (0, 1));
        assert_eq!(e("'a'").loc(), (0, 3));
        assert_eq!(e("sin(x)").loc(), (0, 6));
        assert_eq!(e("10").loc(), (0, 2));
        assert_eq!(e("1 >= 3").loc(), (0, 6));
    }
}
