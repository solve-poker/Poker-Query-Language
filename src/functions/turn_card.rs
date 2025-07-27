use super::*;

#[pqlfn]
pub fn turn_card(board: Board) -> PQLCard {
    let c: Turn = board.into();

    c.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[quickcheck]
    fn test_turn_card(board: Board) -> TestResult {
        let turn = board.turn.0;

        TestResult::from_bool(turn == turn_card(board))
    }
}
