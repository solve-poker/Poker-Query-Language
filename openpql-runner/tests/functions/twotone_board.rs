use crate::common::{assert_count_all, assert_count_none};

#[test]
fn two_suit_flop_is_twotone() {
    assert_count_all(
        "select count(twotoneboard(flop)) \
         from game='holdem', board='AhKhQd'",
    );
}

#[test]
fn two_suit_river_is_twotone() {
    // 3 hearts + 2 diamonds.
    assert_count_all(
        "select count(twotoneboard(river)) \
         from game='holdem', board='AhKhQh2d3d'",
    );
}

#[test]
fn monotone_flop_is_not_twotone() {
    assert_count_none(
        "select count(twotoneboard(flop)) \
         from game='holdem', board='AhKhQh'",
    );
}

#[test]
fn rainbow_flop_is_not_twotone() {
    assert_count_none(
        "select count(twotoneboard(flop)) \
         from game='holdem', board='AhKdQc'",
    );
}
