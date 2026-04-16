use crate::common::{assert_count_all, assert_count_none};

#[test]
fn all_hearts_flop_is_flushing() {
    assert_count_all(
        "select count(flushingboard(flop)) \
         from game='holdem', board='AhKhQh'",
    );
}

#[test]
fn all_hearts_river_is_flushing() {
    assert_count_all(
        "select count(flushingboard(river)) \
         from game='holdem', board='AhKhQh2h3h'",
    );
}

#[test]
fn rainbow_flop_is_not_flushing() {
    assert_count_none(
        "select count(flushingboard(flop)) \
         from game='holdem', board='AhKdQc'",
    );
}

#[test]
fn river_with_no_three_of_a_suit_is_not_flushing() {
    // 2 hearts + 2 spades + 1 diamond: no suit reaches 3 → not flushing.
    assert_count_none(
        "select count(flushingboard(river)) \
         from game='holdem', board='AhKh2s3s4d'",
    );
}
