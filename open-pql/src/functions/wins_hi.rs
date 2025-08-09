use super::*;

#[pqlfn(arg, rtn, eval)]
pub fn wins_hi(
    pid: PQLPlayer,
    args: (PQLGame, Board, &PlayerHands, &mut BufferRatings),
) -> PQLBoolean {
    best_hi_rating(pid, PQLStreet::River, args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[quickcheck]
    fn test_wins_hi(hbg: HandBoardGame) {
        let HandBoardGame {
            hand: h1,
            another_hand: h2,
            board,
            game,
            ..
        } = hbg;

        let mut buf = BufferRatings::new(2);
        let hands = vec![h1, h2];

        let win_p1 = wins_hi(0.into(), (game, board, &hands, &mut buf));
        let win_p2 = wins_hi(1.into(), (game, board, &hands, &mut buf));

        let tie_p1 = ties_hi(0.into(), (game, board, &hands, &mut buf));
        let tie_p2 = ties_hi(1.into(), (game, board, &hands, &mut buf));

        let scp_p1 = scoops(0.into(), (game, board, &hands, &mut buf));
        let scp_p2 = scoops(1.into(), (game, board, &hands, &mut buf));

        match (win_p1, win_p2) {
            (true, true) => {
                assert!(tie_p1);
                assert!(tie_p2);
                assert!(!scp_p1);
                assert!(!scp_p2);
            }
            (true, false) => {
                assert!(!tie_p1);
                assert!(!tie_p2);
                assert!(scp_p1);
                assert!(!scp_p2);
            }
            (false, true) => {
                assert!(!tie_p1);
                assert!(!tie_p2);
                assert!(!scp_p1);
                assert!(scp_p2);
            }
            _ => unreachable!(""),
        }
    }
}
