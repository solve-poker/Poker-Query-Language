use super::*;

#[pqlfn(arg, rtn, eval)]
pub fn rate_hi_hand(
    text: &str,
    game: PQLGame,
) -> Result<PQLHiRating, RuntimeError> {
    let c64 = to_five_cards(text)?;

    Ok(match game {
        PQLGame::Holdem | PQLGame::Omaha => eval_holdem7(c64),
        PQLGame::ShortDeck => eval_shortdeck7(c64),
    })
}

#[cfg_attr(coverage_nightly, coverage(off))]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[quickcheck]
    fn test_rate_hi_hand(hbg: HandBoardGame) -> TestResult {
        let cards: [_; 5] = hbg.board.into();

        let s = cards.map(|c| c.to_string()).join("");

        let rating = match hbg.game {
            PQLGame::Holdem | PQLGame::Omaha => {
                eval_holdem7((&cards as &[_]).into())
            }
            PQLGame::ShortDeck => eval_shortdeck7((&cards as &[_]).into()),
        };

        TestResult::from_bool(rating == rate_hi_hand(&s, hbg.game).unwrap())
    }

    #[test]
    fn test_rate_hi_hand_error() {
        let g = PQLGame::Holdem;

        assert!(rate_hi_hand(" As Ks Qs Js Ts ", g).is_ok());

        let s = "AsKsQsJsTs";
        for i in 0..s.len() {
            assert!(to_five_cards(&s[0..i]).is_err());
        }

        assert![matches!(
            rate_hi_hand(" A Ks Qs Js Ts ", g),
            Err(RuntimeError::RateHiHandParseFailed(_))
        )];
        assert![matches!(
            rate_hi_hand(" sA Ks Qs Js Ts ", g),
            Err(RuntimeError::RateHiHandParseFailed(_))
        )];

        assert![matches!(
            rate_hi_hand("AsKsQsJsTs9s", g),
            Err(RuntimeError::RateHiHandExpectedFiveCards(_))
        )];

        assert![matches!(
            rate_hi_hand("AsAsAsAsAs", g),
            Err(RuntimeError::RateHiHandExpectedFiveCards(_))
        )];
    }
}
