use crate::common::{assert_count_all, assert_count_none};

#[test]
fn fully_pinned_turn_equals_fourth_card() {
    // Position 3 (turn) is pinned by the board range.
    assert_count_all(
        "select count(turncard() = tocard('Jh')) \
         from game='holdem', board='AhKhQhJhTh'",
    );
}

#[test]
fn turn_card_is_not_unrelated_card() {
    assert_count_none(
        "select count(turncard() = tocard('2c')) \
         from game='holdem', board='AhKhQhJhTh'",
    );
}

#[test]
fn turn_card_differs_from_river_card_when_both_pinned() {
    assert_count_none(
        "select count(turncard() = rivercard()) \
         from game='holdem', board='AhKhQhJhTh'",
    );
}
