use super::*;

#[pqlfn(arg, rtn, eval)]
pub fn river_card(board: Board) -> PQLCard {
    let c: River = board.into();

    c.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[quickcheck]
    fn test_river_card(board: Board) -> TestResult {
        let river = board.river.0;

        TestResult::from_bool(river == river_card(board))
    }
}
