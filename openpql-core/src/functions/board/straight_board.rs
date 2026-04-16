use crate::{PQLBoard, PQLCardSet, PQLGame, PQLRankSet};

#[inline]
fn has_straight_draw<const SD: bool>(ranks: PQLRankSet) -> bool {
    PQLRankSet::all_straights::<SD>()
        .iter()
        .any(|&bits| (bits & ranks).count() >= 3)
}

pub fn straight_board(game: PQLGame, board: PQLBoard) -> bool {
    let ranks = PQLRankSet::from(PQLCardSet::from(board));

    if game.is_shortdeck() {
        has_straight_draw::<true>(ranks)
    } else {
        has_straight_draw::<false>(ranks)
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use openpql_prelude::cards;

    use super::*;

    fn assert_straight_board(game: PQLGame, s: &str, expected: bool) {
        let board = PQLBoard::from_slice(&cards!(s));
        assert_eq!(straight_board(game, board), expected, "{s} {expected}");
    }

    #[test]
    fn test_holdem_omaha() {
        for game in [PQLGame::Holdem, PQLGame::Omaha] {
            assert_straight_board(game, "5h 6c 7d", true);
            assert_straight_board(game, "2h 5c 9d", false);
            assert_straight_board(game, "Ah 2c 3d", true);
            assert_straight_board(game, "Qh Kc Ad", true);
            assert_straight_board(game, "5h 6c 7d 8s 9h", true);
        }
    }

    #[test]
    fn test_shortdeck() {
        let game = PQLGame::ShortDeck;
        assert_straight_board(game, "9h Tc Jd", true);
        assert_straight_board(game, "Ah 8d 9c", true);
    }
}
