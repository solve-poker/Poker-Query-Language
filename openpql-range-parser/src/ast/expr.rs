use super::{LocInfo, Term};

#[derive(PartialEq, Eq, Debug)]
pub enum Expr {
    Not(Box<Self>, Box<Self>),
    And(Box<Self>, Box<Self>),
    Or(Box<Self>, Box<Self>),
    Term(Term, LocInfo),
}

impl From<(Term, LocInfo)> for Expr {
    fn from((t, loc): (Term, LocInfo)) -> Self {
        Self::Term(t, loc)
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::*;

    fn assert_expr(s: &str, expected: Expr) {
        assert_eq!(
            *parse_expr(false, s).unwrap(),
            expected,
            "{s} != {expected:?}"
        );
    }

    fn term_loc(s: &str, a: Loc, b: Loc) -> (Term, LocInfo) {
        (parse_term(s).unwrap(), (a, b))
    }

    fn binop<S, T>(l: S, r: T, f: fn(Box<Expr>, Box<Expr>) -> Expr) -> Expr
    where
        Expr: From<S> + From<T>,
    {
        f(Box::new(Expr::from(l)), Box::new(Expr::from(r)))
    }

    fn not<S, T>(l: S, r: T) -> Expr
    where
        Expr: From<S> + From<T>,
    {
        binop(l, r, Expr::Not)
    }

    fn and<S, T>(l: S, r: T) -> Expr
    where
        Expr: From<S> + From<T>,
    {
        binop(l, r, Expr::And)
    }

    fn or<S, T>(l: S, r: T) -> Expr
    where
        Expr: From<S> + From<T>,
    {
        binop(l, r, Expr::Or)
    }

    #[test]
    fn test_expr_term() {
        assert_expr("AsA", term_loc("AsA", 0, 3).into());
    }

    #[test]
    fn test_expr_and() {
        assert_expr("AsA:ss", and(term_loc("AsA", 0, 3), term_loc("ss", 4, 6)));
    }

    #[test]
    fn test_expr_or() {
        assert_expr("AsA,ss", or(term_loc("AsA", 0, 3), term_loc("ss", 4, 6)));
    }

    #[test]
    fn test_expr_not() {
        assert_expr("AsA!ss", not(term_loc("AsA", 0, 3), term_loc("ss", 4, 6)));
    }

    #[test]
    fn test_expr_precedence() {
        assert_expr(
            "A:B!c,d",
            or(
                and(
                    term_loc("A", 0, 1),
                    not(term_loc("B", 2, 3), term_loc("c", 4, 5)),
                ),
                term_loc("d", 6, 7),
            ),
        );

        assert_expr(
            "A:B!(c,d)",
            and(
                term_loc("A", 0, 1),
                not(
                    term_loc("B", 2, 3),
                    or(term_loc("c", 5, 6), term_loc("d", 7, 8)),
                ),
            ),
        );
    }
}
