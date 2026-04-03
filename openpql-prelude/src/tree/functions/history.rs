use crate::tree::Action;

pub fn is_descendent(left: &[Action], right: &[Action]) -> bool {
    match (left, right) {
        ([head_left, tail_left @ ..], [head_right, tail_right @ ..]) => {
            if head_left == head_right {
                is_descendent(tail_left, tail_right)
            } else {
                false
            }
        }
        ([], _) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::history;

    #[test]
    fn test_is_descendent() {
        assert!(is_descendent(
            &history!(c, 1, 2, c, c),
            &history!(c, 1, 2, c, c, c)
        ));
        assert!(is_descendent(
            &history!(c, 1, 2, c, c, c),
            &history!(c, 1, 2, c, c, c)
        ));
        assert!(!is_descendent(
            &history!(c, 1, 2, c, c, c),
            &history!(c, 1, 20, c, c, c)
        ));
        assert!(!is_descendent(
            &history!(c, 1, 2, c, c, c),
            &history!(c, 1, 2, c, c)
        ));
    }
}
