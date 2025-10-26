use super::{
    Array, Card, From, Idx, Suit4, VarCondition, VarConditionSuit,
    ast::SuitConst,
};

#[derive(PartialEq, Eq, Debug, Default, Clone)]
pub enum ConstrainSuit<const N: usize>
where
    [Idx; N]: Array<Item = Idx>,
{
    Match(Suit4),
    Var(VarConditionSuit<N>),
    #[default]
    Nil,
}

impl<const N: usize> ConstrainSuit<N>
where
    [Idx; N]: Array<Item = Idx>,
{
    #[inline]
    pub fn reject(&self, cs: &[Card], perm: &[Idx], i: usize) -> bool {
        match self {
            Self::Match(s4) => {
                if !s4.contains_suit(cs[i].suit) {
                    return true;
                }
            }

            Self::Var(VarConditionSuit(VarCondition {
                equal,
                not_equal,
                banned,
                ..
            })) => {
                if banned.contains_suit(cs[i].suit) {
                    return true;
                }

                for original_idx in equal {
                    if let Some(j) = perm.iter().position(|k| original_idx == k)
                        && cs[i].suit != cs[j].suit
                    {
                        return true;
                    }
                }

                for original_idx in not_equal {
                    if let Some(j) = perm.iter().position(|k| original_idx == k)
                        && cs[i].suit == cs[j].suit
                    {
                        return true;
                    }
                }
            }

            Self::Nil => (),
        }

        false
    }
}

impl<const N: usize> From<Option<SuitConst>> for ConstrainSuit<N>
where
    [Idx; N]: Array<Item = Idx>,
{
    fn from(op: Option<SuitConst>) -> Self {
        op.map_or(Self::Nil, Self::from)
    }
}

impl<const N: usize> From<SuitConst> for ConstrainSuit<N>
where
    [Idx; N]: Array<Item = Idx>,
{
    fn from(s: SuitConst) -> Self {
        Self::Match(Suit4::from(1 << s as u8))
    }
}
