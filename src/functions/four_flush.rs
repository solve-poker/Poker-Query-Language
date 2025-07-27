use three_flush::n_flush;

use super::*;

#[pqlfn]
pub fn four_flush(hand: &Hand, street: PQLStreet, board: Board) -> PQLBoolean {
    n_flush::<4>(hand, board, street)
}

#[cfg(test)]
mod tests {
    use three_flush::tests::has_n_flush;

    use super::*;

    #[quickcheck]
    fn test_four_flush(hbg: HandBoardGame) -> TestResult {
        TestResult::from_bool(
            has_n_flush(&hbg.hand, hbg.board, hbg.street, 4)
                == four_flush(hbg.hand.as_ref(), hbg.street, hbg.board),
        )
    }
}
