use itertools::Itertools;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref PERM11: Vec<Vec<u8>> = (0..1).permutations(1).collect_vec();
    pub static ref PERM21: Vec<Vec<u8>> = (0..2).permutations(1).collect_vec();
    pub static ref PERM22: Vec<Vec<u8>> = (0..2).permutations(2).collect_vec();
    pub static ref PERM31: Vec<Vec<u8>> = (0..3).permutations(1).collect_vec();
    pub static ref PERM32: Vec<Vec<u8>> = (0..3).permutations(2).collect_vec();
    pub static ref PERM33: Vec<Vec<u8>> = (0..3).permutations(3).collect_vec();
    pub static ref PERM41: Vec<Vec<u8>> = (0..4).permutations(1).collect_vec();
    pub static ref PERM42: Vec<Vec<u8>> = (0..4).permutations(2).collect_vec();
    pub static ref PERM43: Vec<Vec<u8>> = (0..4).permutations(3).collect_vec();
    pub static ref PERM44: Vec<Vec<u8>> = (0..4).permutations(4).collect_vec();
    pub static ref PERM_BOARD_4: Vec<Vec<u8>> = vec![
        vec![0, 1, 2, 3],
        vec![0, 2, 1, 3],
        vec![1, 0, 2, 3],
        vec![1, 2, 0, 3],
        vec![2, 0, 1, 3],
        vec![2, 1, 0, 3],
    ];
    pub static ref PERM_BOARD_5: Vec<Vec<u8>> = vec![
        vec![0, 1, 2, 3, 4],
        vec![0, 2, 1, 3, 4],
        vec![1, 0, 2, 3, 4],
        vec![1, 2, 0, 3, 4],
        vec![2, 0, 1, 3, 4],
        vec![2, 1, 0, 3, 4],
    ];
}

#[inline]
pub fn range_cond_indices(
    n: usize,
    r: usize,
    board: bool,
) -> &'static Vec<Vec<u8>> {
    match (n, r, board) {
        (1, 1, false) => PERM11.as_ref(),

        (2, 1, false) => PERM21.as_ref(),
        (2, 2, false) => PERM22.as_ref(),

        (3, 1, false) | (5, 1, true) => PERM31.as_ref(),
        (3, 2, false) | (5, 2, true) => PERM32.as_ref(),
        (3, 3, false) | (5, 3, true) => PERM33.as_ref(),

        (4, 1, false) => PERM41.as_ref(),
        (4, 2, false) => PERM42.as_ref(),
        (4, 3, false) => PERM43.as_ref(),
        (4, 4, false) => PERM44.as_ref(),

        (5, 4, true) => PERM_BOARD_4.as_ref(),
        (5, 5, true) => PERM_BOARD_5.as_ref(),

        _ => unimplemented!("n={n}; r={r}; board: {board}"),
    }
}

#[cfg(test)]
mod tests {
    use self::range_cond_indices as indices;
    use super::*;

    #[test]
    fn test_indices() {
        assert_eq!(*indices(1, 1, false), [[0]]);
        assert_eq!(*indices(2, 1, false), [[0], [1]]);
        assert_eq!(*indices(2, 2, false), [[0, 1], [1, 0]]);
        assert_eq!(*indices(3, 1, false), [[0], [1], [2]]);
        assert_eq!(indices(3, 2, false).len(), 6);
        assert_eq!(indices(3, 3, false).len(), 6);
        assert_eq!(*indices(4, 1, false), [[0], [1], [2], [3]]);
        assert_eq!(indices(4, 2, false).len(), 12);
        assert_eq!(indices(4, 3, false).len(), 24);
        assert_eq!(indices(4, 4, false).len(), 24);
    }

    #[test]
    fn test_indices_board() {
        assert_eq!(*indices(5, 1, true), *indices(3, 1, false));
        assert_eq!(*indices(5, 2, true), *indices(3, 2, false));
        assert_eq!(*indices(5, 3, true), *indices(3, 3, false));
        assert_eq!(
            *indices(5, 4, true),
            [
                [0, 1, 2, 3],
                [0, 2, 1, 3],
                [1, 0, 2, 3],
                [1, 2, 0, 3],
                [2, 0, 1, 3],
                [2, 1, 0, 3],
            ]
        );

        assert_eq!(
            *indices(5, 5, true),
            [
                [0, 1, 2, 3, 4],
                [0, 2, 1, 3, 4],
                [1, 0, 2, 3, 4],
                [1, 2, 0, 3, 4],
                [2, 0, 1, 3, 4],
                [2, 1, 0, 3, 4],
            ]
        );
    }

    #[test]
    #[should_panic(expected = "not implemented: n=5; r=1; board: false")]
    fn test_unimpl() {
        let _ = indices(5, 1, false);
    }
}
