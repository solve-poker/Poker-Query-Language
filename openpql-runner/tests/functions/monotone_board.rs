use crate::common::{assert_count_all, assert_count_none};

#[test]
fn all_hearts_flop_is_monotone() {
    assert_count_all(
        "select count(monotoneboard(flop)) \
         from game='holdem', board='AhKhQh'",
    );
}

#[test]
fn all_spades_river_is_monotone() {
    assert_count_all(
        "select count(monotoneboard(river)) \
         from game='holdem', board='AsKsQs2s3s'",
    );
}

#[test]
fn rainbow_flop_is_not_monotone() {
    assert_count_none(
        "select count(monotoneboard(flop)) \
         from game='holdem', board='AhKdQc'",
    );
}

#[test]
fn twotone_flop_is_not_monotone() {
    assert_count_none(
        "select count(monotoneboard(flop)) \
         from game='holdem', board='AhKhQd'",
    );
}
