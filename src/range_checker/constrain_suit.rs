use super::{ast::SuitConst, Card, Idx, Suit4};

#[derive(PartialEq, Eq, Debug, Default, Clone)]
pub(super) enum ConstrainSuit {
    Match(Suit4),
    Var(Vec<Idx>, Vec<Idx>, Suit4),
    #[default]
    Nil,
}

impl ConstrainSuit {
    #[inline]
    pub fn reject(&self, cs: &[Card], perm: &[Idx], i: usize) -> bool {
        match self {
            Self::Match(s4) => {
                if !s4.contains_suit(cs[i].s) {
                    return true;
                }
            }

            Self::Var(eq, neq, banned) => {
                if banned.contains_suit(cs[i].s) {
                    return true;
                }

                for original_idx in eq {
                    if let Some(j) = perm.iter().position(|k| original_idx == k)
                    {
                        if cs[i].s != cs[j].s {
                            return true;
                        }
                    }
                }

                for original_idx in neq {
                    if let Some(j) = perm.iter().position(|k| original_idx == k)
                    {
                        if cs[i].s == cs[j].s {
                            return true;
                        }
                    }
                }
            }

            Self::Nil => (),
        }

        false
    }
}

impl From<Option<SuitConst>> for ConstrainSuit {
    fn from(op: Option<SuitConst>) -> Self {
        op.map_or(Self::Nil, Self::from)
    }
}

impl From<(Vec<Idx>, Vec<Idx>, u8)> for ConstrainSuit {
    fn from((eq, neq, v): (Vec<Idx>, Vec<Idx>, u8)) -> Self {
        Self::Var(eq, neq, Suit4::from_u8(v))
    }
}

impl From<SuitConst> for ConstrainSuit {
    fn from(s: SuitConst) -> Self {
        Self::Match(Suit4::from_u8(1 << s as u8))
    }
}
