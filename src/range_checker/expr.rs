use super::{ast, Card, Deps, Error, Leaf};

#[derive(PartialEq, Eq, Debug, Clone)]
pub(super) enum Expr<const N: usize, const B: bool> {
    Not(Box<Expr<N, B>>, Box<Expr<N, B>>),
    And(Box<Expr<N, B>>, Box<Expr<N, B>>),
    Or(Box<Expr<N, B>>, Box<Expr<N, B>>),
    Leaf(Leaf<N, B>),
}

impl<const N: usize, const B: bool> Expr<N, B> {
    #[inline]
    pub fn is_satisfied(&self, cs: &[Card]) -> bool {
        match self {
            Self::Not(l, r) => l.is_satisfied(cs) && !r.is_satisfied(cs),
            Self::And(l, r) => l.is_satisfied(cs) && r.is_satisfied(cs),
            Self::Or(l, r) => l.is_satisfied(cs) || r.is_satisfied(cs),
            Self::Leaf(e) => e.is_satisfied(cs),
        }
    }
}

impl<const N: usize, const B: bool> TryFrom<ast::Expr> for Expr<N, B> {
    type Error = Error;

    fn try_from(expr: ast::Expr) -> Result<Self, Self::Error> {
        match expr {
            ast::Expr::Not(l, r) => Ok(Self::Not(
                Box::new((*l).try_into()?),
                Box::new((*r).try_into()?),
            )),
            ast::Expr::And(l, r) => Ok(Self::And(
                Box::new((*l).try_into()?),
                Box::new((*r).try_into()?),
            )),
            ast::Expr::Or(l, r) => Ok(Self::Or(
                Box::new((*l).try_into()?),
                Box::new((*r).try_into()?),
            )),
            ast::Expr::Term(t, loc) => {
                Ok(Self::Leaf((t, Deps(loc)).try_into()?))
            }
        }
    }
}
