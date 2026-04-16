use crate::common::{assert_count_all, assert_count_none};

#[test]
fn fully_pinned_river_equals_fifth_card() {
    // Board ranges pin positions 3 (turn) and 4 (river); only the flop
    // cards are permuted. With board='AhKhQhJhTh', river must be Th.
    assert_count_all(
        "select count(rivercard() = tocard('Th')) \
         from game='holdem', board='AhKhQhJhTh'",
    );
}

#[test]
fn river_card_is_not_unrelated_card() {
    assert_count_none(
        "select count(rivercard() = tocard('2c')) \
         from game='holdem', board='AhKhQhJhTh'",
    );
}

#[test]
fn river_card_is_pinned_low_card() {
    assert_count_all(
        "select count(rivercard() = tocard('6c')) \
         from game='holdem', board='2c3d4h5s6c'",
    );
}
