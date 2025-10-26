use super::{
    Array, ConstrainSuit, From, Idx, RangeCard, Suit, Suit4, SuitVar, Term,
    TermElem, VarCondition,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(super) struct VarConditionSuit<const N: usize>(
    pub(crate) VarCondition<Suit4, Suit, N>,
)
where
    [Idx; N]: Array<Item = Idx>;

impl<const N: usize> VarConditionSuit<N>
where
    [Idx; N]: Array<Item = Idx>,
{
    #[inline]
    #[allow(clippy::enum_glob_use)]
    fn from_term(term: &Term, var: SuitVar, self_idx: Idx) -> Self {
        use RangeCard::*;

        let mut inner = VarCondition::<Suit4, Suit, N>::default();

        for (i, e) in term.0.iter().enumerate() {
            if i == self_idx as usize {
                continue;
            }

            match e {
                TermElem::Card(CC(_, s) | VC(_, s) | AC(s)) => {
                    inner.banned |= *s;
                }
                TermElem::Card(CV(_, other) | VV(_, other) | AV(other)) => {
                    inner.set_indices(*other == var, i);
                }
                _ => (),
            }
        }

        Self(inner)
    }
}

impl<const N: usize> From<(&Term, SuitVar, Idx)> for VarConditionSuit<N>
where
    [Idx; N]: Array<Item = Idx>,
{
    fn from((t, v, i): (&Term, SuitVar, Idx)) -> Self {
        Self::from_term(t, v, i)
    }
}

impl<const N: usize> From<(&Term, SuitVar, Idx)> for ConstrainSuit<N>
where
    [Idx; N]: Array<Item = Idx>,
{
    fn from((t, v, i): (&Term, SuitVar, Idx)) -> Self {
        Self::Var(VarConditionSuit::from_term(t, v, i))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    fn assert_varcond(
        (term, var, self_idx): (&str, SuitVar, Idx),
        expected: (&[Idx], &[Idx], Suit4),
    ) {
        assert!(self_idx < 4);

        let cond = VarConditionSuit::<4>::from((
            &parse_term(term).unwrap(),
            var,
            self_idx,
        ));

        assert_eq!(cond.0.equal.as_slice(), expected.0);
        assert_eq!(cond.0.not_equal.as_slice(), expected.1);
        assert_eq!(cond.0.banned, expected.2);
    }

    #[test]
    fn test_var_info_suit() {
        use SuitVar::*;

        let t = "x[c][AdKh-]ysx";
        assert_varcond((t, Y, 3), (&[], &[0, 5], s4!("s")));
        assert_varcond((t, X, 0), (&[5], &[3], s4!("s")));

        assert_varcond(("xAs", X, 0), (&[], &[], s4!("s")));
        assert_varcond(("xOs", X, 0), (&[], &[], s4!("s")));
        assert_varcond(("xs", X, 0), (&[], &[], s4!("s")));

        assert_varcond(("xAy", X, 0), (&[], &[1], Suit4::default()));
        assert_varcond(("xRy", X, 0), (&[], &[1], Suit4::default()));
        assert_varcond(("xy", X, 0), (&[], &[1], Suit4::default()));

        assert_varcond(("xAx", X, 0), (&[1], &[], Suit4::default()));
        assert_varcond(("xRx", X, 0), (&[1], &[], Suit4::default()));
        assert_varcond(("xx", X, 0), (&[1], &[], Suit4::default()));
    }
}
