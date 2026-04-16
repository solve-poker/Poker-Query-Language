use crate::{
    PQLBoard, PQLCard, PQLCardCount, PQLGame, PQLHandType,
    functions::outs_to_hand_type,
};

pub fn min_outs_to_hand_type(
    game: PQLGame,
    hand: &[PQLCard],
    board: PQLBoard,
    target: PQLHandType,
    min: PQLCardCount,
) -> bool {
    outs_to_hand_type(game, hand, board, target) >= min
}

#[cfg(test)]
mod tests {
    use openpql_prelude::{board, cards};

    use super::*;

    #[test]
    fn test_min_outs_to_hand_type_cases() {
        let game = PQLGame::Holdem;
        let h = cards!("Js Th");
        let b = board!("9h 8s 2c");

        assert!(min_outs_to_hand_type(game, &h, b, PQLHandType::Straight, 8));
        assert!(min_outs_to_hand_type(game, &h, b, PQLHandType::Straight, 1));
        assert!(!min_outs_to_hand_type(
            game,
            &h,
            b,
            PQLHandType::Straight,
            9
        ));
    }
}
