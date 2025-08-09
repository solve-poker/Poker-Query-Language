use super::*;

#[pqlfn(arg, rtn, eval)]
pub fn board_in_range(range: &mut PQLBoardRange, board: Board) -> PQLBoolean {
    range.is_satisfied(&<[_; 5]>::from(board))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_board_in_range() {
        let mut c = PQLBoardRange::from_src("AKQJT").unwrap();

        assert!(!board_in_range(&mut c, board!("AsKhQcTsJh")));
    }
}
