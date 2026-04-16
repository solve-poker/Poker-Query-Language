use crate::common::{assert_count_all, assert_count_none};

#[test]
fn parses_card_equal_to_itself() {
    assert_count_all(
        "select count(tocard('As') = tocard('As')) \
         from game='holdem', board='*'",
    );
}

#[test]
fn different_cards_are_not_equal() {
    assert_count_none(
        "select count(tocard('As') = tocard('Kh')) \
         from game='holdem', board='*'",
    );
}

#[test]
fn same_rank_different_suit_is_not_equal() {
    assert_count_none(
        "select count(tocard('As') = tocard('Ah')) \
         from game='holdem', board='*'",
    );
}

#[test]
fn parses_pinned_river_card() {
    // River is pinned to position 4 of the board range.
    assert_count_all(
        "select count(tocard('7d') = rivercard()) \
         from game='holdem', board='AhKhQh2c7d'",
    );
}
