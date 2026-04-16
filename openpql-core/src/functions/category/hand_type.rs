use crate::{PQLBoard, PQLCard, PQLGame, PQLHandType, functions::hi_rating};

pub fn hand_type(
    game: PQLGame,
    hand: &[PQLCard],
    board: PQLBoard,
) -> PQLHandType {
    hi_rating(game, hand, board).into()
}

#[cfg(test)]
mod tests {
    use openpql_prelude::{board, cards};

    use super::*;

    #[test]
    fn test_hand_type_cases() {
        assert_eq!(
            hand_type(PQLGame::Holdem, &cards!("7s 7h"), board!("7c As Ah")),
            PQLHandType::FullHouse,
        );
        assert_eq!(
            hand_type(PQLGame::Holdem, &cards!("Ks Qh"), board!("2c 5d 9h")),
            PQLHandType::HighCard,
        );
    }
}
