use super::*;

#[pqlfn(arg, rtn, eval)]
pub fn straight_board(
    street: PQLStreet,
    (game, board): (PQLGame, Board),
) -> PQLBoolean {
    let rs = board_ranks(street, board).to_u16();

    match game {
        PQLGame::Holdem | PQLGame::Omaha => {
            for bits in ARR_STRAIGHT {
                if (bits & rs).count_ones() >= 3 {
                    return true;
                }
            }
        }

        PQLGame::ShortDeck => {
            for bits in ARR_STRAIGHT_SHORT {
                if (bits & rs).count_ones() >= 3 {
                    return true;
                }
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_straight_board(hbg: HandBoardGame) -> TestResult {
        let rs = board_ranks(hbg.street, hbg.board).to_u16();

        let is_straight_board = (match hbg.game {
            PQLGame::ShortDeck => &ARR_STRAIGHT_SHORT as &[_],
            _ => &ARR_STRAIGHT as &[_],
        })
        .iter()
        .any(|bits| (bits & rs).count_ones() >= 3);

        TestResult::from_bool(
            is_straight_board
                == straight_board(hbg.street, (hbg.game, hbg.board)),
        )
    }
}
