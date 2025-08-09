use super::*;

#[pqlfn]
pub fn three_flush(hand: &Hand, street: PQLStreet, board: Board) -> PQLBoolean {
    n_flush::<3>(hand, board, street)
}

pub fn n_flush<const N: u8>(
    hand: &Hand,
    board: Board,
    street: PQLStreet,
) -> PQLBoolean {
    let c64_hand: Card64 = hand.into();
    let c64_board: Card64 = (board, street).into();

    for s in Suit::ARR_ALL {
        let h = c64_hand.count_by_suit(s);
        if h > 0 {
            let b = c64_board.count_by_suit(s);

            if h.min(2) + b >= N {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::*;

    pub fn has_n_flush(
        hand: &Hand,
        board: Board,
        street: PQLStreet,
        count_all: usize,
    ) -> bool {
        let c64_hand: Card64 = hand.into();
        let c64_board: Card64 = (board, street).into();

        Suit::ARR_ALL.iter().any(|&s| {
            let h = c64_hand.count_by_suit(s);
            let b = c64_board.count_by_suit(s);

            h > 0 && (h.min(2) + b) as usize >= count_all
        })
    }

    #[quickcheck]
    fn test_three_flush(hbg: HandBoardGame) -> TestResult {
        TestResult::from_bool(
            has_n_flush(&hbg.hand, hbg.board, hbg.street, 3)
                == three_flush(hbg.hand.as_ref(), hbg.street, hbg.board),
        )
    }
}
