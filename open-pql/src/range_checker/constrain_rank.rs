use super::{Card, Idx, Rank16, ast::RankConst};

#[derive(PartialEq, Eq, Debug, Default, Clone)]
pub(super) enum ConstrainRank {
    Match(Rank16),
    Diff(Idx, i8),
    Var(Vec<Idx>, Vec<Idx>, Rank16),
    #[default]
    Nil,
}

pub const fn rank_diff(lhs: Card, rhs: Card) -> i8 {
    (lhs.rank as i8) - (rhs.rank as i8)
}

impl ConstrainRank {
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

            Self::Var(eq, neq, banned) => {
                if banned.contains_rank(cs[i].rank) {
                    return true;
                }

                for original_idx in eq {
                    if let Some(j) = perm.iter().position(|k| original_idx == k)
                        && cs[i].rank != cs[j].rank
                    {
                        return true;
                    }
                }

                for original_idx in neq {
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

impl From<Rank16> for ConstrainRank {
    fn from(r: Rank16) -> Self {
        Self::Match(r)
    }
}

impl From<(Vec<Idx>, Vec<Idx>, u16)> for ConstrainRank {
    fn from((eq, neq, v): (Vec<Idx>, Vec<Idx>, u16)) -> Self {
        Self::Var(eq, neq, Rank16::from_u16(v))
    }
}

impl From<RankConst> for ConstrainRank {
    fn from(r: RankConst) -> Self {
        Self::Match(Rank16::from_u16(1 << r as u8))
    }
}
