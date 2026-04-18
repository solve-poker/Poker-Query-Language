use super::{
    BinOp, FnCall, Ident, Loc, LocInfo, Num, Spanned, Str, UnaryOp, str,
};

#[derive(Clone, PartialEq, derive_more::From, derive_more::Debug)]
pub enum Expr<'i> {
    #[debug("{_0:?}")]
    Ident(Ident<'i>),
    #[debug("{_0:?}")]
    Str(Str<'i>),
    #[debug("{_0:?}")]
    FnCall(FnCall<'i>),
    #[debug("{_0:?}")]
    Num(Num),
    #[debug("{_1:?} {} {_2:?}", _to_op(*_0))]
    BinOp(BinOp, Box<Self>, Box<Self>),
    #[debug("{} {_2:?}", _to_unary_op(*_0))]
    UnaryOp(UnaryOp, Loc, Box<Self>),
}

#[inline]
const fn _to_op(op: BinOp) -> &'static str {
    match op {
        BinOp::Add => "+",
        BinOp::Sub => "-",
        BinOp::Mul => "*",
        BinOp::Div => "/",
        BinOp::Eq => "=",
        BinOp::Ge => "≥",
        BinOp::Gt => ">",
        BinOp::Le => "≤",
        BinOp::Lt => "<",
        BinOp::And => "and",
        BinOp::Or => "or",
    }
}

#[inline]
const fn _to_unary_op(op: UnaryOp) -> &'static str {
    match op {
        UnaryOp::Not => "not",
    }
}

impl Spanned for Expr<'_> {
    fn loc(&self) -> LocInfo {
        match self {
            Expr::Ident(id) => id.loc,
            Expr::Str(s) => s.loc,
            Expr::FnCall(fncall) => fncall.loc,
            Expr::Num(int) => int.loc,
            Expr::BinOp(_, l, r) => (l.loc().0, r.loc().1),
            Expr::UnaryOp(_, start, e) => (*start, e.loc().1),
        }
    }
}

impl Expr<'_> {
    pub(crate) fn binop(op: BinOp, l: Self, r: Self) -> Self {
        Self::BinOp(op, Box::new(l), Box::new(r))
    }

    pub(crate) fn unary_op(op: UnaryOp, start: Loc, e: Self) -> Self {
        Self::UnaryOp(op, start, Box::new(e))
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::*;

    fn assert_expr(src: &str, expected: &str) {
        assert_eq!(format!("{:?}", parse_expr(src).unwrap()), expected);
    }

    #[test]
    fn test_binop() {
        assert_expr("1 + 1", "1 + 1");
        assert_expr("1 - 1", "1 - 1");
        assert_expr("1 * 1", "1 * 1");
        assert_expr("1 / 1", "1 / 1");

        assert_expr("1 = 1", "1 = 1");
        assert_expr("1 > 1", "1 > 1");
        assert_expr("1 >= 1", "1 ≥ 1");
        assert_expr("1 < 1", "1 < 1");
        assert_expr("1 <= 1", "1 ≤ 1");

        assert_expr("a and b", "a and b");
        assert_expr("a or b", "a or b");
        assert_expr("a AND b", "a and b");
        assert_expr("a OR b", "a or b");
        assert_expr("a or b and c", "a or b and c");
        assert_expr("a = 1 and b = 2", "a = 1 and b = 2");
    }

    #[test]
    fn test_unary_op() {
        assert_expr("not a", "not a");
        assert_expr("NOT a", "not a");
        assert_expr("not not a", "not not a");
        assert_expr("not a and b", "not a and b");
        assert_expr("a and not b", "a and not b");
        assert_expr("not a = 1", "not a = 1");
    }

    #[test]
    fn test_expr() {
        assert_expr("id", "id");
        assert_expr("'str'", "\"str\"");
        assert_expr("sin(x)", "sin(x)");
    }

    fn assert_loc(src: &str, start: Loc, end: Loc) {
        assert_eq!(parse_expr(src).unwrap().loc(), (start, end));
    }

    #[test]
    fn test_loc() {
        assert_loc("a", 0, 1);
        assert_loc("'a'", 0, 3);
        assert_loc("sin(x)", 0, 6);
        assert_loc("10", 0, 2);
        assert_loc("1 >= 3", 0, 6);
        assert_loc("not a", 0, 5);
    }

    #[test]
    fn test_debug() {
        fn assert_dbg(s: &str, expected: &str) {
            assert_eq!(format!("{:?}", parse_expr(s).unwrap()), expected);
        }

        assert_dbg("ident", "ident");

        assert_dbg("'string'", "\"string\"");

        assert_dbg("fncall(v1, v2)", "fncall(v1,v2)");

        assert_dbg("100", "100");
        assert_dbg("3.14", "3.14");

        assert_dbg("1 = 1", "1 = 1");
        assert_dbg("1 >= 1", "1 ≥ 1");
    }
}
