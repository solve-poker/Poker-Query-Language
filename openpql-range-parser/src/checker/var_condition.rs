use super::{Array, Idx, IdxVec, PhantomData, ops};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct VarCondition<M, E, const N: usize>
where
    [Idx; N]: Array<Item = Idx>,
    M: ops::BitOrAssign<E>,
{
    pub equal: IdxVec<N>,
    pub not_equal: IdxVec<N>,
    pub banned: M,
    pub _marker: PhantomData<E>,
}

impl<M, E, const N: usize> VarCondition<M, E, N>
where
    [Idx; N]: Array<Item = Idx>,
    M: ops::BitOrAssign<E>,
{
    #[allow(clippy::cast_possible_truncation)]
    #[inline]
    pub fn set_indices(&mut self, is_equal: bool, idx: usize) {
        if is_equal {
            self.equal.push(idx as Idx);
        } else {
            self.not_equal.push(idx as Idx);
        }
    }
}
