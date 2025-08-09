use super::{LocInfo, Term};

#[derive(PartialEq, Eq, Debug)]
pub enum Expr {
    Not(Box<Expr>, Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Term(Term, LocInfo),
}

impl From<(Term, LocInfo)> for Expr {
    fn from((t, loc): (Term, LocInfo)) -> Self {
        Self::Term(t, loc)
    }
}

#[cfg(test)]
mod tests {

    use super::{
        super::{
            super::{parse, tests::parse_term},
            Term,
        },
        Expr,
    };

    fn p(s: &str) -> Expr {
        *parse(s).unwrap()
    }

    fn t(s: &str) -> Term {
        parse_term(s).unwrap()
    }

    #[test]
    fn test_expr_term() {
        assert_eq!(p("AsA"), (t("AsA"), (0, 3)).into());
    }

    #[test]
    fn test_expr_and() {
        assert_eq!(
            p("AsA:ss"),
            Expr::And(
                Box::new((t("AsA"), (0, 3)).into()),
                Box::new((t("ss"), (4, 6)).into())
            )
        );
    }

    #[test]
    fn test_expr_or() {
        assert_eq!(
            p("AsA,ss"),
            Expr::Or(
                Box::new((t("AsA"), (0, 3)).into()),
                Box::new((t("ss"), (4, 6)).into())
            )
        );
    }

    #[test]
    fn test_expr_not() {
        assert_eq!(
            p("AsA!ss"),
            Expr::Not(
                Box::new((t("AsA"), (0, 3)).into()),
                Box::new((t("ss"), (4, 6)).into())
            )
        );
    }

    #[test]
    fn test_expr_precedence() {
        assert_eq!(
            p("A:B!c,d"),
            Expr::Or(
                Box::new(Expr::And(
                    Box::new((t("A"), (0, 1)).into()),
                    Box::new(Expr::Not(
                        Box::new((t("B"), (2, 3)).into()),
                        Box::new((t("c"), (4, 5)).into()),
                    )),
                )),
                Box::new((t("d"), (6, 7)).into()),
            ),
        );

        assert_eq!(
            p("A:B!(c,d)"),
            Expr::And(
                Box::new((t("A"), (0, 1)).into()),
                Box::new(Expr::Not(
                    Box::new((t("B"), (2, 3)).into()),
                    Box::new(Expr::Or(
                        Box::new((t("c"), (5, 6)).into()),
                        Box::new((t("d"), (7, 8)).into()),
                    )),
                )),
            ),
        );
    }
}
