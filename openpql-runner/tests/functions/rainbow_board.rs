use crate::common::{assert_count_all, assert_count_none};

#[test]
fn three_suits_flop_is_rainbow() {
    assert_count_all(
        "select count(rainbowboard(flop)) \
         from game='holdem', board='AhKdQc'",
    );
}

#[test]
fn four_suits_turn_is_rainbow() {
    assert_count_all(
        "select count(rainbowboard(turn)) \
         from game='holdem', board='AhKdQc2s'",
    );
}

#[test]
fn two_hearts_flop_is_not_rainbow() {
    assert_count_none(
        "select count(rainbowboard(flop)) \
         from game='holdem', board='AhKhQd'",
    );
}

#[test]
fn river_is_never_rainbow() {
    // Only 4 suits exist, so a 5-card board can never be rainbow.
    assert_count_none(
        "select count(rainbowboard(river)) \
         from game='holdem', board='AhKdQc2s3s'",
    );
}
