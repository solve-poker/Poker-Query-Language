use super::{Card, Constrain, Error, LocInfo, ast, range_cond_indices};

#[derive(PartialEq, Eq, Debug, Clone)]
pub(super) struct Leaf<const N: usize, const B: bool> {
    constrains: [Constrain; N],
}

impl<const N: usize, const B: bool> Leaf<N, B> {
    pub const fn new(constrains: [Constrain; N]) -> Self {
        Self { constrains }
    }

    #[inline]
    pub fn is_satisfied(&self, cs: &[Card]) -> bool {
        let n = self.constrains.len();
        let r = cs.len();

        !range_cond_indices(n, r, B)
            .iter()
            .all(|perm| Constrain::reject(&self.constrains, cs, perm))
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Deps(pub LocInfo);

impl<const N: usize, const B: bool> TryFrom<(ast::Term, Deps)> for Leaf<N, B> {
    type Error = Error;

    fn try_from((term, deps): (ast::Term, Deps)) -> Result<Self, Self::Error> {
        let mut constrains = vec![];
        let mut i = 0;

        for el in &term.inner {
            match el {
                ast::TermElem::Card(c) => {
                    constrains.push(Constrain::from_card(&term, *c, i));
                    i += 1;
                }

                ast::TermElem::List(l) => {
                    constrains.push(Constrain::from_list(l));
                    i += 1;
                }

                ast::TermElem::Span(s) => {
                    let v = Constrain::from_span(s, i);

                    i += v.len().to_le_bytes()[0];

                    constrains.extend(v);
                }
            }
        }

        if i as usize > N {
            Err(Error::TooManyCardsInRange(deps.0))
        } else {
            for _ in i as usize..N {
                constrains.push(Constrain::default());
            }

            Ok(Self::new(constrains.try_into().unwrap()))
        }
    }
}
