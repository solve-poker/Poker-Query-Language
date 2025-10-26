use super::{
    Array, Card, From, Idx, Rank16, RankDiff, VarCondition, VarConditionRank,
    ast::RankConst,
};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum ConstrainRank<const N: usize>
where
    [Idx; N]: Array<Item = Idx>,
{
    Match(Rank16),
    Diff(Idx, RankDiff),
    Var(VarConditionRank<N>),
    #[default]
    Nil,
}

pub const fn rank_diff(lhs: Card, rhs: Card) -> RankDiff {
    (lhs.rank as RankDiff) - (rhs.rank as RankDiff)
}

impl<const N: usize> ConstrainRank<N>
where
    [Idx; N]: Array<Item = Idx>,
{
    #[inline]
    pub fn reject(&self, cs: &[Card], perm: &[Idx], i: usize) -> bool {
        match self {
            Self::Match(r16) => {
                if !r16.contains_rank(cs[i].rank) {
                    return true;
                }
            }

            Self::Diff(original_idx, d) => {
                if let Some(j) = perm.iter().position(|k| original_idx == k)
                    && rank_diff(cs[j], cs[i]) != *d
                {
                    return true;
                }
            }

            Self::Var(VarConditionRank(VarCondition {
                equal,
                not_equal,
                banned,
                ..
            })) => {
                if banned.contains_rank(cs[i].rank) {
                    return true;
                }

                for original_idx in equal {
                    if let Some(j) = perm.iter().position(|k| original_idx == k)
                        && cs[i].rank != cs[j].rank
                    {
                        return true;
                    }
                }

                for original_idx in not_equal {
                    if let Some(j) = perm.iter().position(|k| original_idx == k)
                        && cs[i].rank == cs[j].rank
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

impl<const N: usize> From<Rank16> for ConstrainRank<N>
where
    [Idx; N]: Array<Item = Idx>,
{
    fn from(r: Rank16) -> Self {
        Self::Match(r)
    }
}

impl<const N: usize> From<RankConst> for ConstrainRank<N>
where
    [Idx; N]: Array<Item = Idx>,
{
    fn from(r: RankConst) -> Self {
        Self::Match(Rank16::from(1 << r as u8))
    }
}
