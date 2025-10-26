use super::{Array, Card, Deps, Error, Idx, Leaf, ast};

#[derive(PartialEq, Eq, Debug, Clone)]
pub(super) enum Expr<const N: usize, const B: bool>
where
    [Idx; N]: Array<Item = Idx>,
{
    Not(Box<Self>, Box<Self>),
    And(Box<Self>, Box<Self>),
    Or(Box<Self>, Box<Self>),
    Leaf(Leaf<N, B>),
}

impl<const N: usize, const B: bool> Expr<N, B>
where
    [Idx; N]: Array<Item = Idx>,
{
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

impl<const N: usize, const B: bool> TryFrom<ast::Expr> for Expr<N, B>
where
    [Idx; N]: Array<Item = Idx>,
{
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
