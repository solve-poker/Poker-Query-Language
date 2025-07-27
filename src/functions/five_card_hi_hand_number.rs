use super::*;
lazy_static! {
    pub static ref HOLDEM_MAP_DATA: Vec<u8> = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/holdem-map-ranking-number.bin"
    ))
    .to_vec();
    pub static ref SHORTDECK_MAP_DATA: Vec<u8> = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/shortdeck-map-ranking-number.bin"
    ))
    .to_vec();
    pub static ref HOLDEM_MAP: FxHashMap<i16, u16> =
        bincode::deserialize(&HOLDEM_MAP_DATA).unwrap();
    pub static ref SHORTDECK_MAP: FxHashMap<i16, u16> =
        bincode::deserialize(&SHORTDECK_MAP_DATA).unwrap();
}

const NUM_OF_DISTICNT_RANKINGS: PQLInteger = 7462;
const NUM_OF_DISTICNT_RANKINGS_SHORT: PQLInteger = 840;

/// Note:
/// the number is for 5 card hand (the set of possible rankings on river may be smaller than this.)
#[pqlfn(arg, rtn, eval)]
pub fn five_card_hi_hand_number(
    hand: &Hand,
    street: PQLStreet,
    (game, board): (PQLGame, Board),
) -> PQLInteger {
    let ranking = hi_rating(hand, street, (game, board));

    match game {
        PQLGame::Holdem | PQLGame::Omaha => {
            NUM_OF_DISTICNT_RANKINGS
                - PQLInteger::from(HOLDEM_MAP[&ranking.to_i16()])
        }

        PQLGame::ShortDeck => {
            NUM_OF_DISTICNT_RANKINGS_SHORT
                - PQLInteger::from(SHORTDECK_MAP[&ranking.to_i16()])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_five_card_hi_hand_number_holdem() {
        fn flop(hand: &Hand, board: Board) -> PQLInteger {
            five_card_hi_hand_number(
                hand,
                PQLStreet::Flop,
                (PQLGame::Holdem, board),
            )
        }

        assert_eq!(1, flop(&cards!["AsKs"], board!("QsJsTs 5d6c")));
        assert_eq!(10, flop(&cards!["As2s"], board!("3s4s5s 5d6c")));

        assert_eq!(11, flop(&cards!["AsAh"], board!("AdAcKs 5d6c")));
        assert_eq!(166, flop(&cards!["2s2h"], board!("2d2c3s 5d6c")));

        assert_eq!(167, flop(&cards!["AsAh"], board!("AdKcKs 5d6c")));
        assert_eq!(322, flop(&cards!["2s2h"], board!("2d3c3s 5d6c")));

        assert_eq!(323, flop(&cards!["AsKs"], board!("QsJs9s 5d6c")));
        assert_eq!(1599, flop(&cards!["2s3s"], board!("4s5s7s 5d6c")));

        assert_eq!(1600, flop(&cards!["AsKs"], board!("QsJsTh 5d6c")));
        assert_eq!(1609, flop(&cards!["As2s"], board!("3s4s5h 5d6c")));

        assert_eq!(1610, flop(&cards!["AsAh"], board!("AdKcQs 5d6c")));
        assert_eq!(2467, flop(&cards!["2s2h"], board!("2d3c4s 5d6c")));

        assert_eq!(2468, flop(&cards!["AsAh"], board!("KdKcQs 5d6c")));
        assert_eq!(3325, flop(&cards!["2s2h"], board!("3d3c4s 5d6c")));

        assert_eq!(3326, flop(&cards!["AsAh"], board!("KdQcJs 5d6c")));
        assert_eq!(6185, flop(&cards!["2s2h"], board!("3d4c5s 5d6c")));

        assert_eq!(6186, flop(&cards!["AsKh"], board!("QdJc9s 5d6c")));
        assert_eq!(7462, flop(&cards!["2s3h"], board!("4d5c7s 5d6c")));
    }

    #[test]
    fn test_five_card_hi_hand_number_shortdeck() {
        fn flop(hand: &Hand, board: Board) -> PQLInteger {
            five_card_hi_hand_number(
                hand,
                PQLStreet::Flop,
                (PQLGame::ShortDeck, board),
            )
        }

        assert_eq!(1, flop(&cards!["AsKs"], board!("QsJsTs 5d6c")));
        assert_eq!(5, flop(&cards!["Ts9s"], board!("8s7sAs 5d6c")));

        assert_eq!(6, flop(&cards!["AsAh"], board!("AdAcKs 5d6c")));
        assert_eq!(61, flop(&cards!["7s7h"], board!("7d7c8s 5d6c")));

        assert_eq!(62, flop(&cards!["AsKs"], board!("QsJs9s 5d6c")));
        assert_eq!(112, flop(&cards!["QsTs"], board!("9s8s7s 5d6c")));

        assert_eq!(113, flop(&cards!["AsAh"], board!("AdKcKs 5d6c")));
        assert_eq!(168, flop(&cards!["7s7h"], board!("7d8c8s 5d6c")));

        assert_eq!(169, flop(&cards!["AsKs"], board!("QsJsTh 5d6c")));
        assert_eq!(173, flop(&cards!["Ts9s"], board!("8s7sAh 5d6c")));

        assert_eq!(174, flop(&cards!["AsAh"], board!("AdKcQs 5d6c")));
        assert_eq!(341, flop(&cards!["7s7h"], board!("7d8c9s 5d6c")));

        assert_eq!(342, flop(&cards!["AsAh"], board!("KdKcQs 5d6c")));
        assert_eq!(509, flop(&cards!["8s8h"], board!("7d7c9s 5d6c")));

        assert_eq!(510, flop(&cards!["AsAh"], board!("KdQcJs 5d6c")));
        assert_eq!(789, flop(&cards!["7s7h"], board!("8d9cTs 5d6c")));

        assert_eq!(790, flop(&cards!["AsKh"], board!("QdJc9s 5d6c")));
        assert_eq!(840, flop(&cards!["QsTh"], board!("9d8c7s 5d6c")));
    }
}
