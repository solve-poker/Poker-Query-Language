use super::*;

/// Virtual Heap
/// stores non-copy values
#[derive(
    Clone,
    Debug,
    Default,
    derive_more::DerefMut,
    derive_more::Deref,
    derive_more::From,
)]
pub struct VmHeap {
    inner: Vec<VmHeapValue>,
}

impl VmHeap {
    pub fn get_ref<T: 'static>(&self, i: HeapIdx) -> &T {
        self[i].as_any().downcast_ref::<T>().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_get_ref() {
        let (mut player, board) = mk_ranges(PQLGame::default(), &["AA"], "KK");
        let range = player.pop().unwrap();

        let heap = VmHeap {
            inner: vec![
                "test".to_string().into(),
                range.clone().into(),
                board.clone().into(),
            ],
        };

        assert_eq!(heap.get_ref::<PQLString>(0), "test");
        assert!(heap.get_ref::<PQLRange>(1).src_eq(&range));
        assert!(heap.get_ref::<PQLBoardRange>(2).src_eq(&board));
    }
}
